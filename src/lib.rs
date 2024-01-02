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

// Text colors
const BLACK: Color = Color::Rgb(Rgb
{
	r: 0.0, g: 0.0, b: 0.0, icc_profile: None
});
const RED: Color = Color::Rgb(Rgb
{
	r: 69.0, g: 13.0, b: 13.0, icc_profile: None
});

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

// Calculates the height of a font with a certain font size
/*fn calc_text_height(font_size_data: &Font, font_scale: &Scale) -> f32
{
	let v_metrics = font_size_data.v_metrics(*font_scale);
	v_metrics.ascent - v_metrics.descent
}*/

// Converts rusttype font units to printpdf millimeters (Mm)
fn font_units_to_mm(font_unit_width: f32) -> f32
{
	let mm_to_font_ratio = 0.47;
	font_unit_width * mm_to_font_ratio
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

// Writes text onto multiple lines / pages so it doesn't go off the side or bottom of the page
fn safe_write(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, text: &str, x: f64, y: &mut f64, font: &IndirectFontRef,
font_size_data: &Font, font_scale: &Scale, newline_amount: f64, x_start_offset: f64) -> PdfLayerReference
{
	// The layer that gets returned
	let mut layer_ref = (*layer).clone();
	// Number of millimeters to shift the text over by at the start of a new paragraph
	let mut x_offset: f64 = x_start_offset;
	// Split the text into paragraphs by newlines
	let paragraphs = text.split('\n');
	// Flag to make sure the cursor doesn't get reset on the first paragraph
	let mut set_cursor = false;
	// Loop through each paragraph
	for paragraph in paragraphs
	{
		// Only reset the cursor if it's not the first paragraph
		if set_cursor { layer_ref.set_text_cursor(Mm(x + x_offset), Mm(*y)); }
		else { set_cursor = true; }

		// Split the paragraph into tokens by whitespace
		let tokens = paragraph.split_whitespace();
		let tokens_vec = tokens.collect::<Vec<&str>>();
		// If there are no tokens, skip to next paragraph
		if tokens_vec.len() < 1 { continue }
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
			if width as f64 > X_END - x_offset
			{
				// Write the line without the current token
				layer_ref.write_text(line, &font);
				// Begin a new text section
				layer_ref.end_text_section();
				layer_ref.begin_text_section();
				// Move the cursor down a line
				*y -= newline_amount;
				layer_ref.set_text_cursor(Mm(x), Mm(*y));
				// If the cursor is too low
				if *y < Y_END
				{
					// End the current text section
					layer_ref.end_text_section();
					// Create a new page
					(_, layer_ref) = make_new_page(doc, layer_count, background.clone(), img_transform);
					// Create a new text section
					layer_ref.begin_text_section();
					// Set the cursor to the top of the page
					*y = Y_START;
					layer_ref.set_text_cursor(Mm(x), Mm(*y));
				}
				// Reset the x_offset to 0 in case the last line already used it
				x_offset = 0.0;
				// Reset the line to the current token
				line = token.to_string();
			}
			// If this line still isn't long enough, add the current token to the line
			else { line = new_line; }
		}
		// Write any remaining text into its own line
		layer_ref.write_text(line, &font);
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

// Adds text to a spell page
fn add_spell_text(doc: &PdfDocumentReference, layer: &PdfLayerReference, layer_count: &mut i32,
background: image::DynamicImage, img_transform: &ImageTransform, text: &str, color: Color, font_size: f32, x: f64,
y: &mut f64, font: &IndirectFontRef, font_size_data: &Font, font_scale: &Scale, newline_amount: f64, ending_newline: f64)
-> PdfLayerReference
{
	// Set the text color
	layer.set_fill_color(color);
	// Begins a text section
	layer.begin_text_section();
	// Sets the font and cursor location
	layer.set_font(&font, font_size as f64);
	layer.set_text_cursor(Mm(x), Mm(*y));
	// Add spell text to the page
	let mut new_layer = safe_write(doc, &layer, layer_count, background.clone(), img_transform, &text, x, y, &font, &font_size_data,
		&font_scale, newline_amount, 0.0);
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
background: image::DynamicImage, img_transform: &ImageTransform, field: &str, text: &str, color: Color, font_size: f32,
x: f64, y: &mut f64, field_font: &IndirectFontRef, text_font: &IndirectFontRef, field_font_size_data: &Font,
text_font_size_data: &Font, font_scale: &Scale, newline_amount: f64, ending_newline: f64, x_start_offset: f64)
-> PdfLayerReference
{
	// Set the text color
	layer.set_fill_color(color);
	// Begins a text section
	layer.begin_text_section();
	// Sets the font to the field font
	layer.set_font(&field_font, font_size as f64);
	// Sets the cursor location
	layer.set_text_cursor(Mm(x + x_start_offset), Mm(*y));
	// Add the field text to the page
	layer.write_text(field, &field_font);
	// Calculate the width of the field text
	let field_width = calc_text_width(field_font_size_data, font_scale, field);
	// Set the font to the text font
	layer.set_font(&text_font, font_size as f64);
	// Add the spell text to the page
	let mut new_layer = safe_write(doc, &layer, layer_count, background.clone(), img_transform, &text, x, y, &text_font,
		&text_font_size_data, &font_scale, newline_amount, field_width as f64 + x_start_offset);
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
	match spell.level
	{
		spells::Level::Cantrip => format!("{} {}", spell.school, spell.level),
		_ => format!("{} {}", spell.level, spell.school)
	}
}

pub fn generate_spellbook(title: &str, file_name: &str, spell_list: Vec<&spells::Spell>)
-> Result<(), Box<dyn std::error::Error>>
{
    // Load custom font

	// File path to font
	const FONT_NAME: &str = "Bookman";
	let font_directory = format!("fonts/{}", FONT_NAME);

	// Read the data from the font files
    let regular_font_data = std::fs::read(format!("{}/{}-Regular.otf", font_directory.clone(), FONT_NAME))?;
	let italic_font_data = std::fs::read(format!("{}/{}-Italic.otf", font_directory.clone(), FONT_NAME))?;
	let bold_font_data = std::fs::read(format!("{}/{}-Bold.otf", font_directory.clone(), FONT_NAME))?;
	let bold_italic_font_data = std::fs::read(format!("{}/{}-BoldItalic.otf", font_directory.clone(), FONT_NAME))?;

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

	// Calculate text heights
	/*let title_text_height = calc_text_height(&regular_font_size_data, &title_font_scale) as f64;
	let header_text_height = calc_text_height(&regular_font_size_data, &header_font_scale) as f64;
	let level_school_text_height = calc_text_height(&italic_font_size_data, &body_font_scale) as f64;
	let spell_field_text_height = calc_text_height(&bold_font_size_data, &body_font_scale) as f64;
	let body_text_height = calc_text_height(&regular_font_size_data, &body_font_scale) as f64;
	let upcast_text_height = calc_text_height(&bold_italic_font_size_data, &body_font_scale) as f64;*/

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

    // Define text
    let text = "Hello! The quick fox jumped over the lazy brown dog. Peter Piper picked a patch of prickly purple pickle peppers.";

    // Add text using the custom font to the page
	let _ = add_spell_text(&doc, &cover_layer_ref, &mut layer_count, img_data.clone(), &img_transform, text, BLACK,
		TITLE_FONT_SIZE, X_START, &mut 200.0, &regular_font, &regular_font_size_data, &title_font_scale, TITLE_NEWLINE,
		0.0);

	// Add next pages

	// Loop through each spell
	for spell in spell_list
	{
		// Initialize the page

		// Create a new image since cloning the old one isn't allowed for some reason
		let img = Image::from_dynamic_image(&img_data.clone());
		// Create a new page
		let (page, mut layer_ref) = make_new_page(&doc, &mut layer_count, img_data.clone(), &img_transform);
		// Create a new bookmark for this page
		doc.add_bookmark(spell.name, page);
		// Keeps track of the current height to place text at
		let mut text_height: f64 = Y_START;

		// Add text to the page

		// Add the name of the spell as a header
		layer_ref = add_spell_text(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, spell.name, RED,
			HEADER_FONT_SIZE, X_START, &mut text_height, &regular_font, &regular_font_size_data, &header_font_scale,
			HEADER_NEWLINE, HEADER_NEWLINE);

		// Add the level and the spell's school of magic
		let text = get_level_school_text(spell);
		layer_ref = add_spell_text(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, &text, BLACK,
			BODY_FONT_SIZE, X_START, &mut text_height, &italic_font, &italic_font_size_data, &body_font_scale,
			BODY_NEWLINE, HEADER_NEWLINE);

		// Add the casting time of the spell
		layer_ref = add_spell_field(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform,
			"Casting Time: ", &spell.casting_time.to_string(), BLACK, BODY_FONT_SIZE, X_START, &mut text_height, &bold_font,
			&regular_font, &bold_font_size_data, &regular_font_size_data, &body_font_scale, BODY_NEWLINE, BODY_NEWLINE,
			0.0);

		// Add the range of the spell
		layer_ref = add_spell_field(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, "Range: ",
			&spell.range.to_string(), BLACK, BODY_FONT_SIZE, X_START, &mut text_height, &bold_font, &regular_font,
			&bold_font_size_data, &regular_font_size_data, &body_font_scale, BODY_NEWLINE, BODY_NEWLINE, 0.0);

		// Add the components of the spell
		layer_ref = add_spell_field(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, "Components: ",
			&spell.get_component_string(), BLACK, BODY_FONT_SIZE, X_START, &mut text_height, &bold_font, &regular_font,
			&bold_font_size_data, &regular_font_size_data, &body_font_scale, BODY_NEWLINE, BODY_NEWLINE, 0.0);

		// Add the duration of the spell
		layer_ref = add_spell_field(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, "Duration: ",
			&spell.duration.to_string(), BLACK, BODY_FONT_SIZE, X_START, &mut text_height, &bold_font, &regular_font,
			&bold_font_size_data, &regular_font_size_data, &body_font_scale, BODY_NEWLINE, HEADER_NEWLINE, 0.0);

		// Add the spell's description
		layer_ref = add_spell_text(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform, spell.description,
			BLACK, BODY_FONT_SIZE, X_START, &mut text_height, &regular_font, &regular_font_size_data, &body_font_scale,
			BODY_NEWLINE, BODY_NEWLINE);

		// If the spell has an upcast description
		if let Some(description) = spell.upcast_description
		{
			layer_ref = add_spell_field(&doc, &layer_ref, &mut layer_count, img_data.clone(), &img_transform,
				"At Higher Levels. ", description, BLACK, BODY_FONT_SIZE, X_START, &mut text_height, &bold_italic_font,
				&regular_font, &bold_italic_font_size_data, &regular_font_size_data, &body_font_scale, BODY_NEWLINE, 0.0,
				10.0);
		}

		// Increment the layer counter
		layer_count += 1;
	}

    // Save the document to a file
    let file = std::fs::File::create(file_name)?;
    doc.save(&mut std::io::BufWriter::new(file))?;

    Ok(())
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn the_test()
	{
		// Create vec of spells for testing
		let spell_list = vec![&phb_spells::fire_bolt, &phb_spells::fireball];
		// Create spellbook
		let _ = generate_spellbook("Spellbook", "Spellbook.pdf", spell_list);
	}
}
