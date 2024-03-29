//! Library for making pdf documents of spells that a 5th edition D&D character has.
//!
//! See repository for documentation on spell files.
//!
//! Repository at <https://github.com/ChandlerJayCalkins/dnd_spellbook_maker>.

use std::fs;
extern crate image;
pub use printpdf::{Mm, PdfDocumentReference, ImageTransform, ImageRotation};
use printpdf::{PdfDocument, PdfLayerReference, IndirectFontRef, Color, Rgb, Point, Line, PdfPageIndex, Image};
use rusttype::{Font, Scale, point};
pub mod spells;

// Used for conveying what type of font is being used
// Mostly used for font units to mm conversion
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FontType
{
	Regular,
	Bold,
	Italic,
	BoldItalic
}

// Calculates the width of text with a certain font in printpdf millimeters (Mm)
fn calc_text_width(font_scalars: &FontScalars, text: &str, font_type: &FontType, font_size_data: &Font,
font_scale: &Scale) -> f32
{
	let width = font_size_data.layout(text, *font_scale, point(0.0, 0.0))
		.map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
		.last()
		.unwrap_or(0.0);
	font_units_to_mm(font_scalars, width, font_type)
}

// Calculates the height of a number of lines of text given the font, font size, newline size, and number of lines
fn calc_text_height(font_scalars: &FontScalars, font_type: &FontType, font_size_data: &Font, font_scale: &Scale,
newline_amount: f32, lines: usize) -> f32
{
	// If there are no lines, return 0 for the height
	if lines == 0 { return 0.0; }
	// Calculate the amount of space every newline takes up
	let line_height = ((lines - 1) as f32) * newline_amount;
	// Calculate the height of a the lower half and the upper half of a line of text in this font
	let v_metrics = font_size_data.v_metrics(*font_scale);
	let font_height = font_units_to_mm(font_scalars, v_metrics.ascent - v_metrics.descent, font_type);
	// Return the amount of space the newlines take up plus the space a single line takes up
	line_height + font_height
}

// Conterts rusttype font units to printpdf millimeters (Mm)
fn font_units_to_mm(font_scalars: &FontScalars, font_units: f32, font_type: &FontType) -> f32
{
	let scaler: f32 = match font_type
	{
		FontType::Regular => font_scalars.regular_scalar(),
		FontType::Bold => font_scalars.bold_scalar(),
		FontType::Italic => font_scalars.italic_scalar(),
		FontType::BoldItalic => font_scalars.bold_italic_scalar()
	};
	font_units * scaler
}

// Calculates the width of the widest text in each column of a table vec along with that column's index
fn get_max_column_widths(font_scalars: &FontScalars, table_vec: &Vec<Vec<&str>>, columns: usize,
body_font_type: &FontType, header_font_type: &FontType, body_font_size_data: &Font, header_font_size_data: &Font,
font_scale: &Scale) -> Vec<(usize, f32)>
{
	let mut widths = Vec::with_capacity(columns);
	for i in 0..columns
	{
		// Flag to tell when to use header font instead of body font
		let mut header = true;
		let mut max_width: f32 = 0.0;
		for j in 0..table_vec.len()
		{
			// Calculate width with either header or body text depending on header flag
			let width = if header
			{
				header = false;
				calc_text_width(font_scalars, table_vec[j][i], header_font_type, header_font_size_data, font_scale)
			}
			else { calc_text_width(font_scalars, table_vec[j][i], body_font_type, body_font_size_data, font_scale) };
			max_width = max_width.max(width);
		}
		widths.push((i, max_width));
	}
	widths
}

// Writes a table to the pdf doc
fn write_table(doc: &PdfDocumentReference, layer_name_prefix: &str, layer: &PdfLayerReference,
page_number_data: &mut Option<(&PageNumberData, &mut bool, f32, &FontType, &IndirectFontRef, &Font, &Scale)>,
layer_count: &mut i32, background_img_data: &Option<(image::DynamicImage, ImageTransform)>, font_scalars: &FontScalars,
title_lines: &Vec<String>, table: &Vec<Vec<Vec<String>>>, text_color: &Color, font_size: f32, page_width: f32,
page_height: f32, table_x_start: f32, table_x_end: f32, column_starts: &Vec<f32>, column_widths: &Vec<f32>,
centered_columns: &Vec<bool>, y_high: f32, y_low: f32, x: &mut f32, y: &mut f32, body_font_type: &FontType,
header_font_type: &FontType, body_font: &IndirectFontRef, header_font: &IndirectFontRef, body_font_size_data: &Font,
header_font_size_data: &Font, font_scale: &Scale, table_options: &TableOptions, title_font_scale: &Scale,
newline_amount: f32) -> PdfLayerReference
{
	// Create a vec of all layers of pages that are used to write the table to
	let mut layers = vec![(*layer).clone()];
	// Index of the most recent page created
	let mut layers_index = 0;
	// Data for the current font that is being used
	// Starts with header font for first row
	let mut current_font = header_font;
	let mut current_font_size_data = header_font_size_data;
	let mut current_font_type = header_font_type;
	// Keeps track of the last x position
	let mut last_x = *x;
	// Used for telling when to place the off row color lines
	let mut off_row = false;
	// Adjusts the y position by a certain amount between rows
	// Starts as 0 so the first row doesn't get moved down at all from the starting position
	let mut vertical_margin_adjuster = 0.0;
	// If there is a title
	if title_lines.len() > 0
	{
		// Calculate the width of the table as far as the text goes
		let table_width = table_x_end - table_x_start - table_options.outer_horizontal_margin();
		// Create a newline adjuster to move the y position down before every line
		// Starts as 0 so it doesn't move down at all for the first line
		let mut newline_adjuster = 0.0;
		// Loop through each line in the title to apply it
		for line in title_lines
		{
			// Calculate the width of this line
			let line_width = calc_text_width(font_scalars, &line, current_font_type, current_font_size_data,
				title_font_scale);
			// Set the x position to have the line be centered above the table
			*x = (table_width / 2.0) - (line_width / 2.0) + table_x_start;
			// Apply the line to the page
			apply_textbox_line(doc, layer_name_prefix, &mut layers, &mut layers_index, page_number_data, layer_count,
				background_img_data, font_scalars, &line, text_color, table_options.title_font_size(), page_width,
				page_height, y_high, y_low, x, y, current_font, newline_adjuster);
			// Set the newline adjuster to be the newline amount so it isn't 0 after the first line
			newline_adjuster = newline_amount;
		}
		// Move the y position down by a vertical cell margin so there's space between the title and the header row
		*y -= table_options.vertical_cell_margin();
	}
	// Construct the off row color object
	let (r, g, b) = table_options.off_row_color();
	let off_row_color = Color::Rgb(Rgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, None));
	// Keep track of the starting y position so the y position can be reset to it after applying the off row color lines
	let start_y = *y;
	// Keep track of the starting layer index so it can be reset it after applying the off row color lines
	let start_layers_index = layers_index;
	// Amount of space to vertically adjust the off row color lines by from the text line positions
	let off_row_color_line_y_adjustment = font_size * 0.11;
	// Increase the y position a bit so it lines up with the text lines
	*y += off_row_color_line_y_adjustment;
	// Keeps track of the lowest y position of the current row
	let mut row_lowest_y = *y;

	// Loop through the table a first time to apply the off row color lines

	// Loop through each row in the table
	for row in table
	{
		// Set the y value to just below the last row
		*y = row_lowest_y - vertical_margin_adjuster;
		// Set the vertical margin adjuster to the desired value so it doesn't go down by 0 after the first row
		vertical_margin_adjuster = table_options.vertical_cell_margin();
		// Keep track of where the y position starts for this row so it can be reset to it after writing each cell
		let y_row_start = *y;
		// Create an index for the layers vec that keeps track of the page this row starts being written on
		let row_layers_index = layers_index;
		// Keeps track of how many off row color lines have already been applied to this row
		let mut row_off_row_lines: usize = 0;
		// Loop through each cell in this row
		for cell in row
		{
			// Adjusts the y position by a certain amount between lines
			let mut newline_adjuster = 0.0;
			// Reset the y position to the starting position for this row
			*y = y_row_start;
			// Create an index for the layers vec that keeps track of what page this cell is currently writing to
			let mut cell_layers_index = row_layers_index;
			// Keeps track of how many lines this cell has gone through so it can start applying new off row color lines
			// when needed
			let mut cell_off_row_lines: usize = 0;
			// Loop through each line in this cell
			for _ in cell
			{
				// Apply empty text to go to a new line and create a new page if needed
				apply_textbox_line(doc, layer_name_prefix, &mut layers, &mut cell_layers_index, page_number_data,
					layer_count, background_img_data, font_scalars, "", text_color, font_size, page_width, page_height,
					y_high + off_row_color_line_y_adjustment, y_low + off_row_color_line_y_adjustment, x, y, current_font,
					newline_adjuster);
				// If this is an off row
				if off_row
				{
					// Increment the number of lines this cell has gone through
					cell_off_row_lines += 1;
					// If this cell has gone through more lines than there are off row color lines
					if cell_off_row_lines > row_off_row_lines
					{
						// Create a new off row color lines

						// Create the starting and ending points of the line
						let points = vec!
						[
							(Point::new(Mm(table_x_start), Mm(*y)), false),
							(Point::new(Mm(table_x_end), Mm(*y)), false)
						];
						// Create the line object
						let line = Line
						{
							points: points,
							is_closed: false
						};
						// Set the color of the line
						layers[cell_layers_index].set_outline_color(off_row_color.clone());
						// Calculate the height of the current line of text
						let line_height = calc_text_height(font_scalars, current_font_type, current_font_size_data,
							font_scale, newline_amount, 1);
						// Set the thickness of the off row color line
						layers[cell_layers_index]
							.set_outline_thickness(line_height * table_options.off_row_color_lines_height_scalar());
						// Add the line
						layers[cell_layers_index].add_line(line);
					}
				}

				// Set the newline adjuster to the newline amount so it's not 0 after the first line
				newline_adjuster = newline_amount;
			}

			// If this cell is on the most recently created page and is lower than the lowest y value for this row
			if cell_layers_index == layers_index && *y < row_lowest_y
			{
				// Set the lowest y value for this row to the current y value
				row_lowest_y = *y;
			}
			// If this cell is on a new page
			else if cell_layers_index > layers_index
			{
				// Set the lowest y value for this row to the current y value
				row_lowest_y = *y;
				// Set the layers_index for the most recently created page to this cell's layer index
				layers_index = cell_layers_index;
			}
			// Set the number of off row color lines applied for this row to the max of this cell's lines and the current
			// row total
			row_off_row_lines = std::cmp::max(row_off_row_lines, cell_off_row_lines);
		}

		// Start using the body font after the first row
		current_font = body_font;
		current_font_size_data = body_font_size_data;
		current_font_type = body_font_type;
		// Flip the off row flag
		off_row = !off_row;
	}

	// Reset the y position back to the top of the table and the lowest y value for the current row
	*y = start_y;
	row_lowest_y = start_y;
	// Reset the layers vec index back to the first page
	layers_index = start_layers_index;
	// Reset the vertical margin adjuster to 0 so it doesn't go down at all for the first row
	vertical_margin_adjuster = 0.0;
	// Reset the font back to the header font for the first row again
	current_font = header_font;
	current_font_size_data = header_font_size_data;
	current_font_type = header_font_type;

	// Loop through the table a second time to apply the lines of text

	// Loop through each row in the table
	for row in table
	{
		// Set the y value to just below the last row
		*y = row_lowest_y - vertical_margin_adjuster;
		// Set the vertical margin adjuster to the row margin amount so it's not 0 after the first row
		vertical_margin_adjuster = table_options.vertical_cell_margin();
		// Create a variable to keep track of where to reset the y value to after writing each cell in this row
		let y_row_start = *y;
		// Create an index to keep track of what page this row starts on
		// This makes it so each cell in this row gets written to the correct page
		let row_layers_index = layers_index;
		// Variable to keep track of the column data in column_widths and column_starts
		let mut column_index = 0;

		// Loop through each cell in this row
		for cell in row
		{
			// The amount the text goes down on each newline
			// Starts as 0 so the first line in this cell doesn't get moved down from the correct position
			let mut newline_adjuster = 0.0;
			// Reset the y position to the top of the row
			*y = y_row_start;
			// Create an index to give to apply_textbox_line and keep track of the current page being used
			let mut cell_layers_index = row_layers_index;

			// Loop through each line in this cell
			for line in cell
			{
				// Calculate the width of this line
				let line_width =
					calc_text_width(font_scalars, &line, current_font_type, current_font_size_data, font_scale);
				// If this cell is in a centered column
				if centered_columns[column_index]
				{
					// Set the x position to make the line centered
					*x = (column_widths[column_index] / 2.0) - (line_width / 2.0) + column_starts[column_index];
				}
				// If this isn't a centered column, set the x position to the left side of the cell
				else { *x = column_starts[column_index]; }
				// Set the last_x position to be at the end of this line
				last_x = *x + line_width;
				// Write the line of text
				apply_textbox_line(doc, layer_name_prefix, &mut layers, &mut cell_layers_index, page_number_data,
					layer_count, background_img_data, font_scalars, &line, text_color, font_size, page_width, page_height,
					y_high, y_low, x, y, current_font, newline_adjuster);
				// Start going down a line when creating a new line after the first line gets applied
				newline_adjuster = newline_amount;
			}

			// If this cell is on the most recently created page and is lower than the lowest y value for this row
			if cell_layers_index == layers_index && *y < row_lowest_y
			{
				// Set the lowest y value for this row to the current y value
				row_lowest_y = *y;
			}
			// If this cell is on a new page
			else if cell_layers_index > layers_index
			{
				// Set the lowest y value for this row to the current y value
				row_lowest_y = *y;
				// Set the layers_index for the most recently created page to this cell's layer index
				layers_index = cell_layers_index;
			}
			// Increase the column index to the next column
			column_index += 1;
		}

		// Start using the body font after the first row
		current_font = body_font;
		current_font_size_data = body_font_size_data;
		current_font_type = body_font_type;
	}

	// Set the y position to the lowest y position of the last row
	*y = row_lowest_y;
	// Set the x position to the end of the last line
	*x = last_x;
	// Return the most recent layer
	layers[layers.len() - 1].clone()
}

// Creates a table from a string of tokens with table tags and writes it to the pdf doc
fn create_table(doc: &PdfDocumentReference, layer_name_prefix: &str, layer: &PdfLayerReference,
page_number_data: &mut Option<(&PageNumberData, &mut bool, f32, &FontType, &IndirectFontRef, &Font, &Scale)>,
layer_count: &mut i32, background_img_data: &Option<(image::DynamicImage, ImageTransform)>, font_scalars: &FontScalars,
table_tokens: &Vec<&str>, text_color: &Color, font_size: f32, page_width: f32, page_height: f32, x_left: f32, x_right: f32,
y_high: f32, y_low: f32, x: &mut f32, y: &mut f32, body_font_type: &FontType, header_font_type: &FontType,
body_font: &IndirectFontRef, header_font: &IndirectFontRef, body_font_size_data: &Font, header_font_size_data: &Font,
font_scale: &Scale, table_options: &TableOptions, title_font_scale: &Scale, newline_amount: f32) -> PdfLayerReference
{
	// Tags for delimiting rows and columns in the table
	const TITLE_TAG: &str = "<title>";
	const ROW_TAG: &str = "<row>";
	const COLUMN_TAG: &str = "|";
	// The layer that gets returned
	let mut layer_ref = (*layer).clone();
	// If there are no tokens in the table, do nothing
	if table_tokens.len() < 1 { return layer_ref; }
	// Get a vec of all the tokens in the title, if there is a title
	let mut title_tokens: Vec<&str> = Vec::new();
	// Index of the token after the last title token
	let mut after_title_index = 0;
	// If the first token in the table is the title tag, then the table has a title
	if table_tokens[0] == TITLE_TAG
	{
		// Shift the index of where the rest of the table starts over to the earliest possible end of the title
		after_title_index = 2;

		// Loop through each token in the table after the first to build the title vec
		for &token in &table_tokens[1..]
		{
			// If the token is the title tag, the title is over so stop looping
			if token == TITLE_TAG { break; }
			else
			{
				// If the token starts with an escape backslash, remove it
				let add_token = if token.starts_with("\\") { &token[1..] } else { token };
				// Push the token to the title tokens vec
				title_tokens.push(add_token);
			}
			// Increase the index of the start of the rest of the table string if this token wasn't the ending title token
			after_title_index += 1;
		}
	}
	// Combine all tokens after the header back into a string
	let table_string = table_tokens[after_title_index..].join(" ");
	// Split the string up into rows by the row tag
	let rows: Vec<_> = table_string.split(ROW_TAG).collect();
	// If there are no rows, do nothing
	if rows.len() < 1 { return layer_ref; }
	// Keeps track of the number of columns
	let mut column_count = 0;
	// 2D vec that will store the strings in the table
	let mut table_vec: Vec<Vec<&str>> = Vec::new();

	// Loop through each row in the table to create the table
	for row in rows
	{
		// Create a new row in the table vec
		table_vec.push(Vec::new());
		// Split the row into columns by the COLUMN_DELIM
		let columns: Vec<_> = row.split(COLUMN_TAG).collect();
		// Increase the number of columns if this row has more than the last column amount
		column_count = std::cmp::max(column_count, columns.len());
		// Index of the last row
		let end_index = table_vec.len() - 1;
		// Loop through each column
		for cell in columns
		{
			// Add the column to the end of the table vec
			table_vec[end_index].push(cell.trim());
		}
	}

	// Loop through each row in the table to add extra columns
	let mut index = 0;
	while index < table_vec.len()
	{
		// If this row has less columns than needed
		if table_vec[index].len() < column_count
		{
			// Add columns until it has the correct amount
			for _ in 0..column_count - table_vec[index].len()
			{
				table_vec[index].push("");
			}
		}
		index += 1;
	}

	// Get the width of the widest string in each column
	let max_column_widths = get_max_column_widths(font_scalars, &table_vec, column_count, body_font_type,
		header_font_type, body_font_size_data, header_font_size_data, font_scale);
	// Create vec for holding the actual width of each column
	// This will be determined by first assuming all columns need the same amount of space on a page,
	// then if any columns have a max width less than that, remove the space that column doesn't need
	// and split it up among the rest of the columns
	let mut column_widths = vec![0.0; column_count];
	// Vec of whether or not each column should be centered
	// Being centered or not is deteremined whether or not the column is less wide than the default column width
	let mut centered_columns = vec![false; column_count];
	// Sort the max width of each column from least to greatest
	let mut sorted_max_widths = max_column_widths.clone();
	sorted_max_widths.sort_by(|(_, a), (_, b)| a.partial_cmp(&b).expect("Error sorting table widths"));
	// Get the width of the entire table
	let mut table_width = x_right - x_left - (table_options.outer_horizontal_margin() * 2.0);
	// Calculate the default column width
	let mut default_column_width =
		(table_width - table_options.horizontal_cell_margin() * ((column_count as f32) - 1.0)) / column_count as f32;
	// Keeps track of the number of reamining columns to calculate width for
	let mut remaining_columns = column_count as f32 - 1.0;

	// Loop through each max column width in order of least to greatest to calculate the column's actual width
	for (index, max_width) in sorted_max_widths
	{
		// If the column's max width is less than the default column width
		if max_width < default_column_width
		{
			// Set that column's width to it's max width
			column_widths[index] = max_width;
			// Make this column a centered column
			centered_columns[index] = true;
			// Adjust the default column width to take up the space this column didn't use
			default_column_width += (default_column_width - max_width) / remaining_columns;
			// Decrease the number of columns left to calculate the width of
			remaining_columns -= 1.0;
		}
		else
		{
			// Set this column's width to the default width
			column_widths[index] = default_column_width;
		}
	}

	// Create a vec of the starting x position for the text in each column
	let mut column_starts: Vec<f32> = Vec::with_capacity(column_count);
	// Create a variable that keeps track of the starting x position of the next column
	let mut current_x = x_left + table_options.outer_horizontal_margin();

	// Loop through each column width to calculate the starting x positions for each column
	for width in &column_widths
	{
		// Push those coordinates to the vec
		column_starts.push(current_x);
		// Set the start x position to the position of the next column
		current_x += width + table_options.horizontal_cell_margin();
	}

	// Calculate the sum of the widths of each column
	let actual_table_width: f32 =
		column_widths.iter().sum::<f32>() + table_options.horizontal_cell_margin() * ((column_count as f32) - 1.0);
	// Make the table width smaller if the columns aren't going to take up the whole page
	table_width = table_width.min(actual_table_width);
	let table_start = x_left;
	let table_end = table_start + table_width + (table_options.outer_horizontal_margin() * 2.0);
	// Create a new 3D table vec for storing the rows, columns, and each line in a cell
	let mut table: Vec<Vec<Vec<String>>> = Vec::new();
	// Used for storing the height of each row in a table
	let mut row_heights: Vec<f32> = Vec::new();
	// Flag to tell if header text is currently being processed
	let mut header = true;

	// Loop through the table vec to split each cell of text into lines

	// Loops through each row in the table
	for row in table_vec
	{
		// Add a new row to the final table vec
		table.push(Vec::new());
		// Get the index of the row that was just added
		let last_row = table.len() - 1;
		// Create a new height for the current row
		row_heights.push(0.0);
		// Counts which column is currently being processed so the correct width in column_widths can be accessed
		let mut column = 0;

		// Loop through each cell in a row
		for cell in row
		{
			// Add a new cell to that row
			table[last_row].push(Vec::new());
			// Get the index of the cell / col that was just added
			let last_col = table[last_row].len() - 1;
			// Get a vec of all the tokens in this cell
			let tokens: Vec<_> = cell.split_whitespace().collect();
			// If there are not tokens in this cell, continue to next cell
			if tokens.len() < 1 { column += 1; continue; }
			// If the first token starts with an escape backslash, remove it
			let add_first_token = if tokens[0].starts_with('\\') { &tokens[0][1..] } else { tokens[0] };
			// Create a string that will represent an entire line of text in this cell
			let mut line = add_first_token.to_string();
			
			// Loop through each token after the first
			for token in &tokens[1..]
			{
				// If the token starts with an escape backslash, remove it
				let add_token = if token.starts_with('\\') { &token[1..] } else { token };
				// Create a string of a line with the next token added
				let new_line = format!("{} {}", line, add_token);
				// Calculate the width of this new line (with header or body font)
				let width = if header
				{
					calc_text_width(font_scalars, &new_line, header_font_type, header_font_size_data, font_scale)
				}
				else { calc_text_width(font_scalars, &new_line, body_font_type, body_font_size_data, font_scale) };
				// If the line with this token added is too wide for this column
				if width > column_widths[column]
				{
					// Add the current line
					table[last_row][last_col].push(line);
					// Reset the line to the current token
					line = add_token.to_string();
				}
				// If the new line isn't too wide, add the current token to the current line
				else { line = new_line; }
			}

			// Add the remaining text to the table
			table[last_row][last_col].push(line);

			// Calculate the height of this cell
			let cell_height = if header
			{
				calc_text_height(font_scalars, header_font_type, header_font_size_data, font_scale,
					newline_amount, table[last_row][last_col].len())
			}
			else
			{
				calc_text_height(font_scalars, body_font_type, body_font_size_data, font_scale,
					newline_amount, table[last_row][last_col].len())
			};
			// Replace the total height for this row if this cell's height is larger than the previous amount
			row_heights[last_row] = row_heights[last_row].max(cell_height);
			// Go to the next column_width index
			column += 1;
		}

		// Set the header font flag to false after the first row has been completed
		header = false;
	}

	// Create a vec of all the tokens in the title combined into lines
	let mut title_lines: Vec<String> = Vec::new();
	// Calculate the maximum width of the title to be no wider than the text in the table
	let title_max_width = table_width - (table_options.outer_horizontal_margin() * 2.0);
	// If there is a title
	if title_tokens.len() > 0
	{
		// Create a buffer line to combine tokens into until it takes up enough width
		let mut title_line = title_tokens[0].to_string();
		// Loop through each token after the first to combine them into lines
		for token in &title_tokens[1..]
		{
			// Create a new line to test if another token can be added to the current line
			let new_line = format!("{} {}", title_line, token);
			// Calculate the width of this new line
			let width = calc_text_width(font_scalars, &new_line, header_font_type, header_font_size_data, title_font_scale);
			// If the new line is too wide with the new token added
			if width > title_max_width
			{
				// Add the current line to the title lines vec
				title_lines.push(title_line);
				// Reset the title line to the current token
				title_line = token.to_string();
			}
			// If the new lines isn't too wide, set the current line to it
			else { title_line = new_line; }
		}
		// Add any remaining text as a line to the title lines
		title_lines.push(title_line);
	}

	// Calculate the height of the entire table
	let mut table_height =
		row_heights.iter().sum::<f32>() + (((row_heights.len() - 2) as f32) * table_options.vertical_cell_margin());
	// If there is a title
	if title_lines.len() > 0
	{
		// Add the height of the title into the table height calculation
		table_height += calc_text_height(font_scalars, header_font_type, header_font_size_data, title_font_scale,
			newline_amount, title_lines.len()) + table_options.vertical_cell_margin();
	}
	
	// If the table goes off the current page but isn't longer than a whole page
	if *y - table_height < y_low && table_height <= y_high - y_low
	{
		// Create a new page
		(_, layer_ref) = make_new_page(doc, layer_name_prefix, page_number_data, layer_count, page_width, page_height,
			background_img_data, font_scalars);
		// Set the x and y text positions to the top-left of the page
		*x = x_left;
		*y = y_high;
	}
	// Write the table and return the last layer that was used
	write_table(doc, layer_name_prefix, &layer_ref, page_number_data, layer_count, background_img_data, font_scalars,
		&title_lines, &table, text_color, font_size, page_width, page_height, table_start, table_end, &column_starts,
		&column_widths, &centered_columns, y_high, y_low, x, y, body_font_type, header_font_type, body_font, header_font,
		body_font_size_data, header_font_size_data, font_scale, table_options, title_font_scale, newline_amount)
}

// Creates a new page and returns the layer for it
fn make_new_page(doc: &PdfDocumentReference, layer_name_prefix: &str,
page_number_data: &mut Option<(&PageNumberData, &mut bool, f32, &FontType, &IndirectFontRef, &Font, &Scale)>,
layer_count: &mut i32, width: f32, height: f32, background_img_data: &Option<(image::DynamicImage, ImageTransform)>,
font_scalars: &FontScalars) -> (PdfPageIndex, PdfLayerReference)
{
	// Create a new page
	let (page, layer) = doc.add_page(Mm(width), Mm(height), format!("{} {}", layer_name_prefix, layer_count));
	// Get the new layer
	let layer_ref = doc.get_page(page).get_layer(layer);
	// If there is a background image
	if let Some((img, transform)) = background_img_data
	{
		// Add it to the page
		let image = Image::from_dynamic_image(&img.clone());
		image.add_to_layer(layer_ref.clone(), *transform);
	}
	// Determine whether or not there should be page numbers
	match page_number_data
	{
		// If there should be page numbers
		Some((data, left, font_size, font_type, font, font_size_data, font_scale)) =>
		{
			// Convert the current page number to a string
			let text = (*layer_count).to_string();
			// Determine the x position of the page number based on if it will be on the left or right side of the page
			let x = match left
			{
				// Left side of the page
				true => data.side_margin(),
				// Right side of the page
				false =>
				{
					// Calculate the width of the page number text
					let text_width = calc_text_width(font_scalars, &text, font_type, font_size_data, font_scale);
					// Set x value to be based on the width of the text
					width - data.side_margin() - text_width
				}
			};
			// Write the page number to the new page
			layer_ref.use_text(&text, *font_size, Mm(x), Mm(data.bottom_margin()), font);
			// If the page number is supposed to flip
			if data.flip_sides()
			{
				// Flip to the other side for the next page
				**left = !(**left);
			}
		},
		// If there should not be page numbers, do nothing
		None => ()
	}
	// Increment the layer / page count
	*layer_count += 1;
	(page, layer_ref)
}

// Writes a line of text into a textbox
fn apply_textbox_line(doc: &PdfDocumentReference, layer_name_prefix: &str, layers: &mut Vec<PdfLayerReference>,
layers_index: &mut usize,
page_number_data: &mut Option<(&PageNumberData, &mut bool, f32, &FontType, &IndirectFontRef, &Font, &Scale)>,
layer_count: &mut i32, background_img_data: &Option<(image::DynamicImage, ImageTransform)>, font_scalars: &FontScalars,
text: &str, color: &Color, font_size: f32, page_width: f32, page_height: f32, y_high: f32, y_low: f32, x: &mut f32,
y: &mut f32, font: &IndirectFontRef, newline_amount: f32)
{
	// The layer that will get returned
	let mut layer = layers[*layers_index].clone();
	// Move the text down a level for the new line
	*y -= newline_amount;
	// if the y level is below the bottom of the text box
	if *y < y_low
	{
		// Do stuff to move the text to the next page / a new page

		// Increase the layers index to the next layer in the vec
		*layers_index += 1;
		// If there are still layers in the layers vec
		if *layers_index < layers.len()
		{
			// Set the current layer to the next layer in the layers vec
			layer = layers[*layers_index].clone();
		}
		// If there are no more layers in the layers vec
		else
		{
			// Create a new page
			(_, layer) = make_new_page(doc, layer_name_prefix, page_number_data, layer_count, page_width, page_height,
				background_img_data, font_scalars);
			// Add the layer for that new page to the layers vec
			layers.push(layer.clone());
		}
		// Set the y level to the top of this page
		*y = y_high;
	}
	// Create a new text section on the page
	layer.begin_text_section();
	// Set the text cursor on the page
	layer.set_text_cursor(Mm(*x), Mm(*y));
	// Set the font and font size
	layer.set_font(font, font_size);
	// Set the text color
	layer.set_fill_color(color.clone());
	// Write the text to the page
	layer.write_text(text, &font);
	// End the text section on the page
	layer.end_text_section();
}

// Writes top-left-aligned text into a fixed size text box
// Returns the last layer of the last page that the text appeared on
// Otherwise it returns the layer of the current page
fn write_textbox(doc: &PdfDocumentReference, layer_name_prefix: &str, layer: &PdfLayerReference,
page_number_data: &mut Option<(&PageNumberData, &mut bool, f32, &FontType, &IndirectFontRef, &Font, &Scale)>,
layer_count: &mut i32, background_img_data: &Option<(image::DynamicImage, ImageTransform)>, font_scalars: &FontScalars,
text: &str, color: &Color, font_size: f32, page_width: f32, page_height: f32, x_left: f32, x_right: f32, y_high: f32,
y_low: f32, x: &mut f32, y: &mut f32, font_type: &FontType, font: &IndirectFontRef, font_size_data: &Font,
font_scale: &Scale, tab_amount: f32, newline_amount: f32) -> PdfLayerReference
{
	// Create a vec with just the current layer in it
	let mut layers = vec![(*layer).clone()];
	// Create a layers index parameter and set it to 0
	let mut layers_index = 0;
	// Write the text to the document
	write_textbox_get_all_pages(doc, layer_name_prefix, &mut layers, &mut layers_index, page_number_data, layer_count,
		background_img_data, font_scalars, text, color, font_size, page_width, page_height, x_left, x_right, y_high, y_low,
		x, y, font_type, font, font_size_data, font_scale, tab_amount, newline_amount);
	// Return the most recent layer this text appeared on
	layers[layers.len() - 1].clone()
}

// Does the same thing as write_textbox(), except it returns layers of all pages created while writing this textbox
// Layer of current page is also returned in vec as first element
fn write_textbox_get_all_pages(doc: &PdfDocumentReference, layer_name_prefix: &str, layers: &mut Vec<PdfLayerReference>,
layers_index: &mut usize,
page_number_data: &mut Option<(&PageNumberData, &mut bool, f32, &FontType, &IndirectFontRef, &Font, &Scale)>,
layer_count: &mut i32, background_img_data: &Option<(image::DynamicImage, ImageTransform)>, font_scalars: &FontScalars,
text: &str, color: &Color, font_size: f32, page_width: f32, page_height: f32, x_left: f32, x_right: f32, y_high: f32,
y_low: f32, x: &mut f32, y: &mut f32, font_type: &FontType, font: &IndirectFontRef, font_size_data: &Font,
font_scale: &Scale, tab_amount: f32, newline_amount: f32)
{
	// If either dimensions of the text box overlap each other, do nothing
	if x_left >= x_right || y_high <= y_low { return; }
	// If the x position starts past the right side of the text box
	// Reset it to the left side plus the tab amount and go to a newline
	if *x > x_right { *x = x_left + tab_amount; *y -= newline_amount; }
	// Keeps track of the ending position of the last line
	let mut last_x = *x;
	// Adjusts the x position to be tabbed over on new paragraphs
	// Will be 0 for the first paragraph so the user of the function has control of where the text starts
	let mut tab_adjuster = 0.0;
	// Adjusts the y position to a new line before applying a line
	// Will be 0 for the first line so the first line prints exactly where the y position is
	let mut newline_adjuster = 0.0;
	// Split the text up into paragraphs by newlines
	let paragraphs = text.split('\n');
	// Loop through each paragraph
	for paragraph in paragraphs
	{
		// Move the x position to the left side of the box plus the tab amount since it's a new paragraph
		*x = *x + tab_adjuster;
		// Get a vec of each token in the paragraph
		let tokens: Vec<_> = paragraph.split_whitespace().collect();
		// If there are no tokens in this paragraph, skip it
		if tokens.len() < 1 { continue; }
		// Set the current line to the first token in the paragraph
		let mut line = tokens[0].to_string();
		// Calculate the ending position of this first token
		let line_end = *x + calc_text_width(font_scalars, &line, font_type, font_size_data, font_scale);
		// If this token would go outside of the textbox
		if line_end > x_right
		{
			// Reset to a new line
			*x = x_left + tab_adjuster;
			*y -= newline_amount;
		}
		// Loop through each token after the first
		for token in &tokens[1..]
		{
			// Create a hypothetical new line with the next token
			let new_line = format!("{} {}", line, token);
			// Calculate the width of this new line
			let new_line_end = *x + calc_text_width(font_scalars, &new_line, font_type, font_size_data, font_scale);
			// If the line would be too wide with the next token
			if new_line_end > x_right
			{
				// Write the current line to the page
				apply_textbox_line(doc, layer_name_prefix, layers, layers_index, page_number_data, layer_count,
					background_img_data, font_scalars, &line, color, font_size, page_width, page_height, y_high, y_low, x,
					y, font, newline_adjuster);
				// Set the newline adjuster to the newline amount so it's not 0 after the first line
				newline_adjuster = newline_amount;
				// Set x position to the left side of the text box to undo tabbing on the first line of new paragraphs
				*x = x_left;
				// Set the current line to the next token
				line = token.to_string();
			}
			// If the new line fits within the text box, add the next token to the current line
			else { line = new_line; }
		}
		// Write all remaining text to the page
		apply_textbox_line(doc, layer_name_prefix, layers, layers_index, page_number_data, layer_count,
			background_img_data, font_scalars, &line, color, font_size, page_width, page_height, y_high, y_low, x, y,
			font, newline_adjuster);
		// Sets the tab adjuster to not be 0 anymore after the first paragraph
		tab_adjuster = tab_amount;
		// Set the newline adjuster to the newline amount so it's not 0 after the first line
		newline_adjuster = newline_amount;
		// Calculate where the end of the last line that was written is and save it
		last_x = *x + calc_text_width(font_scalars, &line, font_type, font_size_data, font_scale);
		// Set x position to the left side of the text box to undo tabbing on the first line of new paragraphs
		*x = x_left;
	}
	// Set the x position to the end of the last line that was written
	*x = last_x;
}

// Writes vertically and horizontally centered text into a fixed size text box
// Returns the last layer of the last page that the text appeared on
// Otherwise it returns the layer of the current page
fn write_centered_textbox(doc: &PdfDocumentReference, layer_name_prefix: &str, layer: &PdfLayerReference,
page_number_data: &mut Option<(&PageNumberData, &mut bool, f32, &FontType, &IndirectFontRef, &Font, &Scale)>,
layer_count: &mut i32, background_img_data: &Option<(image::DynamicImage, ImageTransform)>, font_scalars: &FontScalars,
text: &str, color: &Color, font_size: f32, page_width: f32, page_height: f32, x_left: f32, x_right: f32, y_high: f32,
y_low: f32, x: &mut f32, y: &mut f32, font_type: &FontType, font: &IndirectFontRef, font_size_data: &Font,
font_scale: &Scale, newline_amount: f32) -> PdfLayerReference
{
	// Create a vec with just the current layer in it
	let mut layers = vec![(*layer).clone()];
	// Create a layers index parameter and set it to 0
	let mut layers_index = 0;
	// Write the text to the document
	write_centered_textbox_get_all_pages(doc, layer_name_prefix, &mut layers, &mut layers_index, page_number_data,
		layer_count, background_img_data, font_scalars, text, color, font_size, page_width, page_height, x_left, x_right,
		y_high, y_low, x, y, font_type, font, font_size_data, font_scale, newline_amount);
	// Return the most recent layer this text appeared on
	layers[layers.len() - 1].clone()
}

// Writes vertically and horizontally centered text into a fixed size text box
// Returns the last layer of the last page that the text appeared on
// Otherwise it returns the layer of the current page
fn write_centered_textbox_get_all_pages(doc: &PdfDocumentReference, layer_name_prefix: &str,
layers: &mut Vec<PdfLayerReference>, layers_index: &mut usize,
page_number_data: &mut Option<(&PageNumberData, &mut bool, f32, &FontType, &IndirectFontRef, &Font, &Scale)>,
layer_count: &mut i32, background_img_data: &Option<(image::DynamicImage, ImageTransform)>, font_scalars: &FontScalars,
text: &str, color: &Color, font_size: f32, page_width: f32, page_height: f32, x_left: f32, x_right: f32, y_high: f32,
y_low: f32, x: &mut f32, y: &mut f32, font_type: &FontType, font: &IndirectFontRef, font_size_data: &Font,
font_scale: &Scale, newline_amount: f32)
{
	// If either dimensions of the text box overlap each other, do nothing
	if x_left >= x_right || y_high <= y_low { return; }
	// If the x position starts past the right side of the text box
	// Reset it to the left side and go to a newline
	if *x > x_right { *x = x_left; *y -= newline_amount; }
	// Calculate the width and height of the text box
	let textbox_width = x_right - x_left;
	let textbox_height = y_high - y_low;
	// Adjusts the y position to a new line before applying a line
	// Will be 0 for the first line so the first line prints exactly where the y position is
	let mut newline_adjuster = 0.0;
	// Split the text up into tokens
	let tokens: Vec<_> = text.split_whitespace().collect();
	// If there are no tokens, do nothing
	if tokens.len() < 1 { return; }
	// Create a vector of lines that will be written into the textbox
	let mut lines: Vec<String> = Vec::new();
	// Create a string that will be used to combine text together until it's too long to be on a line
	let mut line = String::from(tokens[0]);
	// Loop through each token after the first
	for token in &tokens[1..]
	{
		// Create a new line that will be used to measure if the current line is long enough to fill the textbox space
		let new_line = format!("{} {}", line, token);
		// Calculate the width of this new line
		let width = calc_text_width(font_scalars, &new_line, font_type, font_size_data, font_scale);
		// If the width of the new line is wider than the text box
		if width > textbox_width
		{
			// Add the current line to the lines vec
			lines.push(line);
			// Reset the current line to the current token
			line = token.to_string();
		}
		// Else, add the current token to the current line
		else { line = new_line; }
	}
	// Add any remaining text to the lines vec
	lines.push(line);
	// Calculate the maximum number of lines per textbox
	let max_lines = (textbox_height / newline_amount).floor() as usize;
	// If there are more lines than can fit on one page, just set the y value to the top of the textbox
	if lines.len() > max_lines { *y = y_high; }
	// If all of the lines can fit on one page, set the y value to the top of where the text will start
	else { *y = (y_high / 2.0) + (lines.len() - 1) as f32 / 2.0 * newline_amount; }
	// Loop through each line
	for l in lines
	{
		// Calculate the width of this line
		let width = calc_text_width(font_scalars, &l, font_type, font_size_data, font_scale);
		// Place the x position in the correct place to have this line be horizontally centered
		*x = (textbox_width / 2.0) - (width / 2.0) + x_left;
		// Apply each line of text to the page
		apply_textbox_line(doc, layer_name_prefix, layers, layers_index, page_number_data, layer_count,
			background_img_data, font_scalars, &l, color, font_size, page_width, page_height, y_high, y_low, x, y, font,
			newline_adjuster);
		// Set the x position to the end of the line
		*x += width;
		// Set the newline adjuster so every line after the first actually gets moved down
		newline_adjuster = newline_amount;
	}
}

// Handles bullet point textbox sizing
fn bullet_point_check(font_scalars: &FontScalars, bullet_x_left: &mut bool, bullet_str: &str, font_type: &FontType,
font_size_data: &Font, font_scale: &Scale, x_left_adjustable: &mut f32, x_left: f32)
{
	match *bullet_x_left
	{
		// If the x_left_adjustable hasn't been calculated for this bullet point yet
		false =>
		{
			// Calculate it
			let width = calc_text_width(font_scalars, bullet_str, font_type, font_size_data, font_scale);
			*x_left_adjustable = x_left + width;
			// Mark x_left_adjustable as calculated
			*bullet_x_left = true;
		},
		_ => ()
	}
}

// Does stuff that's required when changing fonts
fn font_change_wrapup(font_scalars: &FontScalars, text: &mut String, x: &mut f32, y: &mut f32, x_left: f32,
font_type: &FontType, font_size_data: &Font, font_scale: &Scale, tab_amount: f32, newline_amount: f32)
{
	// If the buffer just finished a paragraph
	if (*text).ends_with("\n")
	{
		// Set the x and y positions to a new paragraph position
		*y -= newline_amount;
		*x = x_left + tab_amount;
	}
	else
	{
		// Move the x position over by a space
		let space_width = calc_text_width(font_scalars, " ", font_type, font_size_data, font_scale);
		*x += space_width;
	}
	// Clear the buffer
	*text = String::new();
}

// Writes a spell description to a spellbook, including processing changing fonts and adding tables
// Returns the layer of the page that the description text last appears on
fn write_spell_description(doc: &PdfDocumentReference, layer_name_prefix: &str, layer: &PdfLayerReference,
page_number_data: &mut Option<(&PageNumberData, &mut bool, f32, &FontType, &IndirectFontRef, &Font, &Scale)>,
layer_count: &mut i32, background_img_data: &Option<(image::DynamicImage, ImageTransform)>, font_scalars: &FontScalars,
text: &str, color: &Color, font_size: f32, page_width: f32, page_height: f32, x_left: f32, x_right: f32, y_high: f32,
y_low: f32, x: &mut f32, y: &mut f32, regular_font: &IndirectFontRef, bold_font: &IndirectFontRef,
italic_font: &IndirectFontRef, bold_italic_font: &IndirectFontRef, regular_font_size_data: &Font,
bold_font_size_data: &Font, italic_font_size_data: &Font, bold_italic_font_size_data: &Font, font_scale: &Scale,
table_options: &TableOptions, table_title_font_scale: &Scale, tab_amount: f32, newline_amount: f32) -> PdfLayerReference
{
	// The layer that gets returned
	let mut new_layer = (*layer).clone();
	// A buffer of text that gets written to the spellbook when the font changes, a table is inserted, or the text ends
	let mut buffer = String::new();
	let mut table_tokens: Vec<&str> = Vec::new();
	// Font types
	let regular_font_type = FontType::Regular;
	let bold_font_type = FontType::Bold;
	let italic_font_type = FontType::Italic;
	let bold_italic_font_type = FontType::BoldItalic;
	// Keeps track of the font that is currently being used
	let mut current_font = regular_font;
	let mut current_font_size_data = regular_font_size_data;
	let mut current_font_type = regular_font_type;
	// Tags for switching fonts
	const REGULAR_FONT_TAG: &str = "<r>";
	const BOLD_FONT_TAG: &str = "<b>";
	const ITALIC_FONT_TAG: &str = "<i>";
	const BOLD_ITALIC_FONT_TAG: &str = "<bi>";
	const ITALIC_BOLD_FONT_TAG: &str = "<ib>";
	// Str for calculating the left edge of bullet point text boxes
	const BULLET_STR: &str = "• ";
	// Tag for starting and ending tables
	const TABLE_TAG: &str = "<table>";
	// Keeps track of whether or not the text is currently inside of a bullet point of text
	let mut bullet_point = false;
	// The left side of the textbox that gets used for writing textboxes
	let mut x_left_adjustable = x_left;
	// Keeps track of whether or not a table is currently being processed
	let mut in_table = false;
	// Split the text into paragraphs by newlines
	let paragraphs = text.split('\n');
	// Loop through each paragraph
	for paragraph in paragraphs
	{
		// Split the paragraph up into tokens
		let mut tokens: Vec<_> = paragraph.split_whitespace().collect();
		// Whether or not a new x_left_adjustable has been calculated for the current bullet point or not
		let mut bullet_x_left = false;
		// Number of newline amounts to go down by when clearing buffer
		let mut newlines = 1.0;
		// If there is at least one token
		if tokens.len() > 0
		{
			// If the first token is a bullet
			if tokens[0] == "•"
			{
				// If the last paragraph was not a bullet point
				if !bullet_point
				{
					// Make it so the y position will be shifted down 2 newlines instead of just 1
					newlines = 2.0;
					// Begin bullet point processing for this paragraph
					bullet_point = true;
				}
			}
			// If the first token is a dash
			else if tokens[0] == "-"
			{
				// If the last paragraph was not a bullet point
				if !bullet_point
				{
					// Make it so the y position will be shifted down 2 newlines instead of just 1
					newlines = 2.0;
					// Begin bullet point processing for this paragraph
					bullet_point = true;
				}
				// Set the first token to a bullet
				tokens[0] = "•";
			}
			// If this is not a bullet point paragraph but the last one was
			else if bullet_point
			{
				// Move down 2 newline amounts
				*y -= newline_amount * 2.0;
				// Set this paragraph to not be a bullet point
				bullet_point = false;
			}
		}
		// If there are no tokens in this paragraph, move onto the next one
		else { continue; }
		// If this is a bullet point paragraph
		if bullet_point
		{
			// Write any remaining text to the spellbook
			new_layer = write_textbox(doc, layer_name_prefix, &new_layer, page_number_data, layer_count,
				background_img_data, font_scalars, &buffer, color, font_size, page_width, page_height, x_left_adjustable,
				x_right, y_high, y_low, x, y, &current_font_type, current_font, current_font_size_data, font_scale,
				tab_amount, newline_amount);
			// Move the y position down by newlines amount either 1 or 2 times
			*y -= newline_amount * newlines;
			// Move the x position to the left side of the textbox
			*x = x_left;
			// Reset the buffer
			buffer = String::new();
		}
		// Loop through each token
		for token in tokens
		{
			// Do different things depending on what the token is
			match token
			{
				// If the token is a font change tag
				// Regular font
				REGULAR_FONT_TAG =>
				{
					// If a table is currently being processed, push this token to the table_tokens vec
					if in_table { table_tokens.push(token); }
					// If the current font is not already set to this font and 
					else if current_font != regular_font
					{
						// If a bullet point is currently being processed
						if bullet_point
						{
							// Calculate x_left_adjustable if needed
							bullet_point_check(font_scalars, &mut bullet_x_left, BULLET_STR, &current_font_type,
								current_font_size_data, font_scale, &mut x_left_adjustable, x_left);
						}
						// Write the buffer of text to the spellbook with the last font
						new_layer = write_textbox(doc, layer_name_prefix, &new_layer, page_number_data, layer_count,
							background_img_data, font_scalars, &buffer, color, font_size, page_width, page_height,
							x_left_adjustable, x_right, y_high, y_low, x, y, &current_font_type, current_font,
							current_font_size_data, font_scale, tab_amount, newline_amount);
						// Do some other things to prepare for writing more text
						font_change_wrapup(font_scalars, &mut buffer, x, y, x_left_adjustable, &current_font_type,
							current_font_size_data, font_scale, tab_amount, newline_amount);
						// Change the font that is currently being used
						current_font = regular_font;
						current_font_size_data = regular_font_size_data;
						current_font_type = regular_font_type;
					}
				},
				// Bold font
				BOLD_FONT_TAG =>
				{
					if in_table { table_tokens.push(token); }
					else if current_font != bold_font
					{
						if bullet_point
						{
							bullet_point_check(font_scalars, &mut bullet_x_left, BULLET_STR, &current_font_type,
								current_font_size_data, font_scale, &mut x_left_adjustable, x_left);
						}
						new_layer = write_textbox(doc, layer_name_prefix, &new_layer, page_number_data, layer_count,
							background_img_data, font_scalars, &buffer, color, font_size, page_width, page_height,
							x_left_adjustable, x_right, y_high, y_low, x, y, &current_font_type, current_font,
							current_font_size_data, font_scale, tab_amount, newline_amount);
						font_change_wrapup(font_scalars, &mut buffer, x, y, x_left_adjustable, &current_font_type,
							current_font_size_data, font_scale, tab_amount, newline_amount);
						current_font = bold_font;
						current_font_size_data = bold_font_size_data;
						current_font_type = bold_font_type;
					}
				},
				// Italic font
				ITALIC_FONT_TAG =>
				{
					if in_table { table_tokens.push(token); }
					else if current_font != italic_font
					{
						if bullet_point
						{
							bullet_point_check(font_scalars, &mut bullet_x_left, BULLET_STR, &current_font_type,
								current_font_size_data, font_scale, &mut x_left_adjustable, x_left);
						}
						new_layer = write_textbox(doc, layer_name_prefix, &new_layer, page_number_data, layer_count,
							background_img_data, font_scalars, &buffer, color, font_size, page_width, page_height,
							x_left_adjustable, x_right, y_high, y_low, x, y, &current_font_type, current_font,
							current_font_size_data, font_scale, tab_amount, newline_amount);
						font_change_wrapup(font_scalars, &mut buffer, x, y, x_left_adjustable, &current_font_type,
							current_font_size_data, font_scale, tab_amount, newline_amount);
						current_font = italic_font;
						current_font_size_data = italic_font_size_data;
						current_font_type = italic_font_type;
					}
				},
				// Bold-Italic font
				BOLD_ITALIC_FONT_TAG | ITALIC_BOLD_FONT_TAG =>
				{
					if in_table { table_tokens.push(token); }
					else if current_font != bold_italic_font
					{
						if bullet_point
						{
							bullet_point_check(font_scalars, &mut bullet_x_left, BULLET_STR, &current_font_type,
								current_font_size_data, font_scale, &mut x_left_adjustable, x_left);
						}
						new_layer = write_textbox(doc, layer_name_prefix, &new_layer, page_number_data, layer_count,
							background_img_data, font_scalars, &buffer, color, font_size, page_width, page_height,
							x_left_adjustable, x_right, y_high, y_low, x, y, &current_font_type, current_font,
							current_font_size_data, font_scale, tab_amount, newline_amount);
						font_change_wrapup(font_scalars, &mut buffer, x, y, x_left_adjustable, &current_font_type,
							current_font_size_data, font_scale, tab_amount, newline_amount);
						current_font = bold_italic_font;
						current_font_size_data = bold_italic_font_size_data;
						current_font_type = bold_italic_font_type;
					}
				},
				// If the token is a table tag
				TABLE_TAG =>
				{
					// If a table is currently being processed
					if in_table
					{
						// If this is inside of a bullet point
						if bullet_point
						{
							// Calculate x_left_adjustable if needed
							bullet_point_check(font_scalars, &mut bullet_x_left, BULLET_STR, &current_font_type,
								current_font_size_data, font_scale, &mut x_left_adjustable, x_left);
						}
						// End table processing
						in_table = false;
						// Move y position down away from text to the table
						*y -= table_options.outer_vertical_margin();
						// Create the table and write it to the document
						new_layer = create_table(doc, layer_name_prefix, &new_layer, page_number_data, layer_count,
							background_img_data, font_scalars, &table_tokens, color, font_size, page_width, page_height,
							x_left_adjustable, x_right, y_high, y_low, x, y, &regular_font_type, &bold_font_type,
							regular_font, bold_font, regular_font_size_data, bold_font_size_data, font_scale, table_options,
							table_title_font_scale, newline_amount);
						// Move the y position down away from the table
						*y -= table_options.outer_vertical_margin();
						// Reset the x position to the left side of the textbox
						*x = x_left + tab_amount;
						// Reset the buffer
						table_tokens = Vec::new();
						// Reset the font to regular font
						current_font = regular_font;
						current_font_size_data = regular_font_size_data;
						current_font_type = regular_font_type;
					}
					else
					{
						// If this is inside of a bullet point
						if bullet_point
						{
							// Calculate x_left_adjustable if needed
							bullet_point_check(font_scalars, &mut bullet_x_left, BULLET_STR, &current_font_type,
								current_font_size_data, font_scale, &mut x_left_adjustable, x_left);
						}
						// Begin table processing
						in_table = true;
						// Write out the buffer to the document
						new_layer = write_textbox(doc, layer_name_prefix, &new_layer, page_number_data, layer_count,
							background_img_data, font_scalars, &buffer, color, font_size, page_width, page_height,
							x_left_adjustable, x_right, y_high, y_low, x, y, &current_font_type, current_font,
							current_font_size_data, font_scale, tab_amount, newline_amount);
						// Reset the buffer
						buffer = String::new();
					}
				},
				// If the token is anything else
				_ =>
				{
					// If a table is currently being processed
					if in_table
					{
						// Add the token to the table_tokens so it can be added to the table
						table_tokens.push(token);
					}
					else
					{
						// If the token starts with an escape backslash, remove it
						let add_token = if token.starts_with('\\') { &token[1..] } else { token };
						// Add the token to the buffer
						// If the buffer's empty, just set the buffer to the token
						if buffer.is_empty() { buffer = add_token.to_string(); }
						// If the buffer isn't empty, add a space and then the token to the buffer
						else { buffer = format!("{} {}", buffer, add_token); }
					}
				}
			}
		}
		// If this is inside of a bullet point
		if bullet_point
		{
			// Calculate x_left_adjustable if needed
			bullet_point_check(font_scalars, &mut bullet_x_left, BULLET_STR, &current_font_type,
				current_font_size_data, font_scale, &mut x_left_adjustable, x_left);
			// Write the bullet point text
			new_layer = write_textbox(doc, layer_name_prefix, &new_layer, page_number_data, layer_count,
				background_img_data, font_scalars, &buffer, color, font_size, page_width, page_height, x_left_adjustable,
				x_right, y_high, y_low, x, y, &current_font_type, current_font, current_font_size_data, font_scale,
				tab_amount, newline_amount);
			
			// Move the x position to the starting tabbed in position
			*x = x_left + tab_amount;
			// Reset the buffer
			buffer = String::new();
			// Reset the x_left_adjustable to be the normal x_left position
			x_left_adjustable = x_left;
		}
		// Add a newline to the buffer so write_textbox() knows where the end of the paragraph is
		buffer += "\n";
	}
	// Write any remaining text to the spellbook
	new_layer = write_textbox(doc, layer_name_prefix, &new_layer, page_number_data, layer_count, background_img_data,
		font_scalars, &buffer, color, font_size, page_width, page_height, x_left, x_right, y_high, y_low, x, y,
		&current_font_type, current_font, current_font_size_data, font_scale, tab_amount, newline_amount);
	// Return the last layer that was created for this text
	new_layer
}

// Writes ones of the fields of a spell (casting time, components, etc.) to a spellbook document
// Returns the last layer of the last page that the text appeared on
fn write_spell_field(doc: &PdfDocumentReference, layer_name_prefix: &str, layer: &PdfLayerReference,
page_number_data: &mut Option<(&PageNumberData, &mut bool, f32, &FontType, &IndirectFontRef, &Font, &Scale)>,
layer_count: &mut i32, background_img_data: &Option<(image::DynamicImage, ImageTransform)>, font_scalars: &FontScalars,
field_name: &str, field_text: &str, field_name_color: &Color, field_text_color: &Color, font_size: f32, page_width: f32,
page_height: f32, x_left: f32, x_right: f32, y_high: f32, y_low: f32, x: &mut f32, y: &mut f32,
field_name_font_type: &FontType, field_name_font: &IndirectFontRef, field_name_font_size_data: &Font,
regular_font: &IndirectFontRef, bold_font: &IndirectFontRef, italic_font: &IndirectFontRef,
bold_italic_font: &IndirectFontRef, regular_font_size_data: &Font, bold_font_size_data: &Font,
italic_font_size_data: &Font, bold_italic_font_size_data: &Font, font_scale: &Scale, table_options: &TableOptions,
table_title_font_scale: &Scale, tab_amount: f32, newline_amount: f32) -> PdfLayerReference
{
	// Write the field name ("Casting Time:", "Components:", etc.) to the document
	let mut new_layer = write_textbox(doc, layer_name_prefix, layer, page_number_data, layer_count, background_img_data,
		font_scalars, field_name, field_name_color, font_size, page_width, page_height, x_left, x_right, y_high, y_low, x,
		y, field_name_font_type, field_name_font, field_name_font_size_data, font_scale, tab_amount, newline_amount);
	// Shift the x position over by 1 space
	let sideshift = calc_text_width(font_scalars, " ", field_name_font_type, field_name_font_size_data, font_scale);
	*x += sideshift;
	// Write the text for that field to the document
	new_layer = write_spell_description(doc, layer_name_prefix, &new_layer, page_number_data, layer_count,
		background_img_data, font_scalars, field_text, field_text_color, font_size, page_width, page_height, x_left,
		x_right, y_high, y_low, x, y, regular_font, bold_font, italic_font, bold_italic_font, regular_font_size_data,
		bold_font_size_data, italic_font_size_data, bold_italic_font_size_data, font_scale, table_options,
		table_title_font_scale, tab_amount, newline_amount);
	// Return the last layer that was created for this text
	new_layer
}

/// File paths to all the font files needed for `generate_spellbook()`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontPaths
{
	pub regular: String,
	pub bold: String,
	pub italic: String,
	pub bold_italic: String
}

/// Data for what font sizes to use and how large tabs and various newline sizes should be.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FontSizeData
{
	title_font_size: f32,
	header_font_size: f32,
	body_font_size: f32,
	tab_amount: f32,
	title_newline_amount: f32,
	header_newline_amount: f32,
	body_newline_amount: f32
}

impl FontSizeData
{
	/// Constructor
	///
	/// # Parameters
	/// - `title_font_size` Cover page text font size.
	/// - `header_font_size` Spell name font size.
	/// - `body_font_size` Font size for everything else.
	/// - `tab_amount` Tab size in printpdf Mm.
	/// - `title_newline_amount` Newline size for title text in printpdf Mm.
	/// - `header_newline_amount` Newline size for header text in printpdf Mm.
	/// - `body_newline_amount` Newline size for body text in printpdf Mm.
	///
	/// # Output
	/// - `Ok` A FontSizeData object.
	/// - `Err` An error message saying which parameter was invalid. Occurs for negative values.
	pub fn new(title_font_size: f32, header_font_size: f32, body_font_size: f32, tab_amount: f32,
	title_newline_amount: f32, header_newline_amount: f32, body_newline_amount: f32) -> Result<Self, String>
	{
		// Makes sure no values are below 0
		if title_font_size < 0.0 { Err(String::from("Invalid title_font_size.")) }
		else if header_font_size < 0.0 { Err(String::from("Invalid header_font_size.")) }
		else if body_font_size < 0.0 { Err(String::from("Invalid body_font_size.")) }
		else if tab_amount < 0.0 { Err(String::from("Invalid tab_amount.")) }
		else if title_newline_amount < 0.0 { Err(String::from("Invalid title_newline_amount.")) }
		else if header_newline_amount < 0.0 { Err(String::from("Invalid header_newline_amount.")) }
		else if body_newline_amount < 0.0 { Err(String::from("Invalid body_newline_amount.")) }
		else
		{
			Ok(Self
			{
				title_font_size: title_font_size,
				header_font_size: header_font_size,
				body_font_size: body_font_size,
				tab_amount: tab_amount,
				title_newline_amount: title_newline_amount,
				header_newline_amount: header_newline_amount,
				body_newline_amount: body_newline_amount
			})
		}
	}

	// Getters
	pub fn title_font_size(&self) -> f32 { self.title_font_size }
	pub fn header_font_size(&self) -> f32 { self.header_font_size }
	pub fn body_font_size(&self) -> f32 { self.body_font_size }
	pub fn tab_amount(&self) -> f32 { self.tab_amount }
	pub fn title_newline_amount(&self) -> f32 { self.title_newline_amount }
	pub fn header_newline_amount(&self) -> f32 { self.header_newline_amount }
	pub fn body_newline_amount(&self) -> f32 { self.body_newline_amount }
}

/// Scalar values to convert rusttype font units to printpdf millimeters (Mm).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FontScalars
{
	regular: f32,
	bold: f32,
	italic: f32,
	bold_italic: f32
}

impl FontScalars
{
	/// Constructor
	///
	/// # Parameters
	/// - `regular` Scalar value for regular font.
	/// - `bold` Scalar value for bold font.
	/// - `italic` Scalar value for italic font.
	/// - `bold_italic` Scalar value for bold-italic font.
	///
	/// # Output
	/// - `Ok` A FontScalar object.
	// - `Err` An error message saying which parameter was invalid. Occurs for negative values.
	pub fn new(regular: f32, bold: f32, italic: f32, bold_italic: f32) -> Result<Self, String>
	{
		if regular < 0.0 { Err(String::from("Invalid regular scalar.")) }
		else if bold < 0.0 { Err(String::from("Invalid bold scalar.")) }
		else if italic < 0.0 { Err(String::from("Invalid italic scalar.")) }
		else if bold_italic < 0.0 { Err(String::from("Invalid bold_italic scalar.")) }
		else
		{
			Ok(Self
			{
				regular: regular,
				bold: bold,
				italic: italic,
				bold_italic: bold_italic
			})
		}
	}

	// Getters
	pub fn regular_scalar(&self) -> f32 { self.regular }
	pub fn bold_scalar(&self) -> f32 { self.bold }
	pub fn italic_scalar(&self) -> f32 { self.italic }
	pub fn bold_italic_scalar(&self) -> f32 { self.bold_italic }
}

/// Options for tables.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TableOptions
{
	title_font_size: f32,
	horizontal_cell_margin: f32,
	vertical_cell_margin: f32,
	outer_horizontal_margin: f32,
	outer_vertical_margin: f32,
	off_row_color_lines_y_adjust_scalar: f32,
	off_row_color_lines_height_scalar: f32,
	// RGB
	off_row_color: (u8, u8, u8)
}

impl TableOptions
{
	/// Constructor
	///
	/// # Parameters
	/// - `title_font_size` Font size for table title text.
	/// - `horizontal_cell_margin` Space between columns in printpdf Mm.
	/// - `vertical_cell_margin` Space between rows in printpdf Mm.
	/// - `outer_horizontal_margin` Minimum space between sides of table and edge of pages.
	/// - `outer_horizontal_margin` Space above and below table from other text / tables.
	/// - `off_row_color_lines_y_adjust_scalar` Scalar value to adjust off-row color lines to line up with the rows vertically.
	/// - `off_row_color_lines_height_scalar` Scalar value to determine the height of off-row color lines.
	/// - `off_row_color` RGB value of the color of the off-row color lines.
	///
	/// # Output
	/// - `Ok` A TableOptions object.
	/// - `Err` An error message saying which parameter was invalid. Occurs for negative values.
	pub fn new(title_font_size: f32, horizontal_cell_margin: f32, vertical_cell_margin: f32, outer_horizontal_margin: f32,
	outer_vertical_margin: f32, off_row_color_lines_y_adjust_scalar: f32, off_row_color_lines_height_scalar: f32,
	off_row_color: (u8, u8, u8)) -> Result<Self, String>
	{
		// Makes sure none of the float values are below 0
		if title_font_size < 0.0 { Err(String::from("Invalid title_font_size.")) }
		else if horizontal_cell_margin < 0.0 { Err(String::from("Invalid horizontal_cell_margin.")) }
		else if vertical_cell_margin < 0.0 { Err(String::from("Invalid vertical_cell_margin.")) }
		else if outer_horizontal_margin < 0.0 { Err(String::from("Invalid outer_horizontal_margin.")) }
		else if outer_vertical_margin < 0.0 { Err(String::from("Invalid outer_vertical_margin.")) }
		else if off_row_color_lines_y_adjust_scalar < 0.0
		{ Err(String::from("Invalid off_row_color_lines_y_adjust_scalar.")) }
		else if off_row_color_lines_height_scalar < 0.0
		{ Err(String::from("Invalid off_row_color_lines_height_scalar.")) }
		else
		{
			Ok(Self
			{
				title_font_size: title_font_size,
				horizontal_cell_margin: horizontal_cell_margin,
				vertical_cell_margin: vertical_cell_margin,
				outer_horizontal_margin: outer_horizontal_margin,
				outer_vertical_margin: outer_vertical_margin,
				off_row_color_lines_y_adjust_scalar: off_row_color_lines_y_adjust_scalar,
				off_row_color_lines_height_scalar: off_row_color_lines_height_scalar,
				off_row_color: off_row_color
			})
		}
	}

	// Getters
	pub fn title_font_size(&self) -> f32 { self.title_font_size }
	pub fn horizontal_cell_margin(&self) -> f32 { self.horizontal_cell_margin }
	pub fn vertical_cell_margin(&self) -> f32 { self.vertical_cell_margin }
	pub fn outer_horizontal_margin(&self) -> f32 { self.outer_horizontal_margin }
	pub fn outer_vertical_margin(&self) -> f32 { self.outer_vertical_margin }
	pub fn off_row_color_lines_y_adjust_scalar(&self) -> f32 { self.off_row_color_lines_y_adjust_scalar }
	pub fn off_row_color_lines_height_scalar(&self) -> f32 { self.off_row_color_lines_height_scalar }
	pub fn off_row_color(&self) -> (u8, u8, u8) { self.off_row_color }
	// Gives specific values for each rgb value for the off row color
	pub fn off_row_red(&self) -> u8 { self.off_row_color.0 }
	pub fn off_row_green(&self) -> u8 { self.off_row_color.1 }
	pub fn off_row_blue(&self) -> u8 { self.off_row_color.2 }
}

/// RGB colors for types of text in the spellbook.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TextColors
{
	/// Cover page text.
	pub title_color: (u8, u8, u8),
	/// Spell name text.
	pub header_color: (u8, u8, u8),
	/// Everything else.
	pub body_color: (u8, u8, u8)
}

/// Data for determining the size of the page and the margins between sides of the pages and text.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PageSizeData
{
	width: f32,
	height: f32,
	left_margin: f32,
	right_margin: f32,
	top_margin: f32,
	bottom_margin: f32
}

impl PageSizeData
{
	/// Constructor
	///
	/// # Parameters
	/// - `width` Width of the page in printpdf Mm. Standard is 210.
	/// - `height` Height of the page in printpdf Mm. Standard is 297.
	/// - `left_margin` Space between text and left side of page.
	/// - `right_margin` Space between text and right side of page.
	/// - `top_margin` Space between text and top of page.
	/// - `bottom_margin` Space between text and bottom of page.
	///
	/// # Output
	/// - `Ok` A PageSizeData object.
	/// - `Err` An error message saying which parameter(s) was / were invalid. Occurs for negative or overlapping values.
	pub fn new(width: f32, height: f32, left_margin: f32, right_margin: f32, top_margin: f32,
	bottom_margin: f32) -> Result<Self, String>
	{
		// Determines the minimum page dimension between width and height
		let min_dim = width.min(height);
		// If the width is below 0, return an error
		if width <= 0.0
		{
			Err(String::from("Invalid page width."))
		}
		// If the height is below 0, return an error
		else if height <= 0.0
		{
			Err(String::from("Invalid page height."))
		}
		// If either horizontal margin is below 0 or they are combined too big for there to be any text on the page
		else if left_margin <= 0.0 || right_margin <= 0.0 || left_margin + right_margin >= min_dim
		{
			// Return an error
			Err(String::from("Invalid horizontal page margin."))
		}
		// If either vertical margin is below 0 or they are combined too big for there to be any text on the page
		else if top_margin <= 0.0 || bottom_margin <= 0.0 || top_margin + bottom_margin >= min_dim
		{
			// Return an error
			Err(String::from("Invalid vertical page margin."))
		}
		// If it's all ok, construct and return
		else
		{
			Ok(Self
			{
				width: width,
				height: height,
				left_margin: left_margin,
				right_margin: right_margin,
				top_margin: top_margin,
				bottom_margin: bottom_margin
			})
		}
	}

	// Getters
	pub fn width(&self) -> f32 { self.width }
	pub fn height(&self) -> f32 { self.height }
	pub fn left_margin(&self) -> f32 { self.left_margin }
	pub fn right_margin(&self) -> f32 { self.right_margin }
	pub fn top_margin(&self) -> f32 { self.top_margin }
	pub fn bottom_margin(&self) -> f32 { self.bottom_margin }

	// Returns whether or not all of the margins are the same for this object
	pub fn has_same_margins(&self) -> bool
	{
		return self.left_margin == self.right_margin && self.left_margin == self.top_margin &&
			self.left_margin == self.bottom_margin
	}
}

/// Parameters for determining page number behavior.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PageNumberData
{
	start_on_left: bool,
	flip_sides: bool,
	starting_num: i32,
	side_margin: f32,
	bottom_margin: f32
}

impl PageNumberData
{
	/// Constructor
	///
	/// # Parameters
	/// - `start_on_left` Whether or not the page numbers start on the left side.
	/// If the page numbers do not flip sides, this determines what side all page numbers are on.
	/// - `flip_sides` Whether or not the page numbers flip sides every page.
	/// - `starting_num` What number to have the page numbers start on.
	/// - `side_margin` The distance between the page numbers and the side of the page.
	/// - `bottom_margin` The distance between the page numbers and the bottom of the page.
	///
	/// # Output
	/// - `Ok` A PageNumberData object.
	/// - `Err` An error message saying which parameter was invalid. Occurs for negative margin values.
	pub fn new(start_on_left: bool, flip_sides: bool, starting_num: i32, side_margin: f32, bottom_margin: f32)
	-> Result<Self, String>
	{
		// If the side margin is less than 0, return an error
		if side_margin < 0.0
		{
			Err(String::from("Invalid side margin."))
		}
		// If the bottom margin is less than 0, return an error
		else if bottom_margin < 0.0
		{
			Err(String::from("Invalid bottom margin."))
		}
		// If both of those values are ok, construct and return
		else
		{
			Ok(Self
			{
				start_on_left: start_on_left,
				flip_sides: flip_sides,
				starting_num: starting_num,
				side_margin: side_margin,
				bottom_margin: bottom_margin
			})
		}
	}

	// Getters
	pub fn start_on_left(&self) -> bool { self.start_on_left }
	pub fn flip_sides(&self) -> bool { self.flip_sides }
	pub fn starting_num(&self) -> i32 { self.starting_num }
	pub fn side_margin(&self) -> f32 { self.side_margin }
	pub fn bottom_margin(&self) -> f32 { self.bottom_margin }
}

/// Creates a spellbook from a list of spells.
///
/// # Parameters
/// - `title` The name of the spellbook. It will determine what text appears on the cover page and what the pdf document will be named in the meta data.
/// - `spell_list` The list of spells that the spellbook will contain. The spells do not have to be in any particular order.
/// - `font_paths` Struct containing the file paths to the regular, bold, italic, and bold-italic fonts that the spellbook will use for the text.
/// - `page_size_data` Struct containing the data that determines the size of the page and the text margins (space between edge of page and text).
/// - `page_number_options` Option containing a struct of the page number behavior (starting number, positioning, flip sides or not, etc.). A value of `None` will make the spellbook have no page numbers.
/// - `font_size_data` Struct containing the font size for various types of text and spacing behavior like newline amounts and tabbing amounts.
/// - `text_colors` Struct containing the rgb values for each type of text in the spellbook.
/// - `font_scalars` Numbers that determine how the size of each font is calculated. Numbers being slightly off may lead to text spilling off the page or going to new lines too early.
/// You may need to tinker with these values for the fonts you are using until the text in your spellbooks look good to get it right.
/// - `table_options` Struct containing options that determine the appearance of tables.
/// - `background_img_data` Option containing the data needed to put a background image on every page in the spellbook.
/// The `&str` is the file path to the background image and the `&ImageTransform` is a struct containing options that determine the sizing and rotation of the image.
///
/// # Output
/// Returns a Result enum.
/// - `Ok` Returns a struct containing the data of the spellbook that can be saved to a file.
/// This struct is a printpdf::PdfDocumentReference from the printpdf crate (<https://docs.rs/printpdf/latest/printpdf/struct.PdfDocumentReference.html>).
/// - `Err` Returns any errors that occurred.
pub fn generate_spellbook
(
	title: &str, spell_list: &Vec<spells::Spell>, font_paths: &FontPaths,
	page_size_data: &PageSizeData, page_number_options: &Option<PageNumberData>, font_size_data: &FontSizeData,
	text_colors: &TextColors, font_scalars: &FontScalars, table_options: &TableOptions,
	background_img_data: &Option<(&str, &ImageTransform)>
) -> Result<PdfDocumentReference, Box<dyn std::error::Error>>
{
	// Construct the text colors
	let title_color = Color::Rgb(Rgb::new
	(
		text_colors.title_color.0 as f32 / 255.0,
		text_colors.title_color.1 as f32 / 255.0,
		text_colors.title_color.2 as f32 / 255.0,
		None
	));
	let header_color = Color::Rgb(Rgb::new
	(
		text_colors.header_color.0 as f32 / 255.0,
		text_colors.header_color.1 as f32 / 255.0,
		text_colors.header_color.2 as f32 / 255.0,
		None
	));
	let body_color = Color::Rgb(Rgb::new
	(
		text_colors.body_color.0 as f32 / 255.0,
		text_colors.body_color.1 as f32 / 255.0,
		text_colors.body_color.2 as f32 / 255.0,
		None
	));
	
    // Load custom font

	// Read the data from the font files
	let regular_font_data = fs::read(&font_paths.regular)?;
	let bold_font_data = fs::read(&font_paths.bold)?;
	let italic_font_data = fs::read(&font_paths.italic)?;
	let bold_italic_font_data = fs::read(&font_paths.bold_italic)?;

	// Create font size data for each font style
	let result = Font::try_from_bytes(&regular_font_data as &[u8]);
	let regular_font_size_data = match result
	{
		Some(d) => d,
		None => panic!("Could not convert regular font data to bytes.")
	};
	let result = Font::try_from_bytes(&bold_font_data as &[u8]);
	let bold_font_size_data = match result
	{
		Some(d) => d,
		None => panic!("Could not convert bold font data to bytes.")
	};
	let result = Font::try_from_bytes(&italic_font_data as &[u8]);
	let italic_font_size_data = match result
	{
		Some(d) => d,
		None => panic!("Could not convert italic font data to bytes.")
	};
	let result = Font::try_from_bytes(&bold_italic_font_data as &[u8]);
	let bold_italic_font_size_data = match result
	{
		Some(d) => d,
		None => panic!("Could not convert bold italic font data to bytes.")
	};

	// Create font scale objects for each font size
	let title_font_scale = Scale::uniform(font_size_data.title_font_size());
	let header_font_scale = Scale::uniform(font_size_data.header_font_size());
	let body_font_scale = Scale::uniform(font_size_data.body_font_size());
	let table_title_font_scale = Scale::uniform(table_options.title_font_size());

	// Font types for calculating size of text with certain fonts
	let regular_font_type = FontType::Regular;
	let bold_font_type = FontType::Bold;
	let italic_font_type = FontType::Italic;
	//let bold_italic_font_type = FontType::Italic;

	// Load background image
	let img_data = match *background_img_data
	{
		// If there is a background image, open it
		Some((path, transform)) => Some((image::open(path)?, *transform)),
		None => None
	};
	// Create printpdf image object to add background to cover page if there is a background
    let cover_img = match img_data
	{
		Some((ref img, _)) => Some(Image::from_dynamic_image(&img.clone())),
		None => None
	};

    // Create PDF document
    let (doc, cover_page, cover_layer) = PdfDocument::new(title, Mm(page_size_data.width), Mm(page_size_data.height),
		"Cover Layer");

    // Add all styles of the custom font to the document
    let regular_font = doc.add_external_font(&*regular_font_data)?;
	let italic_font = doc.add_external_font(&*italic_font_data)?;
	let bold_font = doc.add_external_font(&*bold_font_data)?;
	let bold_italic_font = doc.add_external_font(&*bold_italic_font_data)?;

	// Create bookmark for cover page
	doc.add_bookmark("Cover", cover_page);

    // Get PdfLayerReference from PdfLayerIndex
	let cover_layer_ref = doc.get_page(cover_page).get_layer(cover_layer);

	// Determine whether or not there is a background image
	match *background_img_data
	{
		// If there is a background image
		Some((_, transform)) =>
		{
			if let Some(img) = cover_img
			{
				// Add it to the page
				img.add_to_layer(cover_layer_ref.clone(), *transform);
			}
		},
		// If there is not background image, do nothing
		None => (),
	}

	// Counter variable for naming each layer incrementally
	let mut layer_count = 1;

	// Flag for telling which side of the page the page numbers are on, if there are any page numbers
	// Left is true, right is false
	#[allow(unused_assignments)]
	let mut left = true;
	// Create the page number data parameters
	let mut page_number_data = match page_number_options
	{
		// If there are page number options
		Some(options) => 
		{
			// Determine which side the page numbers should start on
			left = options.start_on_left();
			// Put all of the page number parameters into a tuple
			Some((options, &mut left, font_size_data.body_font_size(), &regular_font_type, &regular_font,
				&regular_font_size_data, &body_font_scale))
		},
		None => None
	};

	// The positions of each side of the textbox of the page determined by the margins in the page size data
	let x_left = page_size_data.left_margin;
	let x_right = page_size_data.width - page_size_data.right_margin;
	let y_top = page_size_data.height - page_size_data.top_margin;
	let y_low = page_size_data.bottom_margin;

	// Temporary x and y position values needed to call write_centered_textbox
	let mut temp_x: f32 = 0.0;
	let mut temp_y: f32 = 0.0;

	// Prefix for the names of new layers created while creating the cover page(s)
	let cover_layer_name_prefix = "Cover Layer";

    // Add text using the custom font to the page
	let _ = write_centered_textbox
	(
		&doc, cover_layer_name_prefix, &cover_layer_ref, &mut None, &mut layer_count, &img_data,
		font_scalars, title, &title_color, font_size_data.title_font_size(), page_size_data.width, page_size_data.height,
		x_left, x_right, y_top, y_low, &mut temp_x, &mut temp_y, &regular_font_type, &regular_font,
		&regular_font_size_data, &title_font_scale, font_size_data.title_newline_amount()
	);
	
	// Reset the layer count so it begins at either 1 or the desired starting page number
	layer_count = match page_number_options
	{
		// If there are page number options, use the desired starting page number
		Some(options) => options.starting_num(),
		// If there are no page number options, just start at 1
		None => 1
	};
	// Prefix for the names of new layers created while creating new spell pages
	let spell_layer_name_prefix = "Layer";

	// Add next pages

	// Loop through each spell
	for spell in spell_list
	{
		// Create a new page
		let (page, mut layer_ref) = make_new_page
		(
			&doc, spell_layer_name_prefix, &mut page_number_data, &mut layer_count,
			page_size_data.width, page_size_data.height, &img_data, font_scalars
		);
		// Create a new bookmark for this page
		doc.add_bookmark(spell.name.clone(), page);
		// Keeps track of the current x and y position to place text at
		let mut x: f32 = x_left;
		let mut y: f32 = y_top;

		// Add text to the page

		// Add the name of the spell as a header
		layer_ref = write_textbox
		(
			&doc, spell_layer_name_prefix, &layer_ref, &mut page_number_data, &mut layer_count,
			&img_data, font_scalars, &spell.name, &header_color, font_size_data.header_font_size(), page_size_data.width,
			page_size_data.height, x_left, x_right, y_top, y_low, &mut x, &mut y, &regular_font_type, &regular_font,
			&regular_font_size_data, &header_font_scale, font_size_data.tab_amount(),
			font_size_data.header_newline_amount()
		);
		// Move the y position down a bit to put some extra space between lines of text
		y -= font_size_data.header_newline_amount();
		// Reset the x position back to the starting position
		x = x_left;

		// Add the level and the spell's school of magic
		layer_ref = write_textbox
		(
			&doc, spell_layer_name_prefix, &layer_ref, &mut page_number_data, &mut layer_count,
			&img_data, font_scalars, &spell.get_level_school_text(), &body_color, font_size_data.body_font_size(),
			page_size_data.width, page_size_data.height, x_left, x_right, y_top, y_low, &mut x, &mut y, &italic_font_type,
			&italic_font, &italic_font_size_data, &body_font_scale, font_size_data.tab_amount(),
			font_size_data.body_newline_amount()
		);
		y -= font_size_data.header_newline_amount();
		x = x_left;

		// Add the casting time of the spell
		layer_ref = write_spell_field
		(
			&doc, spell_layer_name_prefix, &layer_ref, &mut page_number_data, &mut layer_count,
			&img_data, font_scalars, "Casting Time:", &spell.casting_time.to_string(), &body_color, &body_color,
			font_size_data.body_font_size(), page_size_data.width, page_size_data.height, x_left, x_right, y_top, y_low,
			&mut x, &mut y, &bold_font_type, &bold_font, &bold_font_size_data, &regular_font, &bold_font, &italic_font,
			&bold_italic_font, &regular_font_size_data, &bold_font_size_data, &italic_font_size_data,
			&bold_italic_font_size_data, &body_font_scale, table_options, &table_title_font_scale,
			font_size_data.tab_amount(), font_size_data.body_newline_amount()
		);
		y -= font_size_data.body_newline_amount();
		x = x_left;


		// Add the range of the spell
		layer_ref = write_spell_field
		(
			&doc, spell_layer_name_prefix, &layer_ref, &mut page_number_data, &mut layer_count,
			&img_data, font_scalars, "Range:", &spell.range.to_string(), &body_color, &body_color,
			font_size_data.body_font_size(), page_size_data.width, page_size_data.height, x_left, x_right, y_top, y_low,
			&mut x, &mut y, &bold_font_type, &bold_font, &bold_font_size_data, &regular_font, &bold_font, &italic_font,
			&bold_italic_font, &regular_font_size_data, &bold_font_size_data, &italic_font_size_data,
			&bold_italic_font_size_data, &body_font_scale, table_options, &table_title_font_scale,
			font_size_data.tab_amount(), font_size_data.body_newline_amount()
		);
		y -= font_size_data.body_newline_amount();
		x = x_left;

		// Add the components of the spell
		layer_ref = write_spell_field
		(
			&doc, spell_layer_name_prefix, &layer_ref, &mut page_number_data, &mut layer_count,
			&img_data, font_scalars, "Components:", &spell.get_component_string(), &body_color, &body_color,
			font_size_data.body_font_size(), page_size_data.width, page_size_data.height, x_left, x_right, y_top, y_low,
			&mut x, &mut y, &bold_font_type, &bold_font, &bold_font_size_data, &regular_font, &bold_font, &italic_font,
			&bold_italic_font, &regular_font_size_data, &bold_font_size_data, &italic_font_size_data,
			&bold_italic_font_size_data, &body_font_scale, table_options, &table_title_font_scale,
			font_size_data.tab_amount(), font_size_data.body_newline_amount()
		);
		y -= font_size_data.body_newline_amount();
		x = x_left;

		// Add the duration of the spell
		layer_ref = write_spell_field
		(
			&doc, spell_layer_name_prefix, &layer_ref, &mut page_number_data, &mut layer_count,
			&img_data, font_scalars, "Duration:", &spell.duration.to_string(), &body_color, &body_color,
			font_size_data.body_font_size(), page_size_data.width, page_size_data.height, x_left, x_right, y_top, y_low,
			&mut x, &mut y, &bold_font_type, &bold_font, &bold_font_size_data, &regular_font, &bold_font, &italic_font,
			&bold_italic_font, &regular_font_size_data, &bold_font_size_data, &italic_font_size_data,
			&bold_italic_font_size_data, &body_font_scale, table_options, &table_title_font_scale,
			font_size_data.tab_amount(), font_size_data.body_newline_amount()
		);
		y -= font_size_data.header_newline_amount();
		x = x_left;

		// Add the spell's description
		layer_ref = write_spell_description
		(
			&doc, spell_layer_name_prefix, &layer_ref, &mut page_number_data,
			&mut layer_count, &img_data, font_scalars, &spell.description, &body_color, font_size_data.body_font_size(),
			page_size_data.width, page_size_data.height, x_left, x_right, y_top, y_low, &mut x, &mut y, &regular_font,
			&bold_font, &italic_font, &bold_italic_font, &regular_font_size_data, &bold_font_size_data,
			&italic_font_size_data, &bold_italic_font_size_data, &body_font_scale, table_options, &table_title_font_scale,
			font_size_data.tab_amount(), font_size_data.body_newline_amount()
		);

		// If the spell has an upcast description
		if let Some(description) = &spell.upcast_description
		{
			y -= font_size_data.body_newline_amount();
			x = x_left + font_size_data.tab_amount();
			let text = format!("<bi> At Higher Levels. <r> {}", description);
			_ = write_spell_description
			(
				&doc, spell_layer_name_prefix, &layer_ref, &mut page_number_data, &mut layer_count,
				&img_data, font_scalars, &text, &body_color, font_size_data.body_font_size(), page_size_data.width,
				page_size_data.height, x_left, x_right, y_top, y_low, &mut x, &mut y, &regular_font, &bold_font,
				&italic_font, &bold_italic_font, &regular_font_size_data, &bold_font_size_data, &italic_font_size_data,
				&bold_italic_font_size_data, &body_font_scale, table_options, &table_title_font_scale,
				font_size_data.tab_amount(), font_size_data.body_newline_amount()
			);
		}
	}

	// Return the pdf document
    Ok(doc)
}

/// Saves spellbooks to a file as a pdf document.
///
/// #### Parameters
/// - `doc` A spellbook that gets returned from `generate_spellbook()`.
/// - `file_name` The name to give to the file that the spellbook will be saved to.
/// 
/// #### Output
/// Returns a Result enum.
/// - `Ok` Returns nothing.
/// - `Err` Returns any errors that occurred.
pub fn save_spellbook(doc: PdfDocumentReference, file_name: &str) -> Result<(), Box<dyn std::error::Error>>
{
	let file = fs::File::create(file_name)?;
	doc.save(&mut std::io::BufWriter::new(file))?;
	Ok(())
}

/// Error for when a file name could not be retrieved when processing spell files in `get_all_spells_in_folder()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpellFileNameReadError;
// Makes the struct displayable
impl std::fmt::Display for SpellFileNameReadError
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(f, "Couldn't find a file name.")
	}
}
// Makes the struct officially an error
impl std::error::Error for SpellFileNameReadError {}

/// Returns a vec of spells from every spell file in a folder.
///
/// It assumes that all files in the folder are spell files.
/// 
/// #### Parameters
/// - `folder_path` The file path to the folder to extract every spell from.
/// 
/// #### Output
/// Returns a Result enum.
/// - `Ok` Returns a vec of spell objects that can be inputted into `generate_spellbook()`.
/// - `Err` Returns any errors that occurred.
pub fn get_all_spells_in_folder(folder_path: &str) -> Result<Vec<spells::Spell>, Box<dyn std::error::Error>>
{
	// Gets a list of every file in the folder
	let file_paths = fs::read_dir(folder_path)?;
	// Create a list of the spells that will be returned
	let mut spell_list = Vec::new();
	// Loop through each file in the folder
	for file_path in file_paths
	{
		// Attempt to get a path to the file
		let file_name_option = file_path?.path();
		// Attempt to turn the path into a string
		let file_name = match file_name_option.to_str()
		{
			// If an str of the path was retrieved successfully, obtain it
			Some(name) => name,
			// If an str of the path could not be gotten, return an error
			None => return Err(Box::new(SpellFileNameReadError))
		};
		// Read the spell file, turn it into a spell, and push it to the spell_list vec
		spell_list.push(spells::Spell::from_file(file_name)?);
	}
	// Return the list of spells
	Ok(spell_list)
}

#[cfg(test)]
mod tests
{
	use super::*;
	use std::path::Path;

	// Creates 2 spellbooks that combined contain every spell from the official d&d 5e player's handbook
	#[test]
	fn players_handbook()
	{
		// Spellbook names
		let spellbook_name_1 = "Every Sepll in the Dungeons & Dragons 5th Edition Player's Handbook: Part 1";
		let spellbook_name_2 = "Every Sepll in the Dungeons & Dragons 5th Edition Player's Handbook: Part 2";
		// List of every spell in the player's handbook folder
		let spell_list = get_all_spells_in_folder("spells/players_handbook").unwrap();
		// Split that vec into 2 vecs
		let spell_list_1 = spell_list[..spell_list.len()/2].to_vec();
		let spell_list_2 = spell_list[spell_list.len()/2..].to_vec();
		// File paths to the fonts needed
		let font_paths = FontPaths
		{
			regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
			bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
			italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
			bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
		};
		// Parameters for determining the size of the page and the text margins on the page
		let page_size_data = PageSizeData::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
		// Parameters for determining page number behavior
		let page_number_data = PageNumberData::new(true, false, 1, 5.0, 4.0).unwrap();
		// Parameters for determining font sizes, the tab amount, and newline amounts
		let font_size_data = FontSizeData::new(32.0, 24.0, 12.0, 7.5, 12.0, 8.0, 5.0).unwrap();
		// Colors for each type of text
		let text_colors = TextColors
		{
			title_color: (0, 0, 0),
			header_color: (115, 26, 26),
			body_color: (0, 0, 0)
		};
		// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
		let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
		// Parameters for table margins / padding and off-row color / scaling
		let table_options = TableOptions::new(16.0, 10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
		// File path to the background image
		let background_path = "img/parchment.jpg";
		// Image transform data for the background image
		let background_transform = ImageTransform
		{
			translate_x: Some(Mm(0.0)),
			translate_y: Some(Mm(0.0)),
			scale_x: Some(1.95),
			scale_y: Some(2.125),
			..Default::default()
		};
		// Creates the spellbooks
		let doc_1 = generate_spellbook
		(
			spellbook_name_1, &spell_list_1, &font_paths, &page_size_data, &Some(page_number_data),
			&font_size_data, &text_colors, &font_scalars, &table_options,
			&Some((background_path, &background_transform))
		).unwrap();
		let doc_2 = generate_spellbook
		(spellbook_name_2, &spell_list_2, &font_paths, &page_size_data, &Some(page_number_data),
			&font_size_data, &text_colors, &font_scalars, &table_options,
			&Some((background_path, &background_transform))
		).unwrap();
		// Saves the spellbooks as pdf documents
		let _ = save_spellbook(doc_1, "Player's Handbook Spells 1.pdf").unwrap();
		let _ = save_spellbook(doc_2, "Player's Handbook Spells 2.pdf").unwrap();
	}

	// Create a spellbook with every spell from the xanathar's guide to everything source book
	#[test]
	fn xanathars_guide_to_everything()
	{
		// Spellbook's name
		let spellbook_name = "Every Sepll in the Dungeons & Dragons 5th Edition Source Material Book \"Xanathar's Guide to Everything\"";
		// List of every spell in this folder
		let spell_list = get_all_spells_in_folder("spells/xanathars_guide_to_everything").unwrap();
		// File paths to the fonts needed
		let font_paths = FontPaths
		{
			regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
			bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
			italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
			bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
		};
		// Parameters for determining the size of the page and the text margins on the page
		let page_size_data = PageSizeData::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
		// Parameters for determining page number behavior
		let page_number_data = PageNumberData::new(true, false, 1, 5.0, 4.0).unwrap();
		// Parameters for determining font sizes, the tab amount, and newline amounts
		let font_size_data = FontSizeData::new(32.0, 24.0, 12.0, 7.5, 12.0, 8.0, 5.0).unwrap();
		// Colors for each type of text
		let text_colors = TextColors
		{
			title_color: (0, 0, 0),
			header_color: (115, 26, 26),
			body_color: (0, 0, 0)
		};
		// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
		let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
		// Parameters for table margins / padding and off-row color / scaling
		let table_options = TableOptions::new(16.0, 10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
		// File path to the background image
		let background_path = "img/parchment.jpg";
		// Image transform data for the background image
		let background_transform = ImageTransform
		{
			translate_x: Some(Mm(0.0)),
			translate_y: Some(Mm(0.0)),
			scale_x: Some(1.95),
			scale_y: Some(2.125),
			..Default::default()
		};
		// Creates the spellbook
		let doc = generate_spellbook
		(
			spellbook_name, &spell_list, &font_paths, &page_size_data, &Some(page_number_data),
			&font_size_data, &text_colors, &font_scalars, &table_options,
			&Some((background_path, &background_transform))
		).unwrap();
		// Saves the spellbook to a pdf document
		let _ = save_spellbook(doc, "Xanathar's Guide to Everything Spells.pdf");
	}

	// Create a spellbook with every spell from the tasha's cauldron of everything source book
	#[test]
	fn tashas_cauldron_of_everything()
	{
		// Spellbook's name
		let spellbook_name = "Every Sepll in the Dungeons & Dragons 5th Edition Source Material Book \"Tasha's Cauldron of Everything\"";
		// List of every spell in this folder
		let spell_list = get_all_spells_in_folder("spells/tashas_cauldron_of_everything").unwrap();
		// File paths to the fonts needed
		let font_paths = FontPaths
		{
			regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
			bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
			italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
			bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
		};
		// Parameters for determining the size of the page and the text margins on the page
		let page_size_data = PageSizeData::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
		// Parameters for determining page number behavior
		let page_number_data = PageNumberData::new(true, false, 1, 5.0, 4.0).unwrap();
		// Parameters for determining font sizes, the tab amount, and newline amounts
		let font_size_data = FontSizeData::new(32.0, 24.0, 12.0, 7.5, 12.0, 8.0, 5.0).unwrap();
		// Colors for each type of text
		let text_colors = TextColors
		{
			title_color: (0, 0, 0),
			header_color: (115, 26, 26),
			body_color: (0, 0, 0)
		};
		// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
		let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
		// Parameters for table margins / padding and off-row color / scaling
		let table_options = TableOptions::new(16.0, 10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
		// File path to the background image
		let background_path = "img/parchment.jpg";
		// Image transform data for the background image
		let background_transform = ImageTransform
		{
			translate_x: Some(Mm(0.0)),
			translate_y: Some(Mm(0.0)),
			scale_x: Some(1.95),
			scale_y: Some(2.125),
			..Default::default()
		};
		// Creates the spellbook
		let doc = generate_spellbook
		(
			spellbook_name, &spell_list, &font_paths, &page_size_data, &Some(page_number_data),
			&font_size_data, &text_colors, &font_scalars, &table_options,
			&Some((background_path, &background_transform))
		).unwrap();
		// Saves the spellbook to a pdf document
		let _ = save_spellbook(doc, "Tasha's Cauldron of Everything Spells.pdf");
	}

	// Create a spellbook with every spell from the strixhaven: a curriculum of chaos source book
	#[test]
	fn strixhaven()
	{
		// Spellbook's name
		let spellbook_name =
		"Every Sepll in the Dungeons & Dragons 5th Edition Source Material Book \"Strixhaven: A Curriculum of Chaos\"";
		// List of every spell in this folder
		let spell_list = get_all_spells_in_folder("spells/strixhaven").unwrap();
		// File paths to the fonts needed
		let font_paths = FontPaths
		{
			regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
			bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
			italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
			bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
		};
		// Parameters for determining the size of the page and the text margins on the page
		let page_size_data = PageSizeData::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
		// Parameters for determining page number behavior
		let page_number_data = PageNumberData::new(true, false, 1, 5.0, 4.0).unwrap();
		// Parameters for determining font sizes, the tab amount, and newline amounts
		let font_size_data = FontSizeData::new(32.0, 24.0, 12.0, 7.5, 12.0, 8.0, 5.0).unwrap();
		// Colors for each type of text
		let text_colors = TextColors
		{
			title_color: (0, 0, 0),
			header_color: (115, 26, 26),
			body_color: (0, 0, 0)
		};
		// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
		let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
		// Parameters for table margins / padding and off-row color / scaling
		let table_options = TableOptions::new(16.0, 10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
		// File path to the background image
		let background_path = "img/parchment.jpg";
		// Image transform data for the background image
		let background_transform = ImageTransform
		{
			translate_x: Some(Mm(0.0)),
			translate_y: Some(Mm(0.0)),
			scale_x: Some(1.95),
			scale_y: Some(2.125),
			..Default::default()
		};
		// Creates the spellbook
		let doc = generate_spellbook
		(
			spellbook_name, &spell_list, &font_paths, &page_size_data, &Some(page_number_data),
			&font_size_data, &text_colors, &font_scalars, &table_options,
			&Some((background_path, &background_transform))
		).unwrap();
		// Saves the spellbook to a pdf document
		let _ = save_spellbook(doc, "Strixhaven A Curriculum of Chaos Spells.pdf");
	}

	// Makes sure that creating valid spell files works
	#[test]
	fn create_spell_files()
	{
		// String of the path to the output folder
		let output_folder = String::from("spells/generated_spells");
		// If the output folder doesn't exist yet
		if !Path::new(&output_folder).exists()
		{
			// Create it
			fs::create_dir(&output_folder).unwrap();
		}

		// Create the spells (necronomicon spell duplicates)
		let hell_spell = spells::Spell
		{
			name: String::from("HELL SPELL AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A"),
			level: spells::SpellField::Custom(String::from("100TH-LEVEL")),
			school: spells::SpellField::Custom(String::from("SUPER NECROMANCY")),
			is_ritual: true,
			casting_time: spells::SpellField::Controlled(spells::CastingTime::Reaction(String::from("THAT YOU TAKE WHEN YOU FEEL LIKE CASTING SPELLS AND DOING MAGIC AND AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A"))),
			range: spells::SpellField::Controlled(spells::Range::Yourself(Some(spells::AOE::Cylinder(spells::Distance::Miles(63489), spells::Distance::Miles(49729))))),
			has_v_component: true,
			has_s_component: true,
			m_components: Some(String::from("UNLIMITED POWAHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H")),
			duration: spells::SpellField::Controlled(spells::Duration::Years(57394, true)),
			description: String::from("<ib> CASTING SPELLS AND CONJURING ABOMINATIONS <b> AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA <r> THIS SPELL ISN'T FOR <i> weak underpowered feeble wizards -_-. <r> THIS SPELL IS FOR ONLY THE MOST POWERFUL OF ARCHMAGES AND NECROMANCERS WHO CAN WIELD THE MIGHTIEST OF <bi> ARCANE ENERGY <r> WITH THE FORTITUDE OF A <ib> MOUNTAIN. <b> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
<table> <title> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A \\A \\\\A \\\\\\A \\<title> \\\\<title> \\\\\\<title> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A <title>
COLUMN OF CHAOS | COLUMN OF NECROMANCY
<row> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A | A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
<row> B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B | B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B | B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B
<row> POWER | WIZARDRY
<row> C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C | C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C
<table>
MORE MAGIC SPELLS AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
<table> <title> THIS TABLE AGAIN A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A \\A \\\\A \\\\\\A \\<title> \\\\<title> \\\\\\<title> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A <title>
COLUMN OF CHAOS | COLUMN OF NECROMANCY
<row> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A | A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
<row> B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B | B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B | B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B
<row> POWER | WIZARDRY
<row> C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C | C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C
<table>
YOU CAN'T HANDLE THIS SPELL A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A"),
			upcast_description: Some(String::from("HELL ON EARTH"))
		};
		let power_word_scrunch = spells::Spell
		{
			name: String::from("Power Word Scrunch"),
			level: spells::SpellField::Controlled(spells::Level::Level9),
			school: spells::SpellField::Controlled(spells::MagicSchool::Transmutation),
			is_ritual: false,
			casting_time: spells::SpellField::Controlled(spells::CastingTime::Actions(1)),
			range: spells::SpellField::Controlled(spells::Range::Dist(spells::Distance::Feet(60))),
			has_v_component: true,
			has_s_component: false,
			m_components: None,
			duration: spells::SpellField::Controlled(spells::Duration::Instant),
			description: String::from("Choose 1 target creature or object within range. That target gets scrunched.
- Scrunching has these effects <table> <title> Scrunching Effects <title>
Target | Effect
<row> Creature | Flesh Ball
<row> Object | Ball of that object's material
<row> Creature not made of flesh | Ball of that creature's material
<table>
- Scrunch balls (balls produced from scrunching) can be thrown and do 1d6 bludgeoning damage on hit.
Scrunch ball funny lol."),
			upcast_description: None
		};
		let the_ten_hells = spells::Spell
		{
			name: String::from("The Ten Hells"),
			level: spells::SpellField::Controlled(spells::Level::Level9),
			school: spells::SpellField::Controlled(spells::MagicSchool::Necromancy),
			is_ritual: true,
			casting_time: spells::SpellField::Controlled(spells::CastingTime::Actions(1)),
			range: spells::SpellField::Controlled(spells::Range::Yourself(Some(spells::AOE::Sphere(spells::Distance::Feet(90))))),
			has_v_component: true,
			has_s_component: false,
			m_components: Some(String::from("the nail or claw of a creature from an evil plane")),
			duration: spells::SpellField::Controlled(spells::Duration::Instant),
			description: String::from("Choose any number of creatures made of tangible matter within range. Those creatures must all make a constitution savint throw against your spell save DC. All creatures that fail this saving throw get turned inside out, immediately die, and have their souls eternally damned to all nine hells simultaneously.
Creatures that succeed the saving throw take 20d4 scrunching damage."),
			upcast_description: None
		};

		// The file paths that each of the spells will be written to
		let hell_spell_path = output_folder.clone() + "/hell_spell.spell";
		let power_word_scrunch_path = output_folder.clone() + "/power_word_scrunch.spell";
		let the_ten_hells_path = output_folder.clone() + "/the_ten_hells.spell";
		// Write the spells to their own files in the output folder
		hell_spell.to_file(&hell_spell_path, true).unwrap();
		power_word_scrunch.to_file(&power_word_scrunch_path, true).unwrap();
		the_ten_hells.to_file(&the_ten_hells_path, true).unwrap();

		// Get a vec of the spells that were just created in this test function
		let real_spell_list = vec![hell_spell, power_word_scrunch, the_ten_hells];
		// Get a vec of the spells from the spell files that were just created
		let test_spell_list = get_all_spells_in_folder(&output_folder).unwrap();
		// Ensure that they are exactly the same
		assert_eq!(real_spell_list, test_spell_list);

		// Read the bytes from the spell files that were just created
		let test_hell_spell_bytes = fs::read(&hell_spell_path).unwrap();
		let test_power_word_scrunch_bytes = fs::read(&power_word_scrunch_path).unwrap();
		let test_the_ten_hells_bytes = fs::read(&the_ten_hells_path).unwrap();

		// Read the bytes from the hand made spells that these test spells were based on
		let real_hell_spell_bytes = fs::read("spells/necronomicon/hell_spell.spell").unwrap();
		let real_power_word_scrunch_bytes = fs::read("spells/necronomicon/power_word_scrunch.spell").unwrap();
		let real_the_ten_hells_bytes = fs::read("spells/necronomicon/the_ten_hells.spell").unwrap();

		// Ensure that they are all exactly the same
		assert_eq!(real_hell_spell_bytes, test_hell_spell_bytes);
		assert_eq!(real_power_word_scrunch_bytes, test_power_word_scrunch_bytes);
		assert_eq!(real_the_ten_hells_bytes, test_the_ten_hells_bytes);
	}

	// Stress testing the text formatting
	#[test]
	fn necronomicon()
	{
		// Spellbook's name
		let spellbook_name =
		"THE NECROBOMBINOMICON AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A";
		// List of every spell in the stress test folder
		let spell_list = get_all_spells_in_folder("spells/necronomicon").unwrap();
		// File paths to the fonts needed
		let font_paths = FontPaths
		{
			regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
			bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
			italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
			bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
		};
		// Parameters for determining the size of the page and the text margins on the page
		let page_size_data = PageSizeData::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
		// Parameters for determining page number behavior
		let page_number_data = PageNumberData::new(true, true, 1, 5.0, 4.0).unwrap();
		// Parameters for determining font sizes, the tab amount, and newline amounts
		let font_size_data = FontSizeData::new(32.0, 24.0, 12.0, 7.5, 12.0, 8.0, 5.0).unwrap();
		// Colors for each type of text
		let text_colors = TextColors
		{
			title_color: (0, 0, 0),
			header_color: (115, 26, 26),
			body_color: (0, 0, 0)
		};
		// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
		let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
		// Parameters for table margins / padding and off-row color / scaling
		let table_options = TableOptions::new(16.0, 10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
		// Creates the spellbook
		let doc = generate_spellbook
		(
			spellbook_name, &spell_list, &font_paths, &page_size_data, &Some(page_number_data),
			&font_size_data, &text_colors, &font_scalars, &table_options, &None
		).unwrap();
		// Saves the spellbook to a pdf document
		let _ = save_spellbook(doc, "NECRONOMICON.pdf");
	}

	// For creating spellbooks for myself and friends while I work on creating a ui to use this library
	/*#[test]
	fn create_spellbook()
	{
		// Spellbook's name
		let spellbook_name = "A Spellcaster's Spellbook";
		// Vec of spells that will be added to spellbook
		let mut spell_list = Vec::new();
		// Vec of paths to spell files that will be read from
		let spell_paths = vec!
		[
			"spells/players_handbook/prestidigitation.spell",
			"spells/players_handbook/mending.spell",
			"spells/players_handbook/mage_hand.spell",
			"spells/players_handbook/fire_bolt.spell",
			"spells/strixhaven/silvery_barbs.spell",
			"spells/players_handbook/color_spray.spell",
			"spells/players_handbook/magic_missile.spell",
			"spells/xanathars_guide_to_everything/ice_knife.spell",
			"spells/players_handbook/mage_armor.spell",
			"spells/players_handbook/unseen_servant.spell",
			"spells/players_handbook/detect_magic.spell",
			"spells/players_handbook/alarm.spell",
			"spells/players_handbook/cloud_of_daggers.spell",
			"spells/players_handbook/scorching_ray.spell"
		];
		// Attempt to loop through each spell file and convert it into a spell struct
		for path in spell_paths
		{
			println!("{}", path);
			// Convert spell file into spell struct and add it to spell_list vec
			spell_list.push(spells::Spell::from_file(path).unwrap());
		}
		// File paths to the fonts needed
		let font_paths = FontPaths
		{
			regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
			bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
			italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
			bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
		};
		// Parameters for determining the size of the page and the text margins on the page
		let page_size_data = PageSizeData::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
		// Parameters for determining page number behavior
		let page_number_data = PageNumberData::new(true, false, 1, 5.0, 4.0).unwrap();
		// Parameters for determining font sizes, the tab amount, and newline amounts
		let font_size_data = FontSizeData::new(32.0, 24.0, 12.0, 7.5, 12.0, 8.0, 5.0).unwrap();
		// Colors for each type of text
		let text_colors = TextColors
		{
			title_color: (0, 0, 0),
			header_color: (115, 26, 26),
			body_color: (0, 0, 0)
		};
		// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
		let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
		// Parameters for table margins / padding and off-row color / scaling
		let table_options = TableOptions::new(16.0, 10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
		// File path to the background image
		let background_path = "img/parchment.jpg";
		// Image transform data for the background image
		let background_transform = ImageTransform
		{
			translate_x: Some(Mm(0.0)),
			translate_y: Some(Mm(0.0)),
			scale_x: Some(1.95),
			scale_y: Some(2.125),
			..Default::default()
		};
		// Creates the spellbook
		let doc = generate_spellbook
		(
			spellbook_name, &spell_list, &font_paths, &page_size_data, &Some(page_number_data),
			&font_size_data, &text_colors, &font_scalars, &table_options,
			&Some((background_path, &background_transform))
		).unwrap();
		// Saves the spellbook to a pdf document
		let _ = save_spellbook(doc, "Spellbook.pdf");
	}*/
}
