use std::fs;
extern crate image;
use printpdf::*;
use rusttype::{Font, Scale, point};
pub mod spells;
pub mod phb_spells;

// width and height of each page in millimeters
const PAGE_WIDTH: f64 = 210.0;
const PAGE_HEIGHT: f64 = 297.0;

// Number of millimeters to go downwards for newlines
const TITLE_NEWLINE: f64 = 12.0;
const HEADER_NEWLINE: f64 = 8.0;
const BODY_NEWLINE: f64 = 5.0;

// The number of millimeters text gets shifted over on a new paragraph
const TAB_AMOUNT: f64 = 10.0;

// Starting x and y positions for text on a page
const X_START: f64 = 10.0;
const Y_START: f64 = 280.0;

// Ending x and y positions for text on a page
const X_END: f64 = 190.0;
const Y_END: f64 = 10.0;

// The tokens for changing text font
const REGULAR_FONT_TAG: &str = "<r>";
const BOLD_FONT_TAG: &str = "<b>";
const ITALIC_FONT_TAG: &str = "<i>";
const BOLD_ITALIC_FONT_TAG: &str = "<bi>";
const ITALIC_BOLD_FONT_TAG: &str = "<ib>";

// The tokens that are used for processing tables
const TABLE_TAG: &str = "<table>";
const ROW_TAG: &str = "<row>";
const COLUMN_TAG: &str = "|";

// Calculates the width of some text given the font and the font size it uses
fn calc_text_width(font_size_data: &Font, font_scale: &Scale, text: &str) -> f32
{
	let mut width: f32 = 0.0;
	// Loop through each character in the text
	for c in text.chars()
	{
		// Get the glyph of this character for this font
		let glyph = font_size_data.glyph(c);
		// Calculate the width of this character in this font with this font size
		// Add this width to the total
		width += glyph.scaled(*font_scale).h_metrics().advance_width;
	}
	font_units_to_mm(width)
}

fn new_calc_text_width(text: &str, font_type: &str, font_size_data: &Font, font_scale: &Scale) -> f64
{
	let width = font_size_data.layout(text, *font_scale, point(0.0, 0.0))
		.map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
		.last()
		.unwrap_or(0.0);
	let scaler: f32 = match font_type
	{
		BOLD_FONT_TAG => 1.4,
		ITALIC_FONT_TAG => 0.4,
		BOLD_ITALIC_FONT_TAG | ITALIC_BOLD_FONT_TAG => 2.5,
		_ => 0.45
	};
	(width * scaler) as f64
}

// Calculates the height of a number of lines of text given the font, font size, newline size, and number of lines
fn calc_text_height(font_size_data: &Font, font_scale: &Scale, font_size: f32, newline_amount: f64, lines: usize) -> f32
{
	// Calculate the amount of space every newline takes up
	let line_height = ((lines - 1) as f32) * (newline_amount as f32);
	// Calculate the height of a the lower half and the upper half of a line of text in this font
	let v_metrics = font_size_data.v_metrics(*font_scale);
	let font_height = font_units_to_mm(v_metrics.ascent - v_metrics.descent);
	// Return the amount of space the newlines take up plus the space a single line takes up
	line_height + font_height
}

// Converts rusttype font units to printpdf millimeters (Mm)
fn font_units_to_mm(font_unit_width: f32) -> f32
{
	let mm_to_font_ratio = 0.45;
	font_unit_width * mm_to_font_ratio
}

// Calculates the width of the widest text in each column of a table vec along with that column's index
fn get_max_column_widths(table_vec: &Vec<Vec<&str>>, columns: usize, body_font_size_data: &Font,
header_font_size_data: &Font, font_scale: &Scale) -> Vec<(usize, f32)>
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
			let width = if header { header = false; calc_text_width(header_font_size_data, font_scale, table_vec[j][i]) }
			else { calc_text_width(body_font_size_data, font_scale, table_vec[j][i]) };
			max_width = max_width.max(width);
		}
		widths.push((i, max_width));
	}
	widths
}

// Writes a line of text into a textbox
// Returns the layer of a new page if one had to be created for this line to be applied
// Otherwise it returns the layer of the current page
fn apply_textbox_line(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, text: &str, color: &Color, font_size: f32, x_left: f64,
x_right: f64, y_high: f64, y_low: f64, x: &mut f64, y: &mut f64, font: &IndirectFontRef, newline_amount: f64)
-> PdfLayerReference
{
	// The layer that will get returned
	let mut layer_ref = (*layer).clone();
	// Move the text down a level for the new line
	*y -= newline_amount;
	// if the y level is below the bottom of the text box
	if *y < y_low
	{
		// Create a new page
		(_, layer_ref) = make_new_page(doc, layer_count, background.clone(), img_transform);
		// Set the y level to the top of this page
		*y = y_high;
	}
	// Create a new text section on the page
	layer_ref.begin_text_section();
	// Set the text cursor on the page
	layer_ref.set_text_cursor(Mm(*x), Mm(*y));
	// Set the font and font size
	layer_ref.set_font(font, font_size as f64);
	// Set the text color
	layer_ref.set_fill_color(color.clone());
	// Write the text to the page
	layer_ref.write_text(text, &font);
	// End the text section on the page
	layer_ref.end_text_section();
	// Return the most recent page
	layer_ref
}

// Writes left-aligned text into a fixed size text box
// Returns the last layer of the last page that the text appeared on
// Otherwise it returns the layer of the current page
fn write_textbox(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, text: &str, color: &Color, font_size: f32, x_left: f64,
x_right: f64, y_high: f64, y_low: f64, x: &mut f64, y: &mut f64, font_tag: &str, font: &IndirectFontRef,
font_size_data: &Font, font_scale: &Scale, tab_amount: f64, newline_amount: f64) -> PdfLayerReference
{
	// If either dimensions of the text box overlap each other, do nothing
	if x_left >= x_right || y_high <= y_low { return (*layer).clone(); }
	// If the x position starts past the right side of the text box, reset it to the left side plus the tab amount
	if *x > x_right { *x = x_left + tab_amount; }
	// The layer that will get returned
	let mut layer_ref = (*layer).clone();
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
		// Sets the tab adjuster to not be 0 anymore after the first paragraph
		tab_adjuster = tab_amount;
		// Get a vec of each token in the paragraph
		let tokens: Vec<_> = paragraph.split_whitespace().collect();
		// If there are no tokens in this paragraph, skip it
		if tokens.len() < 1 { continue; }
		// Set the current line to the first token in the paragraph
		let mut line = tokens[0].to_string();
		// Loop through each token after the first
		for token in &tokens[1..]
		{
			// Create a hypothetical new line with the next token
			let new_line = format!("{} {}", line, token);
			// Calculate the width of this new line
			let new_line_end = *x + (calc_text_width(font_size_data, font_scale, &new_line) as f64);
			// If the line would be too wide with the next token
			if new_line_end > x_right
			{
				// Write the current line to the page
				layer_ref = apply_textbox_line(doc, &layer_ref, layer_count, background.clone(), img_transform, &line, color, font_size,
					x_left, x_right, y_high, y_low, x, y, font, newline_adjuster);
				// Set the newline adjuster to the newline amount so it's not 0 after the first line
				newline_adjuster = newline_amount;
				// Set the x position back to the left side of the text box to undo tabbing on the first line of a new paragraph
				*x = x_left;
				// Set the current line to the next token
				line = token.to_string();
			}
			// If the new line fits within the text box, add the next token to the current line
			else { line = new_line; }
		}
		// Write all remaining text to the page
		layer_ref = apply_textbox_line(doc, &layer_ref, layer_count, background.clone(), img_transform, &line, color, font_size,
			x_left, x_right, y_high, y_low, x, y, font, newline_adjuster);
		// Set the newline adjuster to the newline amount so it's not 0 after the first line
		newline_adjuster = newline_amount;
		// Calculate where the end of the last line that was written is and save it
		last_x = *x + (calc_text_width(font_size_data, font_scale, &line) as f64);
	}
	// Set the x position to the end of the last line that was written
	*x = last_x;
	// Return the last layer that the text appeared on
	layer_ref
}

/*fn write_table(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, table: &Vec<Vec<Vec<String>>>, color: &Color, font_size: f32,
x: &mut f64, y: &mut f64, body_font: &IndirectFontRef, header_font: &IndirectFontRef, body_font_size_data: &Font,
header_font_size_data: &Font, font_scale: &Scale, newline_amount: f64) -> PdfLayerReference
{
	for row in table
	{
		for cell in row
		{

		}
	}
}*/

// Writes a table to the pdf doc
fn create_table(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, table_string: &str, color: &Color, font_size: f32,
x: &mut f64, y: &mut f64, body_font: &IndirectFontRef, header_font: &IndirectFontRef, body_font_size_data: &Font,
header_font_size_data: &Font, font_scale: &Scale, newline_amount: f64) -> PdfLayerReference
{
	// The layer that gets returned
	let mut layer_ref = (*layer).clone();
	// Split <table> tokens off of the ends of the table string
	let mut tokens: Vec<_> = table_string.split_whitespace().collect();
	if tokens.len() < 2 { return layer_ref; }
	tokens.remove(0);
	tokens.pop();
	// Get a vec of every row in the table (separated by the ROW delimeter)
	let new_table_string = tokens.join(" ");
	let rows: Vec<_> = new_table_string.split(ROW_TAG).collect();
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
	println!("{:?}", table_vec);
	// Get the width of the widest string in each column
	let max_column_widths = get_max_column_widths(&table_vec, column_count, body_font_size_data, header_font_size_data,
		font_scale);
	println!("{:?}", max_column_widths);
	// Create vec for holding the actual width of each column
	// This will be determined by first assuming all columns need the same amount of space on a page,
	// then if any columns have a max width less than that, remove the space that column doesn't need
	// and split it up among the rest of the columns
	let mut column_widths = vec![0.0; column_count];
	// Sort the max width of each column from least to greatest
	let mut sorted_max_widths = max_column_widths.clone();
	sorted_max_widths.sort_by(|(_, a), (_, b)| a.partial_cmp(&b).expect("Error sorting table widths"));
	println!("{:?}", sorted_max_widths);
	// Get the width of the entire table
	let mut table_width = X_END - X_START;
	// Space between cells in the table
	let cell_margin = 10.0;
	// Calculate the default column width
	let mut default_column_width = (table_width - cell_margin * ((column_count as f64) - 1.0)) / column_count as f64;
	println!("{}", default_column_width);
	// Keeps track of the number of reamining columns to calculate width for
	let mut remaining_columns = column_count as f64 - 1.0;
	// Loop through each max column width in order of least to greatest
	for (index, max_width) in sorted_max_widths
	{
		// If the column's max width is less than the default column width
		if (max_width as f64) < default_column_width
		{
			// Set that column's width to it's max width
			column_widths[index] = max_width as f64;
			// Adjust the default column width to take up the space this column didn't use
			default_column_width += (default_column_width - max_width as f64) / remaining_columns;
			// Decrease the number of columns left to calculate the width of
			remaining_columns -= 1.0;
		}
		else
		{
			// Set this column's width to the default width
			column_widths[index] = default_column_width;
		}
	}
	println!("{:?}", column_widths);
	// Calculate the sum of the widths of each column
	let actual_table_width: f64 = column_widths.iter().sum::<f64>() + cell_margin * ((column_count as f64) - 1.0);
	println!("{}", actual_table_width);
	// Make the table width smaller if the columns aren't going to take up the whole page
	table_width = table_width.min(actual_table_width);
	println!("{}", table_width);
	// Create a new 3D table vec for storing the rows, columns, and each line in a cell
	let mut table: Vec<Vec<Vec<String>>> = Vec::new();
	// Used for storing the height of each row in a table
	let mut row_heights: Vec<f32> = Vec::new();
	// Flag to tell if header text is currently being processed
	let mut header = true;
	// Loops through each row in the table
	for row in table_vec
	{
		// Add a new row to the final table vec
		table.push(Vec::new());
		// Get the index of the row that was just added
		let last_row = table.len() - 1;
		// Create a new height for the current row
		row_heights.push(0.0);
		// Counts which column is currently being processed so the correct width in the column_widths vec can be accessed
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
			// Create a string that will represent an entire line of text in this cell
			let mut line = tokens[0].to_string();
			// Loop through each token after the first
			for token in &tokens[1..]
			{
				// Create a string of a line with the next token added
				let new_line = format!("{} {}", line, token);
				// Calculate the width of this new line (with header or body font)
				let width = if header { calc_text_width(header_font_size_data, font_scale, &new_line) }
				else { calc_text_width(body_font_size_data, font_scale, &new_line) };
				// If the line with this token added is too wide for this column
				if width as f64 > column_widths[column]
				{
					// Add the current line
					table[last_row][last_col].push(line);
					// Reset the line to the current token
					line = token.to_string();
				}
				// If the new line isn't too wide, add the current token to the current line
				else { line = new_line; }
			}
			// Add the remaining text to the table
			table[last_row][last_col].push(line);
			// Calculate the height of this cell
			let cell_height = if header { calc_text_height(header_font_size_data, font_scale, font_size, newline_amount, table[last_row][last_col].len()) }
			else { calc_text_height(header_font_size_data, font_scale, font_size, newline_amount, table[last_row][last_col].len()) };
			// Replace the total height for this row if this cell's height is larger than the previous amount
			row_heights[last_row] = row_heights[last_row].max(cell_height);
			// Go to the next column_width index
			column += 1;
		}
		// Set the header font flag to false after the first row has been completed
		header = false;
	}
	println!("{:?}", table);
	println!("{:?}", row_heights);
	// Calculate the height of the entire table
	let table_height = (row_heights.iter().sum::<f32>() as f64) + (((row_heights.len() - 2) as f64) * cell_margin);
	println!("{}", table_height);
	// If the table goes off the current page but isn't longer than a whole page
	/*if *y - table_height < Y_END && table_height <= Y_START - Y_END
	{
		// End the current text section
		layer_ref.end_text_section();
		// Create a new page
		(_, layer_ref) = make_new_page(doc, layer_count, background.clone(), img_transform);
		// Create a new text section
		layer_ref.begin_text_section();
		// Set the cursor to the top of the page
		*y = Y_START;
		layer_ref.set_text_cursor(Mm(X_START), Mm(*y));
		// Reset the text color
		layer_ref.set_fill_color(color.clone());
	}*/
	// If the table is longer than a whole page, just start writing it
	// Else if the table still goes off the current page but isn't longer than a whole page, make a new page and start writing it on there
	// Else, just start writing the table
	// Return the last layer that was used
	layer_ref
}

// Checks when to start and stop table processing in safe_write()
// Returns true if a table is currently being processed or just finished processing, false otherwise
fn check_in_table(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, table_string: &mut String, token: &str,
in_table: &mut bool, color: &Color, font_size: f32, x: &mut f64, y: &mut f64, regular_font: &IndirectFontRef, bold_font: &IndirectFontRef,
regular_font_size_data: &Font, bold_font_size_data: &Font, font_scale: &Scale, newline_amount: f64)
-> (bool, PdfLayerReference)
{
	// If currently in a table
	if *in_table
	{
		// Add the current token to the table string
		*table_string = format!("{} {}", table_string, token);
		// If the token is the table start / end token
		if token == TABLE_TAG
		{
			// Write the table to the pdf doc
			let new_layer = create_table(doc, layer, layer_count, background.clone(), img_transform, table_string, color,
				font_size, x, y, regular_font, bold_font, regular_font_size_data, bold_font_size_data, font_scale,
				newline_amount);
			// Set the in_table flag off
			*in_table = false;
			return (true, new_layer);
		}
		return (true, layer.clone());
	}
	// If not currently in a table and the token is the table start / end token
	else if token == TABLE_TAG
	{
		// Set the in_table flag to true
		*in_table = true;
		// Initialize the table_string to this token
		*table_string = token.to_string();
		return (true, layer.clone());
	}
	(false, layer.clone())
}

// Creates a new page and returns the layer for it
fn make_new_page(doc: &PdfDocumentReference, layer_count: &mut i32, background: image::DynamicImage,
img_transform: &ImageTransform) -> (PdfPageIndex, PdfLayerReference)
{
	// Create a new image since cloning the old one isn't allowed for some reason
	let img = Image::from_dynamic_image(&background.clone());
	// Create a new page
	let (page, layer) = doc.add_page(Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), format!("Layer {}", layer_count));
	// Increment the layer / page count
	*layer_count += 1;
	// Get the new layer
	let layer_ref = doc.get_page(page).get_layer(layer);
	// Add the background image to the page
	img.add_to_layer(layer_ref.clone(), *img_transform);
	(page, layer_ref)
}

// Checks if a new page needs to be made (text too low on current page)
// Returns layer of new page if a new one is created
fn check_new_page(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, color: &Color, font_size: f32, x: f64,
y: &mut f64, font: &IndirectFontRef) -> PdfLayerReference
{
	if *y < Y_END
	{
		// End the current text section
		layer.end_text_section();
		// Create a new page
		let (_, new_layer) = make_new_page(doc, layer_count, background.clone(), img_transform);
		// Create a new text section
		new_layer.begin_text_section();
		// Set the cursor to the top of the page
		*y = Y_START;
		new_layer.set_text_cursor(Mm(x), Mm(*y));
		// Reset the font
		new_layer.set_font(font, font_size as f64);
		// Reset the text color
		new_layer.set_fill_color(color.clone());
		new_layer
	}
	else { layer.clone() }
}

// Checks if a token is a font switch token, switches the current font to that font
// Returns true if the font was switched
fn font_switch<'a>(current_font: &'a IndirectFontRef, current_font_size_data: &'a Font, token: &'a str,
regular_font: &'a IndirectFontRef, bold_font: &'a IndirectFontRef, italic_font: &'a IndirectFontRef,
bold_italic_font: &'a IndirectFontRef, regular_font_size_data: &'a Font, bold_font_size_data: &'a Font,
italic_font_size_data: &'a Font, bold_italic_font_size_data: &'a Font) -> (bool, &'a IndirectFontRef, &'a Font<'a>)
{
	match token
	{
		// Regular font
		"<r>" => (true, regular_font, regular_font_size_data),
		// Bold font
		"<b>" => (true, bold_font, bold_font_size_data),
		// Italic font
		"<i>" => (true, italic_font, italic_font_size_data),
		// Bold italic font
		"<bi>" | "<ib>" => (true, bold_italic_font, bold_italic_font_size_data),
		// Anything else
		_ => (false, current_font, current_font_size_data)
	}
}

fn apply_text<'a>(doc: &'a PdfDocumentReference, layer: &'a PdfLayerReference, layer_count: &'a mut i32,
background: image::DynamicImage, img_transform: &'a ImageTransform, text: &'a str, color: &'a Color, font_size: f32,
x: f64, y: &'a mut f64, font: &'a IndirectFontRef, font_size_data: &'a Font, font_scale: &'a Scale, newline_amount: f64)
-> PdfLayerReference
{
	// Write the line without the current token
	layer.write_text(text, &font);
	// Begin a new text section
	layer.end_text_section();
	layer.begin_text_section();
	// Move the cursor down a line
	*y -= newline_amount;
	layer.set_text_cursor(Mm(x), Mm(*y));
	// Creates a new page if one needs to be created
	check_new_page(doc, &layer, layer_count, background.clone(), img_transform, color, font_size, X_START, y, font)
}

// Writes text onto multiple lines / pages so it doesn't go off the side or bottom of the page
fn safe_write(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, text: &str, color: &Color, font_size: f32, x: &mut f64,
y: &mut f64, regular_font: &IndirectFontRef, bold_font: &IndirectFontRef, italic_font: &IndirectFontRef,
bold_italic_font: &IndirectFontRef, regular_font_size_data: &Font, bold_font_size_data: &Font,
italic_font_size_data: &Font, bold_italic_font_size_data: &Font, font_scale: &Scale, newline_amount: f64,
x_start_offset: f64) -> PdfLayerReference
{
	// Set the current font being used to the regular font
	let mut current_font = regular_font;
	let mut current_font_size_data = regular_font_size_data;
	// The layer that gets returned
	let mut layer_ref = (*layer).clone();
	// Creates a new page if one needs to be created
	layer_ref = check_new_page(doc, &layer_ref, layer_count, background.clone(), img_transform, color,
		font_size, *x, y, current_font);
	// Number of millimeters to shift the text over by at the start of a new paragraph
	let mut x_offset: f64 = x_start_offset;
	// Split the text into paragraphs by newlines
	let paragraphs = text.split('\n');
	// Flag to make sure the cursor doesn't get reset on the first paragraph
	let mut set_cursor = false;
	// Flag to tell if a table is currently bring processed or not
	let mut in_table = false;
	// String for passing tokens onto the create_table() function
	let mut table_string: String = String::new();
	// Loop through each paragraph
	for paragraph in paragraphs
	{
		// Only reset the cursor if it's not the first paragraph
		if set_cursor { layer_ref.set_text_cursor(Mm(X_START + x_offset), Mm(*y)); }
		else { set_cursor = true; }

		// Split the paragraph into tokens by whitespace
		let tokens = paragraph.split_whitespace();
		let mut tokens_vec = tokens.collect::<Vec<&str>>();

		// Loop until the tokens_vec is empty
		while tokens_vec.len() > 0
		{
			let mut font_switched = false;
			// Switch the font if the first token in tokens_vec is a font switch token
			(font_switched, current_font, current_font_size_data) = font_switch(current_font, current_font_size_data,
				&tokens_vec[0], regular_font, bold_font, italic_font, bold_italic_font, regular_font_size_data,
				bold_font_size_data, italic_font_size_data, bold_italic_font_size_data);
			// If the current font wasn't switched
			if !font_switched
			{
				// Exit the loop
				break;
			}
			// If the first token is a font switch token
			else
			{
				// Switch the font
				layer_ref.set_font(current_font, font_size as f64);
				// Remove the font token
				tokens_vec.remove(0);
			}
		}

		// If there are no tokens, skip to next paragraph
		if tokens_vec.len() < 1 { continue }
		// Create a string that will become a line to add to the page made up of tokens
		let mut line = tokens_vec[0].to_string();

		// Check if a table is currently being processed
		let mut skip = false;
		(skip, layer_ref) = check_in_table(doc, &layer_ref, layer_count, background.clone(), img_transform,
			&mut table_string, &paragraph, &mut in_table, color, font_size, x, y, regular_font, bold_font, regular_font_size_data,
			bold_font_size_data, font_scale, newline_amount);
		// If a table is being processed, skip printing this text here
		if skip { continue; }

		// Loop through each token after the first
		for token in &tokens_vec[1..]
		{
			// Flag that tells if the font has been switched or not
			let mut font_switched = false;
			// Keeps track of the last font that was being used before a switch
			let last_font = current_font;
			let last_font_size_data = current_font_size_data;
			// Switch the current font if this token is a font switch token
			(font_switched, current_font, current_font_size_data) = font_switch(current_font, current_font_size_data,
				token, regular_font, bold_font, italic_font, bold_italic_font, regular_font_size_data,
				bold_font_size_data, italic_font_size_data, bold_italic_font_size_data);
			// If the current font was switched
			if font_switched
			{
				// Calculate how wide the line is so far with the font before the switch
				// This makes it so the cursor can be set to the right place in the apply_text call
				x_offset += calc_text_width(last_font_size_data, font_scale, &line) as f64;
				// Apply the text with the previous font
				layer_ref = apply_text(doc, &layer_ref, layer_count, background.clone(), img_transform, &line,
					color, font_size, X_START + x_offset, y, last_font, last_font_size_data, font_scale, 0.0);
				// Set the font to the new one
				layer_ref.set_font(current_font, font_size as f64);
				// Reset the line back to just a space
				line = String::from(" ");
				// If the font that was used was bold italic, add an extra two spaces sicne that font is weird
				if last_font_size_data as *const Font == bold_italic_font_size_data as *const Font { line += "  "; }
				// Go to next token
				continue;
			}

			// Check if a table is currently being processed
			(skip, layer_ref) = check_in_table(doc, &layer_ref, layer_count, background.clone(), img_transform,
				&mut table_string, &token, &mut in_table, color, font_size, x, y, regular_font, bold_font, regular_font_size_data,
				bold_font_size_data, font_scale, newline_amount);
			// If a table is being processed, skip printing this text here
			if skip { continue; }

			// Create a new line to test if the current line is long enough for a newline
			let new_line = format!("{} {}", line, token);
			// Calculate the width of the line with this token added
			let width = calc_text_width(&current_font_size_data, &font_scale, &new_line);
			// If the line is too long with this token added
			if width as f64 > X_END - x_offset
			{
				layer_ref = apply_text(doc, &layer_ref, layer_count, background.clone(), img_transform, &line,
					color, font_size, X_START, y, current_font, current_font_size_data, font_scale, newline_amount);
				// Reset the x_offset to 0 in case the last line already used it
				x_offset = 0.0;
				// Reset the line to the current token
				line = token.to_string();
			}
			// If this line still isn't long enough, add the current token to the line
			else { line = new_line; }
		}
		// Calculate the width of the last line
		let width = calc_text_width(&current_font_size_data, &font_scale, &line);
		// Set x to the width to it keeps track of where the text left off horitontally
		*x = width as f64;
		// Write any remaining text into its own line
		layer_ref.write_text(line, &current_font);
		// End the text section of this paragraph
		layer_ref.end_text_section();
		layer_ref.begin_text_section();
		// Set the x offset to 10 mm so the first paragraph doesn't have any offset but all following ones do
		x_offset = 10.0;
		// Move the cursor down a line
		*y -= newline_amount;
	}
	// Move the cursor back up to be level with the last line of text so the text can be moved down a custom amount
	*y += newline_amount;
	// Return the current layer (will be different if the text spilled into a new page)
	layer_ref
}

// Adds title text for the cover page
fn add_title_text(layer: &PdfLayerReference, text: &str, color: &Color, font_size: f32, font: &IndirectFontRef,
font_size_data: &Font, font_scale: &Scale, newline_amount: f64)
{
	// Set the text color
	layer.set_fill_color(color.clone());
	// Begins a text section
	layer.begin_text_section();
	// Sets the font and cursor location
	layer.set_font(font, font_size as f64);
	// Split the text into tokens by whitespace
	let mut tokens = text.split_whitespace();
	let mut tokens_vec = tokens.collect::<Vec<&str>>();
	// If there are no tokens
	if tokens_vec.len() < 1
	{
		// Add the text "Spellbook" to the title text
		tokens = "Spellbook".split_whitespace();
		tokens_vec = tokens.collect::<Vec<&str>>();
	}
	// Creates a vec of the lines that the title text will be put into
	let mut lines = Vec::<String>::new();
	// Create a string that will become a line to add to the page made up of tokens
	let mut line = tokens_vec[0].to_string();
	// Loop through each token after the first
	for token in &tokens_vec[1..]
	{
		// Create a new line to test if the current line is long enough for a newline
		let new_line = format!("{} {}", line, token);
		// Calculate the width of the line with this token added
		let width = calc_text_width(&font_size_data, &font_scale, &new_line);
		// If the line is too long with this token added
		if width as f64 > X_END - X_START
		{
			// Add the line without this token to the lines vec
			lines.push(line);
			// Reset the current line to just the current token
			line = token.to_string();
		}
		// If this line still isn't long enough, add the current token to the line
		else { line = new_line; }
	}
	// Add the last line to the lines vec
	lines.push(line);
	// Calculate the maximum number of lines that can fit on the cover page
	let max_lines = (PAGE_HEIGHT / newline_amount).floor() as usize;
	// Only use the first max_lines number of lines if there are too many to fit on the page
	lines.truncate(max_lines);
	// Calculate where to start printing the lines based on the number of lines and the height of the lines
	let mut y = (PAGE_HEIGHT / 2.0) + (lines.len() - 1) as f64 / 2.0 * newline_amount;
	// Loop through each line in the title text
	for l in lines
	{
		// Calculate the width of text without the new token added
		let width = calc_text_width(&font_size_data, &font_scale, &l);
		// Set the cursor so the text gets put in the middle of the page
		layer.set_text_cursor(Mm((PAGE_WIDTH / 2.0) - (width as f64 / 2.0)), Mm(y));
		// Write the line without the current token
		layer.write_text(l, &font);
		// Begin a new text section
		layer.end_text_section();
		layer.begin_text_section();
		// Move the cursor down a line
		y -= newline_amount;
	}
	// End this text section
	layer.end_text_section();
}

// Adds text to a spell page
fn add_spell_text(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, text: &str, color: &Color, font_size: f32, x: f64,
y: &mut f64, regular_font: &IndirectFontRef, bold_font: &IndirectFontRef, italic_font: &IndirectFontRef,
bold_italic_font: &IndirectFontRef, regular_font_size_data: &Font, bold_font_size_data: &Font,
italic_font_size_data: &Font, bold_italic_font_size_data: &Font, font_scale: &Scale, newline_amount: f64,
ending_newline: f64, x_start_offset: f64) -> PdfLayerReference
{
	// Set the text color
	layer.set_fill_color(color.clone());
	// Begins a text section
	layer.begin_text_section();
	// Sets the font and cursor location
	layer.set_font(regular_font, font_size as f64);
	layer.set_text_cursor(Mm(x + x_start_offset), Mm(*y));
	let mut temp_x = x;
	// Add spell text to the page
	let mut new_layer = safe_write(doc, &layer, layer_count, background.clone(), img_transform, &text, color, font_size,
		&mut temp_x, y, &regular_font, &bold_font, &italic_font, &bold_italic_font, &regular_font_size_data,
		&bold_font_size_data, &italic_font_size_data, &bold_italic_font_size_data, &font_scale, newline_amount,
		x_start_offset);
	// Ends the text section
	new_layer.end_text_section();
	// Decrement y value by the ending newine amount
	*y -= ending_newline;
	// If the y value is too low
	if *y < Y_END
	{
		// Create a new page
		(_, new_layer) = make_new_page(doc, layer_count, background.clone(), img_transform);
	}
	// Return the current layer (will be different if the text spilled into a new page)
	new_layer
}

fn font_change_wrapup(text: &mut String, x: &mut f64, y: &mut f64, x_left: f64, font_tag: &str, font_size_data: &Font,
font_scale: &Scale, tab_amount: f64, newline_amount: f64)
{
	if (*text).ends_with("\n")
	{
		*y -= newline_amount;
		*x = x_left + tab_amount;
	}
	else
	{
		let space_width = new_calc_text_width(" ", font_tag, font_size_data, font_scale);
		*x += space_width as f64;
	}
	*text = String::new();
}

fn write_spell_description(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, text: &str, color: &Color, font_size: f32, x_left: f64,
x_right: f64, y_high: f64, y_low: f64, x: &mut f64, y: &mut f64, regular_font: &IndirectFontRef,
bold_font: &IndirectFontRef, italic_font: &IndirectFontRef, bold_italic_font: &IndirectFontRef,
regular_font_size_data: &Font, bold_font_size_data: &Font, italic_font_size_data: &Font,
bold_italic_font_size_data: &Font, font_scale: &Scale, tab_amount: f64, newline_amount: f64) -> PdfLayerReference
{
	let mut new_layer = (*layer).clone();
	let mut buffer = String::new();
	let mut current_font = regular_font;
	let mut current_font_size_data = regular_font_size_data;
	let mut last_font_tag = REGULAR_FONT_TAG;
	let mut in_table = false;
	let paragraphs = text.split('\n');
	for paragraph in paragraphs
	{
		let tokens = paragraph.split_whitespace();
		for token in tokens
		{
			match token
			{
				REGULAR_FONT_TAG =>
				{
					if current_font != regular_font
					{
						new_layer = write_textbox(doc, layer, layer_count, background.clone(), img_transform, &buffer,
							color, font_size, x_left, x_right, y_high, y_low, x, y, last_font_tag, current_font,
							current_font_size_data, font_scale, tab_amount, newline_amount);
						font_change_wrapup(&mut buffer, x, y, x_left, last_font_tag, current_font_size_data, font_scale,
							tab_amount, newline_amount);
						current_font = regular_font;
						current_font_size_data = regular_font_size_data;
						last_font_tag = token;
					}
				},
				BOLD_FONT_TAG =>
				{
					if current_font != bold_font
					{
						new_layer = write_textbox(doc, layer, layer_count, background.clone(), img_transform, &buffer,
							color, font_size, x_left, x_right, y_high, y_low, x, y, last_font_tag, current_font,
							current_font_size_data, font_scale, tab_amount, newline_amount);
						font_change_wrapup(&mut buffer, x, y, x_left, last_font_tag, current_font_size_data, font_scale,
							tab_amount, newline_amount);
						current_font = bold_font;
						current_font_size_data = bold_font_size_data;
						last_font_tag = token;
					}
				},
				ITALIC_FONT_TAG =>
				{
					if current_font != italic_font
					{
						new_layer = write_textbox(doc, layer, layer_count, background.clone(), img_transform, &buffer,
							color, font_size, x_left, x_right, y_high, y_low, x, y, last_font_tag, current_font,
							current_font_size_data, font_scale, tab_amount, newline_amount);
						font_change_wrapup(&mut buffer, x, y, x_left, last_font_tag, current_font_size_data, font_scale,
							tab_amount, newline_amount);
						current_font = italic_font;
						current_font_size_data = italic_font_size_data;
						last_font_tag = token;
					}
				},
				BOLD_ITALIC_FONT_TAG | ITALIC_BOLD_FONT_TAG =>
				{
					if current_font != bold_italic_font
					{
						new_layer = write_textbox(doc, layer, layer_count, background.clone(), img_transform, &buffer,
							color, font_size, x_left, x_right, y_high, y_low, x, y, last_font_tag, current_font,
							current_font_size_data, font_scale, tab_amount, newline_amount);
						font_change_wrapup(&mut buffer, x, y, x_left, last_font_tag, current_font_size_data, font_scale,
							tab_amount, newline_amount);
						current_font = bold_italic_font;
						current_font_size_data = bold_italic_font_size_data;
						last_font_tag = token;
					}
				},
				TABLE_TAG =>
				{
					if in_table
					{
						in_table = false;
					}
					else
					{
						in_table = true;
					}
				},
				_ =>
				{
					if buffer.is_empty() { buffer = token.to_string(); }
					else { buffer = format!("{} {}", buffer, token); }
				}
			}
		}
		buffer += "\n";
	}
	new_layer = write_textbox(doc, layer, layer_count, background.clone(), img_transform, &buffer, color, font_size,
		x_left, x_right, y_high, y_low, x, y, last_font_tag, current_font, current_font_size_data, font_scale,
		tab_amount, newline_amount);
	new_layer
}

// Writes ones of the fields of a spell (casting time, components, etc.) to a spellbook document
// Returns the last layer of the last page that the text appeared on
fn write_spell_field(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, field_name: &str, field_text: &str,
field_name_color: &Color, field_text_color: &Color, font_size: f32, x_left: f64, x_right: f64, y_high: f64, y_low: f64,
x: &mut f64, y: &mut f64, field_name_font_tag: &str, field_text_font_tag: &str, field_name_font: &IndirectFontRef,
field_text_font: &IndirectFontRef, field_name_font_size_data: &Font, field_text_font_size_data: &Font,
font_scale: &Scale, tab_amount: f64, newline_amount: f64) -> PdfLayerReference
{
	// Write the field name ("Casting Time:", "Components:", etc.) to the document
	let mut new_layer = write_textbox(doc, layer, layer_count, background.clone(), img_transform, field_name,
		field_name_color, font_size, x_left, x_right, y_high, y_low, x, y, field_name_font_tag, field_name_font,
		field_name_font_size_data, font_scale, tab_amount, newline_amount);
	// Shift the x position over by 1 space
	let sideshift = new_calc_text_width(" ", field_name_font_tag, field_name_font_size_data, font_scale);
	*x += sideshift as f64;
	// Write the text for that field to the document
	new_layer = write_textbox(doc, layer, layer_count, background.clone(), img_transform, field_text,
		field_text_color, font_size, x_left, x_right, y_high, y_low, x, y, field_text_font_tag, field_text_font,
		field_text_font_size_data, font_scale, tab_amount, newline_amount);
	// Return the last layer that was created for this text
	new_layer
}

// Gets the school and level info from a spell and turns it into text that says something like "nth-Level School-Type"
fn get_level_school_text(spell: &spells::Spell) -> String
{
	// Gets a string of the level and the school from the spell
	let mut text = match spell.level
	{
		spells::Level::Cantrip => format!("{} {}", spell.school, spell.level),
		_ => format!("{} {}", spell.level, spell.school.to_string().to_lowercase())
	};
	// If the spell is a ritual
	if spell.is_ritual
	{
		// Add that information to the end of the string
		text += " (ritual)";
	}
	// Return the string
	text
}

pub fn generate_spellbook(title: &str, spell_list: &Vec<spells::Spell>)
-> Result<PdfDocumentReference, Box<dyn std::error::Error>>
{
	// Text colors
	let black = Color::Rgb(Rgb
	{
		r: 0.0, g: 0.0, b: 0.0, icc_profile: None
	});
	let red: Color = Color::Rgb(Rgb
	{
		r: 0.45, g: 0.1, b: 0.1, icc_profile: None
	});
	
    // Load custom font

	// File path to font
	const FONT_NAME: &str = "Bookman";
	let font_directory = format!("fonts/{}", FONT_NAME);

	// Read the data from the font files
    let regular_font_data = fs::read(format!("{}/{}-Regular.otf", font_directory.clone(), FONT_NAME))?;
	let italic_font_data = fs::read(format!("{}/{}-Italic.otf", font_directory.clone(), FONT_NAME))?;
	let bold_font_data = fs::read(format!("{}/{}-Bold.otf", font_directory.clone(), FONT_NAME))?;
	let bold_italic_font_data = fs::read(format!("{}/{}-BoldItalic.otf", font_directory.clone(), FONT_NAME))?;

	// Create font size data for each font style
	let regular_font_size_data = Font::try_from_bytes(&regular_font_data as &[u8]).unwrap();
	let italic_font_size_data = Font::try_from_bytes(&italic_font_data as &[u8]).unwrap();
	let bold_font_size_data = Font::try_from_bytes(&bold_font_data as &[u8]).unwrap();
	let bold_italic_font_size_data = Font::try_from_bytes(&bold_italic_font_data as &[u8]).unwrap();

	// Define font sizes
	const TITLE_FONT_SIZE: f32 = 32.0;
	const HEADER_FONT_SIZE: f32 = 24.0;
	const BODY_FONT_SIZE: f32 = 12.0;

	// Create font scale objects for each font size
	let title_font_scale = Scale::uniform(TITLE_FONT_SIZE);
	let header_font_scale = Scale::uniform(HEADER_FONT_SIZE);
	let body_font_scale = Scale::uniform(BODY_FONT_SIZE);

	println!("{}", calc_text_width(&bold_italic_font_size_data, &body_font_scale, " "));
	println!("{}", calc_text_width(&bold_italic_font_size_data, &body_font_scale, "   "));

	// Load background image
	let img_data = image::open("img/parchment.jpg")?;
    let img1 = Image::from_dynamic_image(&img_data.clone());

	// Determine position, size, and rotation of image
	let img_transform = ImageTransform
	{
		translate_x: Some(Mm(0.0)),
		translate_y: Some(Mm(0.0)),
		scale_x: Some(1.95),
		scale_y: Some(2.125),
		..Default::default()
	};

    // Create PDF document
    let (doc, cover_page, cover_layer) = PdfDocument::new(title, Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), "Cover Layer");

    // Add all styles of the custom font to the document
    let regular_font = doc.add_external_font(&*regular_font_data)?;
	let italic_font = doc.add_external_font(&*italic_font_data)?;
	let bold_font = doc.add_external_font(&*bold_font_data)?;
	let bold_italic_font = doc.add_external_font(&*bold_italic_font_data)?;

	// Create bookmark for cover page
	doc.add_bookmark("Cover", cover_page);

    // Get PdfLayerReference from PdfLayerIndex
	let cover_layer_ref = doc.get_page(cover_page).get_layer(cover_layer);

	// Add the background image to the page
	img1.add_to_layer(cover_layer_ref.clone(), img_transform);

	// Counter variable for naming each layer incrementally
	let mut layer_count = 1;

    // Add text using the custom font to the page
	add_title_text(&cover_layer_ref, title, &black, TITLE_FONT_SIZE, &regular_font, &regular_font_size_data,
		&title_font_scale, TITLE_NEWLINE);

	// Add next pages

	// Loop through each spell
	for spell in spell_list
	{
		// Create a new page
		let (page, mut layer_ref) = make_new_page(&doc, &mut layer_count, img_data.clone(), &img_transform);
		// Create a new bookmark for this page
		doc.add_bookmark(spell.name.clone(), page);
		// Keeps track of the current x and y position to place text at
		let mut x: f64 = X_START;
		let mut y: f64 = Y_START;

		// Add text to the page

		// Add the name of the spell as a header
		/*layer_ref = add_spell_text(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, &spell.name,
			&red, HEADER_FONT_SIZE, X_START, &mut y, &regular_font, &bold_font, &italic_font, &bold_italic_font,
			&regular_font_size_data, &bold_font_size_data, &italic_font_size_data, &bold_italic_font_size_data,
			&header_font_scale, HEADER_NEWLINE, HEADER_NEWLINE, 0.0);*/
		layer_ref = write_textbox(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, &spell.name,
			&red, HEADER_FONT_SIZE, X_START, X_END, Y_START, Y_END, &mut x, &mut y, REGULAR_FONT_TAG, &regular_font,
			&regular_font_size_data, &header_font_scale, TAB_AMOUNT, HEADER_NEWLINE);
		y -= HEADER_NEWLINE;
		x = X_START;

		// Add the level and the spell's school of magic
		let text = get_level_school_text(&spell);
		/*layer_ref = add_spell_text(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, &text, &black,
			BODY_FONT_SIZE, X_START, &mut y, &regular_font, &bold_font, &italic_font, &bold_italic_font,
			&regular_font_size_data, &bold_font_size_data, &italic_font_size_data, &bold_italic_font_size_data,
			&body_font_scale, BODY_NEWLINE, HEADER_NEWLINE, 0.0);*/
		layer_ref = write_textbox(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, &text,
			&black, BODY_FONT_SIZE, X_START, X_END, Y_START, Y_END, &mut x, &mut y, ITALIC_FONT_TAG, &italic_font,
			&italic_font_size_data, &body_font_scale, TAB_AMOUNT, BODY_NEWLINE);
		y -= HEADER_NEWLINE;
		x = X_START;

		// Add the casting time of the spell
		/*let text = format!("<b> Casting Time: <r> {}", &spell.casting_time);
		layer_ref = add_spell_text(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, &text, &black,
			BODY_FONT_SIZE, X_START, &mut y, &regular_font, &bold_font, &italic_font, &bold_italic_font,
			&regular_font_size_data, &bold_font_size_data, &italic_font_size_data, &bold_italic_font_size_data,
			&body_font_scale, BODY_NEWLINE, BODY_NEWLINE, 0.0);*/
		layer_ref = write_spell_field(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform,
			"Casting Time:", &spell.casting_time.to_string(), &black, &black, BODY_FONT_SIZE, X_START, X_END, Y_START,
			Y_END, &mut x, &mut y, BOLD_FONT_TAG, REGULAR_FONT_TAG, &bold_font, &regular_font, &bold_font_size_data,
			&regular_font_size_data, &body_font_scale, TAB_AMOUNT, BODY_NEWLINE);
		y -= BODY_NEWLINE;
		x = X_START;


		// Add the range of the spell
		/*let text = format!("<b> Range: <r> {}", spell.range);
		layer_ref = add_spell_text(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, &text, &black,
			BODY_FONT_SIZE, X_START, &mut y, &regular_font, &bold_font, &italic_font, &bold_italic_font,
			&regular_font_size_data, &bold_font_size_data, &italic_font_size_data, &bold_italic_font_size_data,
			&body_font_scale, BODY_NEWLINE, BODY_NEWLINE, 0.0);*/
		layer_ref = write_spell_field(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform,
			"Range:", &spell.range.to_string(), &black, &black, BODY_FONT_SIZE, X_START, X_END, Y_START, Y_END, &mut x,
			&mut y, BOLD_FONT_TAG, REGULAR_FONT_TAG, &bold_font, &regular_font, &bold_font_size_data,
			&regular_font_size_data, &body_font_scale, TAB_AMOUNT, BODY_NEWLINE);
		y -= BODY_NEWLINE;
		x = X_START;

		// Add the components of the spell
		/*let text = format!("<b> Components: <r> {}", spell.get_component_string());
		layer_ref = add_spell_text(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, &text, &black,
			BODY_FONT_SIZE, X_START, &mut y, &regular_font, &bold_font, &italic_font, &bold_italic_font,
			&regular_font_size_data, &bold_font_size_data, &italic_font_size_data, &bold_italic_font_size_data,
			&body_font_scale, BODY_NEWLINE, BODY_NEWLINE, 0.0);*/
		layer_ref = write_spell_field(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform,
			"Components:", &spell.get_component_string(), &black, &black, BODY_FONT_SIZE, X_START, X_END, Y_START, Y_END,
			&mut x, &mut y, BOLD_FONT_TAG, REGULAR_FONT_TAG, &bold_font, &regular_font, &bold_font_size_data,
			&regular_font_size_data, &body_font_scale, TAB_AMOUNT, BODY_NEWLINE);
		y -= BODY_NEWLINE;
		x = X_START;

		// Add the duration of the spell
		/*let text = format!("<b> Duration: <r> {}", spell.duration);
		layer_ref = add_spell_text(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, &text, &black,
			BODY_FONT_SIZE, X_START, &mut y, &regular_font, &bold_font, &italic_font, &bold_italic_font,
			&regular_font_size_data, &bold_font_size_data, &italic_font_size_data, &bold_italic_font_size_data,
			&body_font_scale, BODY_NEWLINE, HEADER_NEWLINE, 0.0);*/
		layer_ref = write_spell_field(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform,
			"Duration:", &spell.duration.to_string(), &black, &black, BODY_FONT_SIZE, X_START, X_END, Y_START, Y_END,
			&mut x, &mut y, BOLD_FONT_TAG, REGULAR_FONT_TAG, &bold_font, &regular_font, &bold_font_size_data,
			&regular_font_size_data, &body_font_scale, TAB_AMOUNT, BODY_NEWLINE);
		y -= HEADER_NEWLINE;
		x = X_START;

		// Add the spell's description
		/*layer_ref = add_spell_text(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform,
			&spell.description, &black, BODY_FONT_SIZE, X_START, &mut y, &regular_font, &bold_font, &italic_font,
			&bold_italic_font, &regular_font_size_data, &bold_font_size_data, &italic_font_size_data,
			&bold_italic_font_size_data, &body_font_scale, BODY_NEWLINE, BODY_NEWLINE, 0.0);*/
		layer_ref = write_spell_description(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform,
			&spell.description, &black, BODY_FONT_SIZE, X_START, X_END, Y_START, Y_END, &mut x, &mut y, &regular_font,
			&bold_font, &italic_font, &bold_italic_font, &regular_font_size_data, &bold_font_size_data,
			&italic_font_size_data, &bold_italic_font_size_data, &body_font_scale, TAB_AMOUNT, BODY_NEWLINE);

		// If the spell has an upcast description
		if let Some(description) = &spell.upcast_description
		{
			y -= BODY_NEWLINE;
			x = X_START + TAB_AMOUNT;
			let text = format!("<bi> At Higher Levels. <r> {}", description);
			/*layer_ref = add_spell_text(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, &text,
				&black, BODY_FONT_SIZE, X_START, &mut y, &regular_font, &bold_font, &italic_font, &bold_italic_font,
				&regular_font_size_data, &bold_font_size_data, &italic_font_size_data, &bold_italic_font_size_data,
				&body_font_scale, BODY_NEWLINE, BODY_NEWLINE, 10.0);*/
			layer_ref = write_spell_description(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform,
				&text, &black, BODY_FONT_SIZE, X_START, X_END, Y_START, Y_END, &mut x, &mut y, &regular_font, &bold_font,
				&italic_font, &bold_italic_font, &regular_font_size_data, &bold_font_size_data, &italic_font_size_data,
				&bold_italic_font_size_data, &body_font_scale, TAB_AMOUNT, BODY_NEWLINE);
		}

		// Increment the layer counter
		layer_count += 1;
	}

	// Return the pdf document
    Ok(doc)
}

// Saves a spellbook to a file
pub fn save_spellbook(doc: PdfDocumentReference, file_name: &str) -> Result<(), Box<dyn std::error::Error>>
{
	let file = fs::File::create(file_name)?;
	doc.save(&mut std::io::BufWriter::new(file))?;
	Ok(())
}

pub fn get_all_phb_spells() -> Result<Vec<spells::Spell>, Box<dyn std::error::Error>>
{
	let phb_path = "spells/phb";
	let file_paths = fs::read_dir(phb_path)?;
	let mut spell_list = Vec::new();
	for file_path in file_paths
	{
		let file_name_option = file_path?.path();
		let file_name = match file_name_option.to_str()
		{
			Some(s) => s,
			None => panic!("Couldn't find file name")
		};
		spell_list.push(spells::Spell::from_file(file_name)?);
	}
	Ok(spell_list)
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn the_test()
	{
		// Create spellbook
		let spellbook_name = "A Wizard's Very Fancy Spellbook Used for Casting Magical Spells";
		let spell_list = get_all_phb_spells().unwrap();
		let doc = generate_spellbook(spellbook_name, &spell_list).unwrap();
		let _ = save_spellbook(doc, "Spellbook.pdf");
	}

	// For creating spellbooks for myself and friends while I work on creating a ui to use this library
	/*#[test]
	fn create_spellbook()
	{
		let mut spell_list = Vec::new();
		let spell_paths = vec!
		[
			"spells/phb/prestidigitation.txt",
			"spells/phb/mending.txt",
			"spells/phb/mage_hand.txt",
			"spells/phb/fire_bolt.txt",
			"spells/strix/silvery_barbs.txt",
			"spells/phb/color_spray.txt",
			"spells/phb/magic_missile.txt",
			"spells/phb/ice_knife.txt",
			"spells/phb/mage_armor.txt",
			"spells/phb/unseen_servant.txt",
			"spells/phb/detect_magic.txt",
			"spells/phb/alarm.txt",
			"spells/phb/cloud_of_daggers.txt",
			"spells/phb/scorching_ray.txt"
		];
		for path in spell_paths
		{
			println!("{}", path);
			spell_list.push(spells::Spell::from_file(path).unwrap());
		}
		let spellbook_name = "A Spellcaster's Spellbook";
		let doc = generate_spellbook(spellbook_name, &spell_list).unwrap();
		let _ = save_spellbook(doc, "New Spellbook.pdf");
	}*/
}
