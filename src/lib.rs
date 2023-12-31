extern crate image;
use printpdf::*;

pub fn printpdf_test() -> Result<(), Box<dyn std::error::Error>>
{
    // Load custom font
    let font_data = std::fs::read("fonts/Bookman/Bookman-Regular.otf")?;

	// Load image
	let img_data = image::open("img/parchment.jpg")?;
    let img = Image::from_dynamic_image(&img_data);

    // Create PDF document
    let (document, page1, layer1) = PdfDocument::new("My Document", Mm(210.0), Mm(297.0), "Layer 1");

    // Embed the custom font into the document
    let font = document.add_external_font(&*font_data)?;

    // Change PdfLayerIndex to PdfLayerReference
	let layer1_ref = document.get_page(page1).get_layer(layer1);

	// Determine position, size, and rotation of image
	let img_transform = ImageTransform
	{
		translate_x: Some(Mm(0.0)),
		translate_y: Some(Mm(0.0)),
		scale_x: Some(1.95),
		scale_y: Some(2.125),
		..Default::default()
	};

	// Add image to document layer
	img.add_to_layer(layer1_ref.clone(), img_transform);

    // Set font properties
    let font_size = 48.0;
    let text = "Hello!";

    // Add text using the custom font to the page
    layer1_ref.use_text(text, font_size, Mm(10.0), Mm(200.0), &font);

    // Save the document to a file
    let file = std::fs::File::create("output.pdf")?;
    document.save(&mut std::io::BufWriter::new(file))?;

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
