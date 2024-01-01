extern crate image;
use printpdf::*;
use rusttype::{Font, Scale};
pub mod spells;
pub mod phb_spells;

// width and height of each page in millimeters
const page_width: f64 = 210.0;
const page_height: f64 = 297.0;

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
	width
}

// Calculates the height of a font with a certain font size
fn calc_text_height(font_size_data: &Font, font_scale: &Scale) -> f32
{
	let v_metrics = font_size_data.v_metrics(*font_scale);
	v_metrics.ascent - v_metrics.descent
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

// Adds text to a spell page
fn add_spell_text(layer: &PdfLayerReference, text: &str, font_size: f32, x: f64, y: f64, font: &IndirectFontRef,
font_size_data: &Font, font_scale: &Scale)
{
	// Begins a text section
	layer.begin_text_section();
	// Sets the font and cursor location
	layer.set_font(&font, font_size as f64);
	layer.set_text_cursor(Mm(x), Mm(y));
	// Calculate the width of the text
	let width = calc_text_width(&font_size_data, &font_scale, &text);
	// Add spell text to the page
	layer.write_text(text, &font);
	// Ends the text section
	layer.end_text_section();
}

// Adds spell field text to a spell page
fn add_spell_field(layer: &PdfLayerReference, field: &str, text: &str, font_size: f32, x: f64, y: f64,
field_font: &IndirectFontRef, text_font: &IndirectFontRef, field_font_size_data: &Font, text_font_size_data: &Font,
font_scale: &Scale)
{
	// Begins a text section
	layer.begin_text_section();
	// Sets the font to the field font
	layer.set_font(&field_font, font_size as f64);
	// Sets the cursor location
	layer.set_text_cursor(Mm(x), Mm(y));
	// Calculate the width of the text
	let field_width = calc_text_width(&field_font_size_data, &font_scale, &field);
	let text_width = calc_text_width(&text_font_size_data, &font_scale, &text);
	// Add the field text to the page
	layer.write_text(field, &field_font);
	// Set the font to the text font
	layer.set_font(&text_font, font_size as f64);
	// Add the spell text to the page
	layer.write_text(text, &text_font);
	// Ends the text section
	layer.end_text_section();
}

pub fn generate_spellbook(title: &str, file_name: &str, spell_list: Vec<&spells::Spell>) -> Result<(), Box<dyn std::error::Error>>
{
    // Load custom font

	// File path to font
	const font_name: &str = "Bookman";
	let font_directory = format!("fonts/{}", font_name.clone());

	// Read the data from the font files
    let regular_font_data = std::fs::read(format!("{}/{}-Regular.otf", font_directory.clone(), font_name))?;
	let italic_font_data = std::fs::read(format!("{}/{}-Italic.otf", font_directory.clone(), font_name))?;
	let bold_font_data = std::fs::read(format!("{}/{}-Bold.otf", font_directory.clone(), font_name))?;
	let bold_italic_font_data = std::fs::read(format!("{}/{}-BoldItalic.otf", font_directory.clone(), font_name))?;

	// Create font size data for each font style
	let regular_font_size_data = Font::try_from_bytes(&regular_font_data as &[u8]).unwrap();
	let italic_font_size_data = Font::try_from_bytes(&italic_font_data as &[u8]).unwrap();
	let bold_font_size_data = Font::try_from_bytes(&bold_font_data as &[u8]).unwrap();
	let bold_italic_font_size_data = Font::try_from_bytes(&bold_italic_font_data as &[u8]).unwrap();

	// Define font sizes
	const title_font_size: f32 = 32.0;
	const header_font_size: f32 = 24.0;
	const body_font_size: f32 = 12.0;

	// Create font scale objects for each font size
	let title_font_scale = Scale::uniform(title_font_size);
	let header_font_scale = Scale::uniform(header_font_size);
	let body_font_scale = Scale::uniform(body_font_size);

	// Calculate text heights
	let title_text_height = calc_text_height(&regular_font_size_data, &title_font_scale) as f64;
	let header_text_height = calc_text_height(&regular_font_size_data, &header_font_scale) as f64;
	let level_school_text_height = calc_text_height(&italic_font_size_data, &body_font_scale) as f64;
	let spell_field_text_height = calc_text_height(&bold_font_size_data, &body_font_scale) as f64;
	let body_text_height = calc_text_height(&regular_font_size_data, &body_font_scale) as f64;
	let upcast_text_height = calc_text_height(&bold_italic_font_size_data, &body_font_scale) as f64;

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
    let (doc, cover_page, cover_layer) = PdfDocument::new(title, Mm(page_width), Mm(page_height), "Cover Layer");

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

    // Define text
    let text = "Hello! The quick brown fox jumped over the lazy dog. Peter Piper picked a prickly patch of purple pickle peppers.";

	// Calculate width of text
	let width = calc_text_width(&regular_font_size_data, &title_font_scale, &text);

    // Add text using the custom font to the page
    cover_layer_ref.use_text(format!("{} {}", width, text), title_font_size as f64, Mm(10.0), Mm(200.0), &regular_font);

	// Add next pages
	
	// Starting x and y positions for text on a page
	const x_start: f64 = 10.0;
	const y_start: f64 = 280.0;

	// Text height to mm ratio for scaling vertical text placement
	const height_mm_ratio: f64 = 4.0;

	// Number of millimeters to go downwards for newlines
	const large_newline: f64 = 8.0;
	const small_newline: f64 = 5.0;

	// Counter variable for naming each layer incrementally
	let mut layer_count = 1;

	// Loop through each spell
	for spell in spell_list
	{
		// Initialize the page

		// Create a new image since cloning the old one isn't allowed for some reason
		let img = Image::from_dynamic_image(&img_data.clone());
		// Create a new page
		let (page, layer) = doc.add_page(Mm(page_width), Mm(page_height), format!("Layer {}", layer_count));
		// Create a new bookmark for this page
		doc.add_bookmark(spell.name, page);
		// Get a reference to the layer for this page
		let layer_ref = doc.get_page(page).get_layer(layer);
		// Add the background image to the page
		img.add_to_layer(layer_ref.clone(), img_transform);
		// Keeps track of the current height to place text at
		let mut text_height: f64 = y_start;

		// Add text to the page

		// Add the name of the spell as a header
		add_spell_text(&layer_ref, spell.name, header_font_size, x_start, y_start, &regular_font, &regular_font_size_data,
			&header_font_scale);
		text_height -= large_newline;

		// Add the level and the spell's school of magic
		let text = get_level_school_text(spell);
		add_spell_text(&layer_ref, &text, body_font_size, x_start, text_height, &italic_font,
			&italic_font_size_data, &body_font_scale);
		text_height -= large_newline;

		// Add the casting time of the spell
		add_spell_field(&layer_ref, "Casting Time: ", &spell.casting_time.to_string(), body_font_size, x_start, text_height, &bold_font, &regular_font,
			&bold_font_size_data, &regular_font_size_data, &body_font_scale);
		text_height -= small_newline;

		// Add the range of the spell
		add_spell_field(&layer_ref, "Range: ", &spell.range.to_string(), body_font_size, x_start, text_height, &bold_font, &regular_font,
			&bold_font_size_data, &regular_font_size_data, &body_font_scale);
		text_height -= small_newline;

		// Add the components of the spell
		add_spell_field(&layer_ref, "Components: ", &spell.get_component_string(), body_font_size, x_start, text_height, &bold_font, &regular_font,
			&bold_font_size_data, &regular_font_size_data, &body_font_scale);
		text_height -= small_newline;

		// Add the duration of the spell
		add_spell_field(&layer_ref, "Duration: ", &spell.duration.to_string(), body_font_size, x_start, text_height, &bold_font, &regular_font,
			&bold_font_size_data, &regular_font_size_data, &body_font_scale);
		text_height -= large_newline;

		// Add the spell's description
		add_spell_text(&layer_ref, spell.description, body_font_size, x_start, text_height, &regular_font, &regular_font_size_data,
			&body_font_scale);
		text_height -= small_newline;

		// If the spell has an upcast description
		if let Some(description) = spell.upcast_description
		{
			add_spell_field(&layer_ref, "At Higher Levels. ", description, body_font_size, x_start, text_height, &bold_italic_font, &regular_font,
			&bold_italic_font_size_data, &regular_font_size_data, &body_font_scale);
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
