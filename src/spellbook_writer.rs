//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//	Spellbook PDF document generation
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;
use std::cell::Ref;
use std::error::Error;

extern crate image;
use printpdf::{PdfDocumentReference, PdfDocument, PdfLayerReference, IndirectFontRef, Color, Rgb, Point, Line, PdfPageIndex, Image};

use crate::spellbook_gen_types::*;
use crate::spells;

const LAYER_NAME_PREFIX: &str = "Layer";
const DEFAULT_SPELLBOOK_TITLE: &str = "Spellbook";
const TITLE_LAYER_NAME: &str = "Title Layer";
const TITLE_PAGE_NAME: &str = "Title Page";

/// All data needed to write spells to a pdf document.
// Can't derive clone or debug unfortunately.
pub struct SpellbookWriter<'a>
{
	document: PdfDocumentReference,
	layers: Vec<PdfLayerReference>,
	current_layer_index: usize,
	font_data: FontData<'a>,
	page_size_data: PageSizeData,
	page_number_data: Option<PageNumberData<'a>>,
	background: Option<BackgroundImage>,
	table_options: TableOptions,
	// Current x position of text
	x: f32,
	// Current y position of text
	y: f32
}

impl <'a> SpellbookWriter<'a>
{
	/// # Parameters
	///
	/// - `title` The title of the spellbook.
	/// - `font_paths` File paths to all of the font variants (regular, bold, italic, bold-italic).
	/// - `font_sizes` Font sizes for each type of text in the spellbook (except page numbers).
	/// - `font_scalars` Scalar values to make sure text width can be calculated correctly for each font variant.
	/// - `spacing_options` Tab size and newline sizes for each type of text (except page numbers).
	/// - `text_colors` The RGB color values for each type of text (except page numbers).
	/// - `page_size_options` Page width, height, and margin values.
	/// - `page_number_options` Settings for how page numbers look (`None` for no page numbers).
	/// - `background` An image filepath to use as backgrounds for each page and transform data to make it fit on
	/// the page the way you want.
	/// - `table_options` Sizing and color options for tables in spell descriptions.
	///
	/// # Output
	///
	/// - `Ok` Returns a `printpdf` PDF document of a spellbook and a vec of the layers in the document.
	/// - `Err` Returns any errors that occured.
	pub fn create_spellbook
	(
		title: &str,
		spells: Vec<spells::Spell>,
		font_paths: FontPaths,
		font_sizes: FontSizes,
		font_scalars: FontScalars,
		spacing_options: SpacingOptions,
		text_colors: TextColors,
		page_size_options: PageSizeOptions,
		page_number_options: Option<PageNumberOptions>,
		background: Option<(&str, ImageTransform)>,
		table_options: TableOptions
	)
	-> Result<(PdfDocumentReference, Vec<PdfLayerReference>), Box<dyn Error>>
	{
		// Construct a spellbook writer
		let mut spellbook_writer = SpellbookWriter::new
		(
			title,
			font_paths,
			font_sizes,
			font_scalars,
			spacing_options,
			text_colors,
			page_size_options,
			page_number_options,
			background,
			table_options
		)?;

		Ok((spellbook_writer.document, spellbook_writer.layers))
	}

	/// Constructor
	///
	/// # Parameters
	///
	/// - `title` The title of the spellbook.
	/// - `font_paths` File paths to all of the font variants (regular, bold, italic, bold-italic).
	/// - `font_sizes` Font sizes for each type of text in the spellbook (except page numbers).
	/// - `font_scalars` Scalar values to make sure text width can be calculated correctly for each font variant.
	/// - `spacing_options` Tab size and newline sizes for each type of text (except page numbers).
	/// - `text_colors` The RGB color values for each type of text (except page numbers).
	/// - `page_size_options` Page width, height, and margin values.
	/// - `page_number_options` Settings for how page numbers look (`None` for no page numbers).
	/// - `background` An image filepath to use as backgrounds for each page and transform data to make it fit on
	/// the page the way you want.
	/// - `table_options` Sizing and color options for tables in spell descriptions.
	///
	/// # Output
	///
	/// - `Ok` A SpellbookWriter instance.
	/// - `Err` Returns any errors that occured.
	fn new
	(
		title: &str,
		font_paths: FontPaths,
		font_sizes: FontSizes,
		font_scalars: FontScalars,
		spacing_options: SpacingOptions,
		text_colors: TextColors,
		page_size_options: PageSizeOptions,
		page_number_options: Option<PageNumberOptions>,
		background: Option<(&str, ImageTransform)>,
		table_options: TableOptions
	)
	-> Result<Self, Box<dyn Error>>
	{
		// Gets a new document and title page.
		let (doc, cover_layer_ref) =
		Self::create_new_doc(title, page_size_options.width(), page_size_options.height());

		// Combined data for all font options along with font references to the pdf doc
		let mut font_data = FontData::new
		(
			&doc,
			font_paths,
			font_sizes,
			font_scalars,
			spacing_options,
			text_colors
		)?;

		// Data for text margins and page dimensions
		let page_size_data = PageSizeData::from(page_size_options);

		// Determine whether or not page numbers are desired
		let page_number_data = match page_number_options
		{
			// If they are, then construct page number data from the options given
			Some(options) => Some(PageNumberData::new(options, &font_data)?),
			None => None
		};

		// Determine whether or not a background image is desired
		let background = match background 
		{
			// If it is, construct background image data from the options given
			Some((file_path, transform)) => Some(BackgroundImage
			{
				image: image::open(file_path)?,
				transform: transform
			}),
			None => None
		};

		// Construct instance of self and return
		Ok(Self
		{
			document: doc,
			layers: vec![cover_layer_ref],
			current_layer_index: 1,
			font_data: font_data,
			page_size_data: page_size_data,
			page_number_data: page_number_data,
			background: background,
			table_options: table_options,
			x: page_size_data.x_min(),
			y: page_size_data.y_max()
		})
	}

	/// Creates a new pdf document with a given title and width / height dimensions and returns the reference to
	/// it and layer for the title page. Returns the pdf document and the layer for the first page.
	fn create_new_doc(title: &str, width: f32, height: f32) -> (PdfDocumentReference, PdfLayerReference)
	{
		// Create the pdf document and the first page
		let (doc, cover_page, cover_layer_index) =
		// If no title was given for the spellbook (the given title string is empty)
		if title.is_empty()
		{
			// Create pdf document with a default title
			PdfDocument::new
			(
				DEFAULT_SPELLBOOK_TITLE,
				Mm(width),
				Mm(height),
				TITLE_LAYER_NAME
			)
		}
		else
		{
			// Create pdf document with the given title
			PdfDocument::new
			(
				title,
				Mm(width),
				Mm(height),
				TITLE_LAYER_NAME
			)
		};

		// Create bookmark for cover page
		doc.add_bookmark(TITLE_PAGE_NAME, cover_page);

		// Get PdfLayerReference (cover_layer_ref) from PdfLayerIndex (cover_layer_index)
		let cover_layer_ref = doc.get_page(cover_page).get_layer(cover_layer_index);

		(doc, cover_layer_ref)
	}

	/// Adds a title page to the current spellbook with the given title.
	fn make_title_page(&mut self, title: &str)
	{
		todo!()
	}

	// General Field Getters

	fn document(&self) -> &PdfDocumentReference { &self.document }
	fn layers(&self) -> &Vec<PdfLayerReference> { &self.layers }
	fn font_data(&self) -> &FontData { &self.font_data }
	fn page_size_data(&self) -> &PageSizeData { &self.page_size_data }
	fn page_number_data(&self) -> &Option<PageNumberData> { &self.page_number_data }
	fn background(&self) -> &Option<BackgroundImage> { &self.background }
	fn table_options(&self) -> &TableOptions { &self.table_options }
	/// Current x position of the text
	fn x(&self) -> &f32 { &self.x }
	/// Current y position of the text
	fn y(&self) -> &f32 { &self.y }

	// Layer Getters

	fn current_layer(&self) -> &PdfLayerReference
	{
		&self.layers.last().expect("Empty spellbook: no layers found. There should at least be a cover layer.")
	}

	// Font Getters

	/// The current font variant being used to write text (regular, bold, italic, bold-italic).
	fn current_font_variant(&self) -> &FontVariant { self.font_data.current_font_variant() }
	/// The current type of text being written.
	fn current_text_type(&self) -> &TextType { self.font_data.current_text_type() }
	/// `IndirectFontRefs` for each font variant (regular, bold, italic, bold-italic).
	fn all_font_refs(&self) -> &FontRefs { self.font_data.all_font_refs() }
	/// Font sizes for each type of text.
	fn all_font_sizes(&self) -> &FontSizes { self.font_data.all_font_sizes() }
	/// Scalar values for each font variant (regular, bold, italic, bold-italic).
	fn all_scalars(&self) -> &FontScalars { self.font_data.all_scalars() }
	/// Size data for each font variant (regular, bold, italic, bold-italic).
	fn all_size_data(&self) -> &FontSizeData { self.font_data.all_size_data() }
	/// Font scale sizing data for each type of text.
	fn all_scales(&self) -> &FontScales { self.font_data.all_scales() }
	/// All spacing options that were originally passed to this object.
	fn all_spacing_options(&self) -> &SpacingOptions { self.font_data.all_spacing_options() }
	/// RGB color values for each type of text.
	fn all_text_colors(&self) -> &TextColors { self.font_data.all_text_colors() }
	/// Tab size in pringpdf Mm.
	fn tab_amount(&self) -> f32 { self.font_data.tab_amount() }
	/// The font object for the current font variant being used.
	fn current_font_ref(&self) -> &IndirectFontRef { self.font_data.current_font_ref() }
	/// Font size of the current type of text being used.
	fn current_font_size(&self) -> f32 { self.font_data.current_font_size() }
	/// Scalar value of the current font variant being used (regular, bold, italic, bold-italic).
	fn current_scalar(&self) -> f32 { self.font_data.current_scalar() }
	/// Size data of the current font variant being used (regular, bold, italic, bold-italic).
	fn current_size_data(&self) -> &Font { self.font_data.current_size_data() }
	/// Scale sizing data of the current type of text being used.
	fn current_font_scale(&self) -> &Scale { self.font_data.current_font_scale() }
	/// Newline size in printpdf Mm of the current type of text being used.
	fn current_newline_amount(&self) -> f32 { self.font_data.current_newline_amount() }
	/// RGB color values for the current type of text being used.
	fn current_text_color(&self) -> &(u8, u8, u8) { self.font_data.current_text_color() }

	// Page Size Getters

	// Entire page dimensions
	fn page_width(&self) -> f32 { self.page_size_data.page_width() }
	fn page_height(&self) -> f32 { self.page_size_data.page_height() }
	/// Left
	fn x_min(&self) -> f32 { self.page_size_data.x_min() }
	/// Right
	fn x_max(&self) -> f32 { self.page_size_data.x_max() }
	/// Bottom
	fn y_min(&self) -> f32 { self.page_size_data.y_min() }
	/// Top
	fn y_max(&self) -> f32 { self.page_size_data.y_max() }
	// Dimensions that text can fit inside
	pub fn text_width(&self) -> f32 { self.page_size_data.text_width() }
	pub fn text_height(&self) -> f32 { self.page_size_data.text_height() }

	// Page Number Getters

	/// The side of the page (left or right) the page number starts on.
	fn starting_page_number_side(&self) -> Option<HSide>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.starting_side()),
			None => None
		}
	}

	/// Whether or not the page number flips sides every page.
	fn page_number_flips_sides(&self) -> Option<bool>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.flips_sides()),
			None => None
		}
	}

	/// The starting page number.
	fn starting_page_number(&self) -> Option<i64>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.starting_num()),
			None => None
		}
	}

	/// The font variant the page numbers use.
	fn page_number_font_variant(&self) -> Option<FontVariant>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.font_variant()),
			None => None
		}
	}

	/// The font size of the page numbers.
	fn page_number_font_size(&self) -> Option<f32>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.font_size()),
			None => None
		}
	}

	/// The amount of space between newlines for page numbers in case of overflow.
	fn page_number_newline_amount(&self) -> Option<f32>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.newline_amount()),
			None => None
		}
	}

	/// RGB color values for page numbers.
	fn page_number_color(&self) -> Option<(u8, u8, u8)>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.color()),
			None => None
		}
	}

	/// The amount of space between the side of the page and the page number in printpdf Mm.
	fn page_number_side_margin(&self) -> Option<f32>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.side_margin()),
			None => None
		}
	}
	
	/// The amount of space between the bottom of the page and the page number in printpdf Mm.
	fn page_number_bottom_margin(&self) -> Option<f32>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.bottom_margin()),
			None => None
		}
	}

	/// All of the original page number options that were inputted.
	fn page_number_options(&self) -> Option<&PageNumberOptions>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.options()),
			None => None
		}
	}

	/// The current side of the page (left or right) the page number is on.
	fn current_page_number_side(&self) -> Option<HSide>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.current_side()),
			None => None
		}
	}

	/// Returns the font ref to the current font type bring used for page numbers.
	fn page_number_font_ref(&self) -> Option<&IndirectFontRef>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.font_ref()),
			None => None
		}
	}

	/// Returns the scalar value of the font type being used for page numbers.
	fn page_number_font_scalar(&self) -> Option<f32>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.font_scalar()),
			None => None
		}
	}

	/// Returns the size data of the current font type being used for page numbers.
	fn page_number_font_size_data(&self) -> Option<&Font>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.font_size_data()),
			None => None
		}
	}

	/// The font scale size data for the page numbers.
	fn page_number_font_scale(&self) -> Option<&Scale>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.font_scale()),
			None => None
		}
	}

	// Table Getters

	/// Space between columns in printpdf Mm.
	fn table_horizontal_cell_margin(&self) -> f32 { self.table_options.horizontal_cell_margin() }
	/// Space between rows in printpdf Mm.
	fn table_vertical_cell_margin(&self) -> f32 { self.table_options.vertical_cell_margin() }
	/// Minimum space between sides of table and sides of pages in printpdf Mm.
	fn table_outer_horizontal_margin(&self) -> f32 { self.table_options.outer_horizontal_margin() }
	/// Space above and below table from other text / tables in printpdf Mm.
	fn table_outer_vertical_margin(&self) -> f32 { self.table_options.outer_vertical_margin() }
	/// Scalar value to adjust off-row color lines to line up with the rows vertically.
	fn table_off_row_color_lines_y_adjust_scalar(&self) -> f32
	{ self.table_options.off_row_color_lines_y_adjust_scalar() }
	/// Scalar value to determine the height of off-row color lines.
	fn table_off_row_color_lines_height_scalar(&self) -> f32
	{ self.table_options.off_row_color_lines_height_scalar() }
	// RGB value of the color of the off-row color lines.
	fn table_off_row_color(&self) -> (u8, u8, u8) { self.table_options.off_row_color() }

	// Font Setters

	/// Sets the current font variant that is being used to write text to the spellbook.
	fn set_current_font_variant(&mut self, font_type: FontVariant)
	{ self.font_data.set_current_font_variant(font_type); }
	/// Sets the current type of text that is being written to the spellbook.
	fn set_current_text_type(&mut self, text_type: TextType) { self.font_data.set_current_text_type(text_type); }

	// Page Number Setters

	/// Flips the side of the page that page numbers appear on.
	fn flip_page_number_side(&mut self)
	{
		match &mut self.page_number_data
		{
			Some(ref mut data) => data.flip_side(),
			None => ()
		}
	}
}
