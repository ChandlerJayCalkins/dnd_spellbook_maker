extern crate image;
use printpdf::*;
use rusttype::{Font, Scale};
mod spells;
mod phb_spells;

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
    let regular_font_data = std::fs::read("fonts/Bookman/Bookman-Regular.otf")?;
	let font_size_data = Font::try_from_bytes(&regular_font_data as &[u8]).unwrap();
	let font_scale = Scale::uniform(32.0);

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
    let (doc, page1, layer1) = PdfDocument::new("Spellbook", Mm(page_width), Mm(page_height), "Layer 1");

    // Embed the custom font into the document
    let font = doc.add_external_font(&*regular_font_data)?;

	// Create bookmark for first page
	doc.add_bookmark("Cover", page1);

    // Get PdfLayerReference from PdfLayerIndex
	let layer1_ref = doc.get_page(page1).get_layer(layer1);

	// Add image to document layer
	img1.add_to_layer(layer1_ref.clone(), img_transform);

    // Set font properties
    let font_size = 32.0;
    let text = "Hello! The quick brown fox jumped over the lazy dog. Peter Piper picked a prickly patch of purple pickle peppers.";
	let width = calc_text_width(&font_size_data, &font_scale, &text);

    // Add text using the custom font to the page
    layer1_ref.use_text(format!("{} {}", width, text), font_size, Mm(10.0), Mm(200.0), &font);

	// Add next pages
	let spell_list = vec![&phb_spells::fire_bolt, &phb_spells::fireball];
	let mut layer_count = 2;
	for spell in spell_list
	{
		let img = Image::from_dynamic_image(&img_data.clone());
		let (page, layer) = doc.add_page(Mm(page_width), Mm(page_height), format!("Layer {}", layer_count));
		doc.add_bookmark(spell.name, page);
		let layer_ref = doc.get_page(page).get_layer(layer);
		img.add_to_layer(layer_ref.clone(), img_transform);
		let width = calc_text_width(&font_size_data, &font_scale, &spell.name);
		layer_ref.use_text(format!("{} {}", width, spell.name), font_size, Mm(10.0), Mm(280.0), &font);
		layer_count += 1;
	}

    // Save the document to a file
    let file = std::fs::File::create("output.pdf")?;
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
