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

pub fn printpdf_test() -> Result<(), Box<dyn std::error::Error>>
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
	let bold_italic_size_data = Font::try_from_bytes(&bold_italic_font_data as &[u8]).unwrap();

	// Define font sizes
	const title_font_size: f32 = 32.0;
	const header_font_size: f32 = 24.0;
	const body_font_size: f32 = 12.0;

	// Create font scale objects for each font size
	let title_font_scale = Scale::uniform(title_font_size);
	let header_font_scale = Scale::uniform(header_font_size);
	let body_font_scale = Scale::uniform(body_font_size);

	// Load image
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
    let (doc, cover_page, cover_layer) = PdfDocument::new("Spellbook", Mm(page_width), Mm(page_height), "Cover Layer");

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

	// Create vec of spells for testing
	let spell_list = vec![&phb_spells::fire_bolt, &phb_spells::fireball];
	
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
		// Calculate the text width of the spell's name
		let width = calc_text_width(&regular_font_size_data, &header_font_scale, &spell.name);
		// Add text to the page
		layer_ref.use_text(format!("{} {}", width, spell.name), header_font_size as f64, Mm(10.0), Mm(280.0), &regular_font);
		// Increment the layer counter
		layer_count += 1;
	}

    // Save the document to a file
    let file = std::fs::File::create("Spellbook.pdf")?;
    doc.save(&mut std::io::BufWriter::new(file))?;

    Ok(())
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn it_works()
	{
		let _ = printpdf_test();
	}
}
