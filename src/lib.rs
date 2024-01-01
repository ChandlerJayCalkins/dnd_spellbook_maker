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

fn get_level_school_text(spell: &spells::Spell) -> String
{
	match spell.level
	{
		spells::Level::Cantrip => format!("{} Cantrip", spell.school.to_string()),
		spells::Level::Level1 => format!("1st-Level {}", spell.school.to_string()),
		spells::Level::Level2 => format!("2nd-Level {}", spell.school.to_string()),
		spells::Level::Level3 => format!("3rd-Level {}", spell.school.to_string()),
		spells::Level::Level4 | spells::Level::Level5 | spells::Level::Level6 | spells::Level::Level7 |
		spells::Level::Level8 | spells::Level::Level9 => format!("{}th-Level {}", u8::from(spell.level), spell.school.to_string())
	}
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

	const height_dec1: f64 = 8.0;
	const height_dec2: f64 = 5.0;

	// Counter variable for naming each layer incrementally
	let mut layer_count = 1;

	// Loop through each spell
	for spell in spell_list
	{
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

		// Begins a text section
		layer_ref.begin_text_section();
		// Sets the font, line height, and cursor location
		layer_ref.set_font(&regular_font, header_font_size as f64);
		////////layer_ref.set_line_height(header_font_size);
		layer_ref.set_text_cursor(Mm(x_start), Mm(y_start));
		// Calculate the text width of the spell's name
		let width = calc_text_width(&regular_font_size_data, &header_font_scale, &spell.name);
		// Add spell name to the page
		//layer_ref.use_text(spell.name, header_font_size as f64, Mm(x_start), Mm(y_start), &regular_font);
		layer_ref.write_text(spell.name, &regular_font);
		// Decrease text height so next text gets placed lower
		text_height -= height_dec1;
		// Ends the text section
		layer_ref.end_text_section();

		// Begins a text section
		layer_ref.begin_text_section();
		// Set the font, line height, and cursor location
		layer_ref.set_font(&italic_font, body_font_size as f64);
		layer_ref.set_text_cursor(Mm(x_start), Mm(text_height));
		// get level + school text
		let text = get_level_school_text(spell);
		// Calculate text width of level and school
		let width = calc_text_width(&italic_font_size_data, &body_font_scale, &text);
		// Add level + school text to the page
		//layer_ref.use_text(text, body_font_size as f64, Mm(x_start), Mm(text_height), &italic_font);
		layer_ref.write_text(text, &italic_font);
		// Decrease text height so next text gets placed lower
		text_height -= height_dec1;
		// Ends the text section
		layer_ref.end_text_section();

		// Begins a text section
		layer_ref.begin_text_section();
		layer_ref.set_font(&bold_font, body_font_size as f64);
		layer_ref.set_text_cursor(Mm(x_start), Mm(text_height));
		let field_text = "Casting Time: ";
		let width = calc_text_width(&bold_font_size_data, &body_font_scale, &field_text);
		//layer_ref.use_text(text, body_font_size as f64, Mm(x_start), Mm(text_height), &bold_font);
		layer_ref.write_text(field_text, &bold_font);
		text_height -= height_dec2;
		// Ends the text section
		layer_ref.end_text_section();

		// Begins a text section
		layer_ref.begin_text_section();
		layer_ref.set_font(&bold_font, body_font_size as f64);
		layer_ref.set_text_cursor(Mm(x_start), Mm(text_height));
		let field_text = "Range: ";
		let width = calc_text_width(&bold_font_size_data, &body_font_scale, &field_text);
		//layer_ref.use_text(text, body_font_size as f64, Mm(x_start), Mm(text_height), &bold_font);
		layer_ref.write_text(field_text, &bold_font);
		text_height -= height_dec2;
		// Ends the text section
		layer_ref.end_text_section();

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
