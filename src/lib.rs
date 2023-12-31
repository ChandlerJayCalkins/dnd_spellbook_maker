extern crate image;
use printpdf::*;
mod spells;
mod phb_spells;

// width and height of each page in millimeters
const page_width: f64 = 210.0;
const page_height: f64 = 297.0;

pub fn printpdf_test() -> Result<(), Box<dyn std::error::Error>>
{
    // Load custom font
    let font_data = std::fs::read("fonts/Bookman/Bookman-Regular.otf")?;

	// Load image
	let img_data = image::open("img/parchment.jpg")?;
    let img1 = Image::from_dynamic_image(&img_data.clone());
	let img2 = Image::from_dynamic_image(&img_data.clone());

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
    let font = doc.add_external_font(&*font_data)?;

	// Create bookmark for first page
	doc.add_bookmark("Cover", page1);

    // Get PdfLayerReference from PdfLayerIndex
	let layer1_ref = doc.get_page(page1).get_layer(layer1);

	// Add image to document layer
	img1.add_to_layer(layer1_ref.clone(), img_transform);

    // Set font properties
    let font_size = 48.0;
    let text = "Hello! The quick brown fox jumped over the lazy dog. Peter Piper picked a prickly patch of purple pickle peppers.";

    // Add text using the custom font to the page
    layer1_ref.use_text(text, font_size, Mm(10.0), Mm(200.0), &font);

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
		layer_ref.use_text(spell.name, font_size, Mm(10.0), Mm(280.0), &font);
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
