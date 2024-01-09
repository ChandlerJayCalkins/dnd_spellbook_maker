use std::fs;
extern crate image;
use printpdf::*;
use rusttype::{Font, Scale};
pub mod spells;
pub mod phb_spells;

// width and height of each page in millimeters
const PAGE_WIDTH: f64 = 210.0;
const PAGE_HEIGHT: f64 = 297.0;

// Number of millimeters to go downwards for newlines
const TITLE_NEWLINE: f64 = 12.0;
const HEADER_NEWLINE: f64 = 8.0;
const BODY_NEWLINE: f64 = 5.0;

// Starting x and y positions for text on a page
const X_START: f64 = 10.0;
const Y_START: f64 = 280.0;

// Ending x and y positions for text on a page
const X_END: f64 = 190.0;
const Y_END: f64 = 10.0;

const TABLE: &str = "<table>";

// Calculates the width of some text give the font and the font size it uses
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

// Converts rusttype font units to printpdf millimeters (Mm)
fn font_units_to_mm(font_unit_width: f32) -> f32
{
	let mm_to_font_ratio = 0.47;
	font_unit_width * mm_to_font_ratio
}

// Writes a table to the pdf doc
fn create_table(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, table_string: &str, font_size: f32, x: &mut f64,
y: &mut f64, font: &IndirectFontRef, font_size_data: &Font, font_scale: &Scale, newline_amount: f64) -> PdfLayerReference
{
	// The layer that gets returned
	let mut layer_ref = (*layer).clone();
	layer_ref
}

// Checks when to start and stop table processing in safe_write()
// Returns true if a table is currently being processed or just finished processing, false otherwise
fn check_in_table(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, table_string: &mut String, token: &str,
in_table: &mut bool, font_size: f32, x: &mut f64, y: &mut f64, font: &IndirectFontRef, font_size_data: &Font,
font_scale: &Scale, newline_amount: f64) -> (bool, PdfLayerReference)
{
	// If currently in a table
	if *in_table
	{
		// Add the current token to the table string
		*table_string = format!("{} {}", table_string, token);
		// If the token is the table start / end token
		if token == TABLE
		{
			// Write the table to the pdf doc
			//create_table();
			println!("{}", table_string);
			// Set the in_table flag off
			*in_table = false;
		}
		return (true, layer.clone());
	}
	// If not currently in a table and the token is the table start / end token
	else if token == TABLE
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
background: image::DynamicImage, img_transform: &ImageTransform, color: &Color, font_size: f32, x: &mut f64,
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
		new_layer.set_text_cursor(Mm(*x), Mm(*y));
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
		"<r>" =>
		{
			(true, regular_font, regular_font_size_data)
		},
		// Bold font
		"<b>" =>
		{
			(true, bold_font, bold_font_size_data)
		},
		// Italic font
		"<i>" =>
		{
			(true, italic_font, italic_font_size_data)
		},
		// Bold italic font
		"<bi>" | "<ib>" =>
		{
			(true, bold_italic_font, bold_italic_font_size_data)
		},
		_ => (false, current_font, current_font_size_data)
	}
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
		font_size, x, y, current_font);
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
			&mut table_string, &line, &mut in_table, font_size, x, y, current_font, current_font_size_data, font_scale,
			newline_amount);
		// If a table is being processed, skip printing this text here
		if skip { continue; }

		// Loop through each token after the first
		for token in &tokens_vec[1..]
		{
			let mut font_switched = false;
			// Switch the current font if this token is a font switch token
			(font_switched, current_font, current_font_size_data) = font_switch(current_font, current_font_size_data,
				token, regular_font, bold_font, italic_font, bold_italic_font, regular_font_size_data, bold_font_size_data,
				italic_font_size_data, bold_italic_font_size_data);
			// If the current font was switched
			if font_switched
			{
				// Set the font
				layer_ref.set_font(current_font, font_size as f64);
				// Go to next token
				continue;
			}

			// Check if a table is currently being processed
			(skip, layer_ref) = check_in_table(doc, &layer_ref, layer_count, background.clone(), img_transform,
				&mut table_string, &line, &mut in_table, font_size, x, y, current_font, current_font_size_data, font_scale,
				newline_amount);
			// If a table is being processed, skip printing this text here
			if skip { continue; }

			// Create a new line to test if the current line is long enough for a newline
			let new_line = format!("{} {}", line, token);
			// Calculate the width of the line with this token added
			let width = calc_text_width(&current_font_size_data, &font_scale, &new_line);
			// If the line is too long with this token added
			if width as f64 > X_END - x_offset
			{
				// Write the line without the current token
				layer_ref.write_text(line, &current_font);
				// Begin a new text section
				layer_ref.end_text_section();
				layer_ref.begin_text_section();
				// Move the cursor down a line
				*y -= newline_amount;
				layer_ref.set_text_cursor(Mm(X_START), Mm(*y));

				// Creates a new page if one needs to be created
				layer_ref = check_new_page(doc, &layer_ref, layer_count, background.clone(), img_transform, color,
					font_size, x, y, current_font);
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
		if width as f64 > X_END
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
	// Return the current layer (will be different if the text spilled into a new page)
}

// Adds text to a spell page
fn add_spell_text(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, text: &str, color: &Color, font_size: f32, x: f64,
y: &mut f64, regular_font: &IndirectFontRef, bold_font: &IndirectFontRef, italic_font: &IndirectFontRef,
bold_italic_font: &IndirectFontRef, regular_font_size_data: &Font, bold_font_size_data: &Font,
italic_font_size_data: &Font, bold_italic_font_size_data: &Font, font_scale: &Scale, newline_amount: f64,
ending_newline: f64) -> PdfLayerReference
{
	// Set the text color
	layer.set_fill_color(color.clone());
	// Begins a text section
	layer.begin_text_section();
	// Sets the font and cursor location
	layer.set_font(regular_font, font_size as f64);
	layer.set_text_cursor(Mm(x), Mm(*y));
	let mut temp_x = x;
	// Add spell text to the page
	let mut new_layer = safe_write(doc, &layer, layer_count, background.clone(), img_transform, &text, color, font_size,
		&mut temp_x, y, &regular_font, &bold_font, &italic_font, &bold_italic_font, &regular_font_size_data,
		&bold_font_size_data, &italic_font_size_data, &bold_italic_font_size_data, &font_scale, newline_amount, 0.0);
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

// Adds spell field text to a spell page
fn add_spell_field(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, field: &str, text: &str, color: &Color, font_size: f32,
x: f64, y: &mut f64, regular_font: &IndirectFontRef, bold_font: &IndirectFontRef, italic_font: &IndirectFontRef,
bold_italic_font: &IndirectFontRef, regular_font_size_data: &Font, bold_font_size_data: &Font,
italic_font_size_data: &Font, bold_italic_font_size_data: &Font, font_scale: &Scale, newline_amount: f64,
ending_newline: f64, x_start_offset: f64) -> PdfLayerReference
{
	// Set the text color
	layer.set_fill_color(color.clone());
	// Begins a text section
	layer.begin_text_section();
	// Sets the font to the field font
	layer.set_font(bold_font, font_size as f64);
	// Sets the cursor location
	layer.set_text_cursor(Mm(x + x_start_offset), Mm(*y));
	let mut curr_x = x;
	// Add the field text to the page
	layer.write_text(field, &bold_font);
	// Calculate the width of the field text
	let field_width = calc_text_width(bold_font_size_data, font_scale, field);
	// Set the font to the text font
	layer.set_font(regular_font, font_size as f64);
	// Add the spell text to the page
	let mut new_layer = safe_write(doc, &layer, layer_count, background.clone(), img_transform, &text, color, font_size,
		&mut curr_x, y, &regular_font, &bold_font, &italic_font, &bold_italic_font, &regular_font_size_data,
		&bold_font_size_data, &italic_font_size_data, &bold_italic_font_size_data, &font_scale, newline_amount,
		field_width as f64 + x_start_offset);
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

// Gets the school and level info from a spell and turns it into text that says something like "nth-Level School-Type"
fn get_level_school_text(spell: &spells::Spell) -> String
{
	// Gets a string of the level and the school from the spell
	let mut text = String::from("<i>");
	text = match spell.level
	{
		spells::Level::Cantrip => format!("{} {} {}", text, spell.school, spell.level),
		_ => format!("{} {} {}", text, spell.level, spell.school)
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
		// Keeps track of the current height to place text at
		let mut text_height: f64 = Y_START;

		// Add text to the page

		// Add the name of the spell as a header
		layer_ref = add_spell_text(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, &spell.name, &red,
			HEADER_FONT_SIZE, X_START, &mut text_height, &regular_font, &bold_font, &italic_font, &bold_italic_font,
			&regular_font_size_data, &bold_font_size_data, &italic_font_size_data, &bold_italic_font_size_data,
			&header_font_scale, HEADER_NEWLINE, HEADER_NEWLINE);

		// Add the level and the spell's school of magic
		let text = get_level_school_text(&spell);
		layer_ref = add_spell_text(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, &text, &black,
			BODY_FONT_SIZE, X_START, &mut text_height, &regular_font, &bold_font, &italic_font, &bold_italic_font,
			&regular_font_size_data, &bold_font_size_data, &italic_font_size_data, &bold_italic_font_size_data,
			&body_font_scale, BODY_NEWLINE, HEADER_NEWLINE);

		// Add the casting time of the spell
		layer_ref = add_spell_field(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform,
			"Casting Time: ", &spell.casting_time.to_string(), &black, BODY_FONT_SIZE, X_START, &mut text_height,
			&regular_font, &bold_font, &italic_font, &bold_italic_font, &regular_font_size_data, &bold_font_size_data,
			&italic_font_size_data, &bold_italic_font_size_data, &body_font_scale, BODY_NEWLINE, BODY_NEWLINE, 0.0);

		// Add the range of the spell
		layer_ref = add_spell_field(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, "Range: ",
			&spell.range.to_string(), &black, BODY_FONT_SIZE, X_START, &mut text_height, &regular_font, &bold_font,
			&italic_font, &bold_italic_font, &regular_font_size_data, &bold_font_size_data, &italic_font_size_data,
			&bold_italic_font_size_data, &body_font_scale, BODY_NEWLINE, BODY_NEWLINE, 0.0);

		// Add the components of the spell
		layer_ref = add_spell_field(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, "Components: ",
			&spell.get_component_string(), &black, BODY_FONT_SIZE, X_START, &mut text_height, &regular_font, &bold_font,
			&italic_font, &bold_italic_font, &regular_font_size_data, &bold_font_size_data, &italic_font_size_data,
			&bold_italic_font_size_data, &body_font_scale, BODY_NEWLINE, BODY_NEWLINE, 0.0);

		// Add the duration of the spell
		layer_ref = add_spell_field(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, "Duration: ",
			&spell.duration.to_string(), &black, BODY_FONT_SIZE, X_START, &mut text_height, &regular_font, &bold_font,
			&italic_font, &bold_italic_font, &regular_font_size_data, &bold_font_size_data, &italic_font_size_data,
			&bold_italic_font_size_data, &body_font_scale, BODY_NEWLINE, HEADER_NEWLINE, 0.0);

		// Add the spell's description
		layer_ref = add_spell_text(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform,
			&spell.description, &black, BODY_FONT_SIZE, X_START, &mut text_height, &regular_font, &bold_font, &italic_font,
			&bold_italic_font, &regular_font_size_data, &bold_font_size_data, &italic_font_size_data,
			&bold_italic_font_size_data, &body_font_scale, BODY_NEWLINE, BODY_NEWLINE);

		// If the spell has an upcast description
		if let Some(description) = &spell.upcast_description
		{
			layer_ref = add_spell_field(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform,
				"At Higher Levels. ", &description, &black, BODY_FONT_SIZE, X_START, &mut text_height, &regular_font,
				&bold_font, &italic_font, &bold_italic_font, &regular_font_size_data, &bold_font_size_data,
				&italic_font_size_data, &bold_italic_font_size_data, &body_font_scale, BODY_NEWLINE, 0.0, 10.0);
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
}
