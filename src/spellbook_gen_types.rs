//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//	Input types for SpellbookWriter
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;
use std::error::Error;
use std::fmt;

pub use image::DynamicImage;
pub use rusttype::{Font, Scale, point};
pub use printpdf::{PdfDocumentReference, IndirectFontRef, Color, Rgb};

pub use crate::spellbook_options::*;

pub const SPACE: &str = " ";

/// Converts rgb byte values into a `printpdf::Color` struct.
fn bytes_to_color(rgb: &(u8, u8, u8)) -> Color
{
	const BYTE_MAX: f32 = 255.0;
	Color::Rgb(Rgb::new
	(
		rgb.0 as f32 / BYTE_MAX,
		rgb.1 as f32 / BYTE_MAX,
		rgb.2 as f32 / BYTE_MAX,
		None
	))
}

/// Conveys the type of text that is being used.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum TextType
{
	Title = 0,
	Header = 1,
	Body = 2,
	TableTitle = 3,
	TableBody = 4
}
/// This must always be the same as the number of variants in `TextType`
const TEXTTYPE_VARIANTS: usize = 5;

/// Holds the bytes from inputted font files.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontBytes
{
	pub regular: Vec<u8>,
	pub bold: Vec<u8>,
	pub italic: Vec<u8>,
	pub bold_italic: Vec<u8>
}

/// Holds references to each font type of a font.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontRefs
{
	pub regular: IndirectFontRef,
	pub bold: IndirectFontRef,
	pub italic: IndirectFontRef,
	pub bold_italic: IndirectFontRef
}

/// Holds size data for each font type of a font.
#[derive(Clone, Debug)]
pub struct FontSizeData<'a>
{
	pub regular: Font<'a>,
	pub bold: Font<'a>,
	pub italic: Font<'a>,
	pub bold_italic: Font<'a>
}

/// Holds scale size data for each type of text.
#[derive(Clone, Debug, PartialEq)]
pub struct FontScales
{
	pub title: Scale,
	pub header: Scale,
	pub body: Scale,
	pub table_title: Scale,
	pub table_body: Scale
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextColors
{
	/// Cover page text.
	pub title_color: Color,
	/// Spell name text.
	pub header_color: Color,
	/// Spells fields and description.
	pub body_color: Color,
	/// Title labels above tables in spell descriptions.
	pub table_title_color: Color,
	/// Cell text in spell description tables.
	pub table_body_color: Color
}

impl From<TextColorOptions> for TextColors
{
	fn from(colors: TextColorOptions) -> Self
	{
		Self
		{
			title_color: bytes_to_color(&colors.title_color),
			header_color: bytes_to_color(&colors.header_color),
			body_color: bytes_to_color(&colors.body_color),
			table_title_color: bytes_to_color(&colors.table_title_color),
			table_body_color: bytes_to_color(&colors.table_body_color)
		}
	}
}

/// Keeps track of the current font variant being used, the current type of text, and other data needed to use fonts.
#[derive(Clone, Debug)]
pub struct FontData<'a>
{
	current_font_variant: FontVariant,
	current_text_type: TextType,
	font_bytes: FontBytes,
	font_refs: FontRefs,
	font_sizes: FontSizes,
	scalars: FontScalars,
	size_data: FontSizeData<'a>,
	scales: FontScales,
	spacing_options: SpacingOptions,
	text_colors: TextColors
}

/// Error for when font size data couldn't be converted from bytes read from a font file to an object in rust.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BytesToFontSizeDataConversionError(String);

impl std::fmt::Display for BytesToFontSizeDataConversionError
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(f, "{}", self.0)
	}
}
impl std::error::Error for BytesToFontSizeDataConversionError {}

impl <'a> FontData<'a>
{
	/// Constructor
	///
	/// # Parameters
	///
	/// - `doc` Reference to the pdf document that this font will be used in.
	/// - `font_paths` File paths to the font files.
	/// - `font_sizes` The sizes of each type of text.
	/// - `font_scalars` Scalar values for each font variant so their sizes can be calculated correctly.
	/// - `spacing_options` Tab sizes and newline sizes for each type of text.
	/// - `text_colors` RGB color values for each type of text.
	pub fn new
	(
		doc: &PdfDocumentReference,
		font_paths: FontPaths,
		font_sizes: FontSizes,
		font_scalars: FontScalars,
		spacing_options: SpacingOptions,
		text_colors: TextColorOptions
	)
	-> Result<Self, Box<dyn std::error::Error>>
	{
		// Read the data from the font files
		let regular_font_bytes = fs::read(&font_paths.regular)?;
		let bold_font_bytes = fs::read(&font_paths.bold)?;
		let italic_font_bytes = fs::read(&font_paths.italic)?;
		let bold_italic_font_bytes = fs::read(&font_paths.bold_italic)?;

		// Put the bytes into a struct to reuse them if new font refs need to be created when a new pdf document is created.
		let font_bytes = FontBytes
		{
			regular: regular_font_bytes.clone(),
			bold: bold_font_bytes.clone(),
			italic: italic_font_bytes.clone(),
			bold_italic: bold_italic_font_bytes.clone()
		};

		// Create font size data for each font variant
		let regular_font_size_data = match Font::try_from_vec(regular_font_bytes.clone())
		{
			Some(d) => d,
			None => return Err(Box::new(BytesToFontSizeDataConversionError(String::from
				("Could not convert regular font size data from bytes."))))
		};
		let bold_font_size_data = match Font::try_from_vec(bold_font_bytes.clone())
		{
			Some(d) => d,
			None => return Err(Box::new(BytesToFontSizeDataConversionError(String::from
				("Could not convert bold font size data from bytes."))))
		};
		let italic_font_size_data = match Font::try_from_vec(italic_font_bytes.clone())
		{
			Some(d) => d,
			None => return Err(Box::new(BytesToFontSizeDataConversionError(String::from
				("Could not convert italic font size data from bytes."))))
		};
		let bold_italic_font_size_data = match Font::try_from_vec(bold_italic_font_bytes.clone())
		{
			Some(d) => d,
			None => return Err(Box::new(BytesToFontSizeDataConversionError(String::from
				("Could not convert bold italic font size data from bytes."))))
		};

		// Combine all size data into one struct
		let size_data = FontSizeData
		{
			regular: regular_font_size_data,
			bold: bold_font_size_data,
			italic: italic_font_size_data,
			bold_italic: bold_italic_font_size_data
		};

		// Create font scale objects for each font size
		let title_font_scale = Scale::uniform(font_sizes.title_font_size());
		let header_font_scale = Scale::uniform(font_sizes.header_font_size());
		let body_font_scale = Scale::uniform(font_sizes.body_font_size());
		let table_title_font_scale = Scale::uniform(font_sizes.table_title_font_size());
		let table_body_font_scale = Scale::uniform(font_sizes.table_body_font_size());

		// Combine all font scales into one struct
		let scales = FontScales
		{
			title: title_font_scale,
			header: header_font_scale,
			body: body_font_scale,
			table_title: table_title_font_scale,
			table_body: table_body_font_scale
		};

		// Add all custom font variants to the document and get references to them
		let regular_font_ref = doc.add_external_font(&*regular_font_bytes)?;
		let bold_font_ref = doc.add_external_font(&*bold_font_bytes)?;
		let italic_font_ref = doc.add_external_font(&*italic_font_bytes)?;
		let bold_italic_font_ref = doc.add_external_font(&*bold_italic_font_bytes)?;

		// Combine all font references into one struct
		let font_refs = FontRefs
		{
			regular: regular_font_ref,
			italic: italic_font_ref,
			bold: bold_font_ref,
			bold_italic: bold_italic_font_ref
		};

		// Construct and return
		Ok(Self
		{
			// Use these defaults for the first two fields since the cover page is what gets created first
			current_font_variant: FontVariant::Regular,
			current_text_type: TextType::Title,
			font_bytes: font_bytes,
			font_refs: font_refs,
			font_sizes: font_sizes,
			scalars: font_scalars,
			size_data: size_data,
			scales: scales,
			spacing_options: spacing_options,
			text_colors: TextColors::from(text_colors)
		})
	}

	// Getters

	pub fn current_font_variant(&self) -> &FontVariant { &self.current_font_variant }
	pub fn current_text_type(&self) -> &TextType { &self.current_text_type }
	pub fn bytes(&self) -> &FontBytes { &self.font_bytes }
	pub fn all_font_refs(&self) -> &FontRefs { &self.font_refs }
	// pub fn all_font_sizes(&self) -> &FontSizes { &self.font_sizes }
	pub fn all_scalars(&self) -> &FontScalars { &self.scalars }
	// pub fn all_size_data(&self) -> &FontSizeData { &self.size_data }
	// pub fn all_scales(&self) -> &FontScales { &self.scales }
	// pub fn all_spacing_options(&self) -> &SpacingOptions { &self.spacing_options }
	// pub fn all_text_colors(&self) -> &TextColors { &self.text_colors }
	pub fn tab_amount(&self) -> f32 { self.spacing_options.tab_amount() }

	// /// Returns a vec of bytes that were used to construct certain fields for a specific font variant.
	// pub fn get_bytes_for(&self, font_variant: FontVariant) -> &Vec<u8>
	// {
	// 	match font_variant
	// 	{
	// 		FontVariant::Regular => &self.font_bytes.regular,
	// 		FontVariant::Bold => &self.font_bytes.bold,
	// 		FontVariant::Italic => &self.font_bytes.italic,
	// 		FontVariant::BoldItalic => &self.font_bytes.bold_italic
	// 	}
	// }

	// /// Returns a vec of bytes that were used to construct certain fields for the current font variant.
	// pub fn current_bytes(&self) -> &Vec<u8>
	// {
	// 	match self.current_font_variant
	// 	{
	// 		FontVariant::Regular => &self.font_bytes.regular,
	// 		FontVariant::Bold => &self.font_bytes.bold,
	// 		FontVariant::Italic => &self.font_bytes.italic,
	// 		FontVariant::BoldItalic => &self.font_bytes.bold_italic
	// 	}
	// }

	// /// Returns the font ref for a specific font variant.
	// pub fn get_font_ref_for(&self, font_variant: FontVariant) -> &IndirectFontRef
	// {
	// 	match font_variant
	// 	{
	// 		FontVariant::Regular => &self.font_refs.regular,
	// 		FontVariant::Bold => &self.font_refs.bold,
	// 		FontVariant::Italic => &self.font_refs.italic,
	// 		FontVariant::BoldItalic => &self.font_refs.bold_italic
	// 	}
	// }

	/// Returns the font ref to the current font variant bring used.
	pub fn current_font_ref(&self) -> &IndirectFontRef
	{
		match self.current_font_variant
		{
			FontVariant::Regular => &self.font_refs.regular,
			FontVariant::Bold => &self.font_refs.bold,
			FontVariant::Italic => &self.font_refs.italic,
			FontVariant::BoldItalic => &self.font_refs.bold_italic
		}
	}

	// /// Returns the font size of a specific text type.
	// pub fn get_font_size_for(&self, text_type: TextType) -> f32
	// {
	// 	match text_type
	// 	{
	// 		TextType::Title => self.font_sizes.title_font_size(),
	// 		TextType::Header => self.font_sizes.header_font_size(),
	// 		TextType::Body => self.font_sizes.body_font_size(),
	// 		TextType::TableTitle => self.font_sizes.table_title_font_size(),
	// 		TextType::TableBody => self.font_sizes.table_body_font_size()
	// 	}
	// }

	/// Returns the font size of the current text type bring used.
	pub fn current_font_size(&self) -> f32
	{
		match self.current_text_type
		{
			TextType::Title => self.font_sizes.title_font_size(),
			TextType::Header => self.font_sizes.header_font_size(),
			TextType::Body => self.font_sizes.body_font_size(),
			TextType::TableTitle => self.font_sizes.table_title_font_size(),
			TextType::TableBody => self.font_sizes.table_body_font_size()
		}
	}

	/// Returns the scalar value for a specific font variant.
	pub fn get_scalar_for(&self, font_variant: FontVariant) -> f32
	{
		match font_variant
		{
			FontVariant::Regular => self.scalars.regular_scalar(),
			FontVariant::Bold => self.scalars.bold_scalar(),
			FontVariant::Italic => self.scalars.italic_scalar(),
			FontVariant::BoldItalic => self.scalars.bold_italic_scalar()
		}
	}

	/// Returns the scalar value for the current font variant being used.
	pub fn current_scalar(&self) -> f32
	{
		match self.current_font_variant
		{
			FontVariant::Regular => self.scalars.regular_scalar(),
			FontVariant::Bold => self.scalars.bold_scalar(),
			FontVariant::Italic => self.scalars.italic_scalar(),
			FontVariant::BoldItalic => self.scalars.bold_italic_scalar()
		}
	}

	/// Returns size data for a specific font variant.
	pub fn get_size_data_for(&self, font_variant: FontVariant) -> &Font
	{
		match font_variant
		{
			FontVariant::Regular => &self.size_data.regular,
			FontVariant::Bold => &self.size_data.bold,
			FontVariant::Italic => &self.size_data.italic,
			FontVariant::BoldItalic => &self.size_data.bold_italic
		}
	}

	/// Returns the size data for the current font variant being used.
	pub fn current_size_data(&self) -> &Font
	{
		match self.current_font_variant
		{
			FontVariant::Regular => &self.size_data.regular,
			FontVariant::Bold => &self.size_data.bold,
			FontVariant::Italic => &self.size_data.italic,
			FontVariant::BoldItalic => &self.size_data.bold_italic
		}
	}

	/// Returns the font scale for a specific text type.
	pub fn get_font_scale_for(&self, text_type: TextType) -> &Scale
	{
		match text_type
		{
			TextType::Title => &self.scales.title,
			TextType::Header => &self.scales.header,
			TextType::Body => &self.scales.body,
			TextType::TableTitle => &self.scales.table_title,
			TextType::TableBody => &self.scales.table_body
		}
	}

	/// Returns the font scale of the current text type bring used.
	pub fn current_font_scale(&self) -> &Scale
	{
		match self.current_text_type
		{
			TextType::Title => &self.scales.title,
			TextType::Header => &self.scales.header,
			TextType::Body => &self.scales.body,
			TextType::TableTitle => &self.scales.table_title,
			TextType::TableBody => &self.scales.table_body
		}
	}

	/// Returns the newline amount for a specific text type.
	pub fn get_newline_amount_for(&self, text_type: TextType) -> f32
	{
		match text_type
		{
			TextType::Title => self.spacing_options.title_newline_amount(),
			TextType::Header => self.spacing_options.header_newline_amount(),
			TextType::Body => self.spacing_options.body_newline_amount(),
			TextType::TableTitle => self.spacing_options.table_title_newline_amount(),
			TextType::TableBody => self.spacing_options.table_body_newline_amount()
		}
	}

	/// Returns the newline amount of the current text type being used.
	pub fn current_newline_amount(&self) -> f32
	{
		match self.current_text_type
		{
			TextType::Title => self.spacing_options.title_newline_amount(),
			TextType::Header => self.spacing_options.header_newline_amount(),
			TextType::Body => self.spacing_options.body_newline_amount(),
			TextType::TableTitle => self.spacing_options.table_title_newline_amount(),
			TextType::TableBody => self.spacing_options.table_body_newline_amount()
		}
	}

	// /// Returns the font the RGB values for the font color of a specific text type.
	// pub fn get_text_color_for(&self, text_type: TextType) -> &Color
	// {
	// 	match text_type
	// 	{
	// 		TextType::Title => &self.text_colors.title_color,
	// 		TextType::Header => &self.text_colors.header_color,
	// 		TextType::Body => &self.text_colors.body_color,
	// 		TextType::TableTitle => &self.text_colors.table_title_color,
	// 		TextType::TableBody => &self.text_colors.table_body_color
	// 	}
	// }

	/// Returns the RGB values for the font color of the current text type being used.
	pub fn current_text_color(&self) -> &Color
	{
		match self.current_text_type
		{
			TextType::Title => &self.text_colors.title_color,
			TextType::Header => &self.text_colors.header_color,
			TextType::Body => &self.text_colors.body_color,
			TextType::TableTitle => &self.text_colors.table_title_color,
			TextType::TableBody => &self.text_colors.table_body_color
		}
	}

	// Setters

	/// Sets the current font variant being used (regular, bold, italic, bold-italic).
	pub fn set_current_font_variant(&mut self, font_type: FontVariant) { self.current_font_variant = font_type; }
	/// Sets the current text type of the text.
	pub fn set_current_text_type(&mut self, text_type: TextType) { self.current_text_type = text_type; }
}

/// Holds the width and height of the spellbook pages, and the min and max coordinates for text on the page.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageSizeData
{
	// Entire page dimensions
	page_width: f32,
	page_height: f32,
	// Left
	x_min: f32,
	// Right
	x_max: f32,
	// Bottom
	y_min: f32,
	// Top
	y_max: f32,
	// Dimensions that text can fit inside
	text_width: f32,
	text_height: f32
}

/// Allows page limit coordinates to be constructed from the `PageSizeOptions` user input type.
impl From<PageSizeOptions> for PageSizeData
{
	/// Converts a `PageSizeOptions` object into a `PageSizeData` object for usage in spellbook writing.
	fn from(data: PageSizeOptions) -> Self
	{
		Self
		{
			page_width: data.width(),
			page_height: data.height(),
			x_min: data.left_margin(),
			x_max: data.width() - data.right_margin(),
			y_min: data.bottom_margin(),
			y_max: data.height() - data.top_margin(),
			text_width: data.width() - (data.left_margin() + data.right_margin()),
			text_height: data.height() - (data.bottom_margin() + data.top_margin())
		}
	}
}

impl PageSizeData
{
		// Getters

		// Entire page dimensions
		pub fn page_width(&self) -> f32 { self.page_width }
		pub fn page_height(&self) -> f32 { self.page_height }
		/// Left
		pub fn x_min(&self) -> f32 { self.x_min }
		/// Right
		pub fn x_max(&self) -> f32 { self.x_max }
		/// Bottom
		pub fn y_min(&self) -> f32 { self.y_min }
		/// Top
		pub fn y_max(&self) -> f32 { self.y_max }
		// // Dimensions that text can fit inside
		// pub fn text_width(&self) -> f32 { self.text_width }
		// pub fn text_height(&self) -> f32 { self.text_height }
}

/// Holds all page number data needed for writing them into spellbooks.
#[derive(Clone, Debug)]
pub struct PageNumberData<'a>
{
	options: PageNumberOptions,
	current_side: HSide,
	font_ref: IndirectFontRef,
	font_scalar: f32,
	font_size_data: Font<'a>,
	font_scale: Scale,
	color: Color
}

impl <'a> PageNumberData<'a>
{
	/// Constructor
	///
	/// # Parameters
	///
	/// - `options` Options for how the page numbers should be displayed.
	/// - `font_data` Data for how fonts are displayed in the spellbook.
	pub fn new(options: PageNumberOptions, font_data: &FontData<'_>)
	-> Result<Self, Box<dyn std::error::Error>>
	{
		// Gets copies of all of the font data the page numbers need based on the font variant they will use.
		let (font_ref, font_scalar, font_size_data) = match options.font_variant()
		{
			FontVariant::Regular =>
			(
				font_data.all_font_refs().regular.clone(),
				font_data.all_scalars().regular_scalar(),
				// Create new font size data for this struct since it has problems with holding references
				// to font_data's fields
				match Font::try_from_vec(font_data.bytes().regular.clone())
				{
					Some(d) => d,
					None => return Err(Box::new(BytesToFontSizeDataConversionError(String::from
						("Could not convert regular font size data from bytes."))))
				}
			),
			FontVariant::Bold =>
			(
				font_data.all_font_refs().bold.clone(),
				font_data.all_scalars().bold_scalar(),
				// Create new font size data for this struct since it has problems with holding references
				// to font_data's fields
				match Font::try_from_vec(font_data.bytes().bold.clone())
				{
					Some(d) => d,
					None => return Err(Box::new(BytesToFontSizeDataConversionError(String::from
						("Could not convert bold font size data from bytes."))))
				}
			),
			FontVariant::Italic =>
			(
				font_data.all_font_refs().italic.clone(),
				font_data.all_scalars().italic_scalar(),
				// Create new font size data for this struct since it has problems with holding references
				// to font_data's fields
				match Font::try_from_vec(font_data.bytes().italic.clone())
				{
					Some(d) => d,
					None => return Err(Box::new(BytesToFontSizeDataConversionError(String::from
						("Could not convert italic font size data from bytes."))))
				}
			),
			FontVariant::BoldItalic =>
			(
				font_data.all_font_refs().bold_italic.clone(),
				font_data.all_scalars().bold_italic_scalar(),
				// Create new font size data for this struct since it has problems with holding references
				// to font_data's fields
				match Font::try_from_vec(font_data.bytes().bold_italic.clone())
				{
					Some(d) => d,
					None => return Err(Box::new(BytesToFontSizeDataConversionError(String::from
						("Could not convert bold_italic font size data from bytes."))))
				}
			)
		};

		// Calculates the font scale data based on the font size option.
		let font_scale = Scale::uniform(options.font_size());
		
		// Construct and return
		Ok(Self
		{
			options: options,
			current_side: options.starting_side(),
			font_ref: font_ref,
			font_scalar: font_scalar,
			font_size_data: font_size_data,
			font_scale: font_scale,
			color: bytes_to_color(&options.color())
		})
	}

	// Getters

	// pub fn starting_side(&self) -> HSide { self.options.starting_side() }
	pub fn flips_sides(&self) -> bool { self.options.flips_sides() }
	// pub fn starting_num(&self) -> i64 { self.options.starting_num() }
	// pub fn font_variant(&self) -> FontVariant { self.options.font_variant() }
	pub fn font_size(&self) -> f32 { self.options.font_size() }
	// pub fn newline_amount(&self) -> f32 { self.options.newline_amount() }
	pub fn side_margin(&self) -> f32 { self.options.side_margin() }
	pub fn bottom_margin(&self) -> f32 { self.options.bottom_margin() }
	// pub fn options(&self) -> &PageNumberOptions { &self.options }
	pub fn current_side(&self) -> HSide { self.current_side }
	pub fn font_ref(&self) -> &IndirectFontRef { &self.font_ref }
	pub fn font_scalar(&self) -> f32 { self.font_scalar }
	pub fn font_size_data(&self) -> &Font { &self.font_size_data }
	pub fn font_scale(&self) -> &Scale { &self.font_scale }
	pub fn color(&self) -> &Color { &self.color }

	// Setters

	/// Flips the horizontal side of the page (left or right) the page number is currently on.
	pub fn flip_side(&mut self) { self.current_side = !self.current_side; }
}

/// Holds the background image and the transform data for it (positioning, size, rotation, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct BackgroundImage
{
	image: DynamicImage,
	transform: ImageTransform
}

impl BackgroundImage
{
	/// Constructor
	///
	/// # Parameters
	///
	/// - `image_path` A filepath to an image to use.
	/// - `transform` Transform data for how the image should be placed on pages (positioning, size, rotation, etc.).
	///
	/// # Output
	///
	/// - `Ok` A `BackgroundImage` instance.
	/// - `Err` Any errors that occured.
	pub fn new(image_path: &str, transform: ImageTransform) -> Result<Self, Box<dyn Error>>
	{
		// Construct and return
		Ok(Self
		{
			// Constructs a `image::DynamicImage` from the file at the given filepath
			image: image::open(image_path)?,
			transform: transform
		})
	}

	// Getters

	pub fn image(&self) -> &DynamicImage { &self.image }
	pub fn transform(&self) -> &ImageTransform { &self.transform }
}

/// Holds the extra data needed for making tables inside of spellbooks.
#[derive(Clone, Debug, PartialEq)]
pub struct TableData
{
	horizontal_cell_margin: f32,
	vertical_cell_margin: f32,
	outer_horizontal_margin: f32,
	outer_vertical_margin: f32,
	off_row_color_lines_y_adjust_scalar: f32,
	off_row_color_lines_height_scalar: f32,
	off_row_color: Color
}

impl From<TableOptions> for TableData
{
	/// Allows `TableData`s to be constructed from `TableOptions`
	fn from(options: TableOptions) -> Self
	{
		Self
		{
			horizontal_cell_margin: options.horizontal_cell_margin(),
			vertical_cell_margin: options.vertical_cell_margin(),
			outer_horizontal_margin: options.outer_horizontal_margin(),
			outer_vertical_margin: options.outer_vertical_margin(),
			off_row_color_lines_y_adjust_scalar: options.off_row_color_lines_y_adjust_scalar(),
			off_row_color_lines_height_scalar: options.off_row_color_lines_height_scalar(),
			off_row_color: bytes_to_color(&options.off_row_color())
		}
	}
}

impl TableData
{
	// Getters
	pub fn horizontal_cell_margin(&self) -> f32 { self.horizontal_cell_margin }
	pub fn vertical_cell_margin(&self) -> f32 { self.vertical_cell_margin }
	pub fn outer_horizontal_margin(&self) -> f32 { self.outer_horizontal_margin }
	pub fn outer_vertical_margin(&self) -> f32 { self.outer_vertical_margin }
	pub fn off_row_color_lines_y_adjust_scalar(&self) -> f32 { self.off_row_color_lines_y_adjust_scalar }
	pub fn off_row_color_lines_height_scalar(&self) -> f32 { self.off_row_color_lines_height_scalar }
	pub fn off_row_color(&self) -> &Color { &self.off_row_color }
}

/// Used for returning the result of whether or not a token was a table tag, an escaped table tag, or neither.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableTagCheckResult
{
	TableTag(usize),
	EscapedTableTag,
	NotTableTag
}

/// Holds a single token in a spellbook
#[derive(Clone, Debug, PartialEq)]
pub enum Token
{
	/// A symbol that changes the font variant that the following text uses.
	// Ex: Regular: "<r>", Bold: "<b>", Italic: "<i>", Bold-Italic: "<bi>" or "<ib>".
	FontTag(FontVariant),
	/// A space character
	Space,
	/// Tokens that are treated like text and are applied to the page.
	Text(TextToken)
}

impl Token
{
	/// Gets a string of this token as it will appear in the spellbook.
	/// Font tags will return an empty string, text tokens will return the string they are holding.
	pub fn as_spellbook_string(&self) -> &str
	{
		static EMPTY_STR: &str = "";
		match self
		{
			Self::FontTag(_) => EMPTY_STR,
			Self::Space => SPACE,
			Self::Text(token) => &token.text()
		}
	}
}

impl fmt::Display for Token
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		match self
		{
			Self::FontTag(tag) => tag.fmt(f),
			Self::Space => write!(f, "{}", SPACE),
			Self::Text(token) => token.fmt(f)
		}
	}
}

/// Holds a single token of text in a spellbook along with the width of the text in the font it will be applied with.
#[derive(Clone, Debug, PartialEq)]
pub struct TextToken
{
	/// The actual text.
	text: String,
	/// The width of the token in `printpdf::Mm` units.
	width: f32
}

impl TextToken
{
	// /// Creates a new text token from a string and font data. Calculates width automatically.
	// pub fn new(text: &str, font_size_data: &Font, font_scale: &Scale, font_scalar: f32) -> Self
	// {
	// 	let width = calc_text_width(text, font_size_data, font_scale, font_scalar);
	// 	Self
	// 	{
	// 		text: String::from(text),
	// 		width: width
	// 	}
	// }

	/// Creates a new text token from a string and a precalculated width of that string. Does not check to make sure
	/// the given width is correct.
	pub fn with_width(text: &str, width: f32) -> Self
	{
		Self
		{
			text: String::from(text),
			width: width
		}
	}

	/// Creates a new empty text token with no text and a width of 0.0.
	pub fn empty() -> Self
	{
		Self
		{
			text: String::new(),
			width: 0.0
		}
	}

	// Getters

	/// Returns the text this object is holding.
	pub fn text(&self) -> &str {&self.text.as_str() }
	// /// Returns the width of the text his object is holding.
	// pub fn width(&self) -> f32 { self.width }
	/// Returns the number of bytes in the string of this text.
	pub fn byte_count(&self) -> usize { self.text.len() }
}

impl fmt::Display for TextToken
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self.text)
	}
}

/// Holds a line of tokens that will be applied to a spellbook along with the width of the entire line.
#[derive(Clone, Debug, PartialEq)]
pub struct TextLine
{
	/// The line of tokens that will be applied to the spellbook.
	tokens: Vec<Token>,
	/// The width of the entire line in `printpdf::Mm` units.
	width: f32,
	/// The number of bytes the string of this line will take up
	byte_count: usize,
	/// Holds the text type of this line (used for calculating space widths)
	text_type: TextType,
	/// Holds the current font variant of the line (used for calculating space widths)
	current_font_variant: FontVariant,
	/// Holds the font variant of the previous token in the line (used for calculating space widths)
	previous_font_variant: FontVariant
}

impl std::ops::Index<usize> for TextLine
{
	type Output = Token;
	/// Allows `TextLine`s to be indexed using the index operator `[]` to retrieve individual tokens.
	fn index(&self, index: usize) -> &Self::Output { &self.tokens[index] }
}

impl TextLine
{
	// /// Creates a new empty text line.
	// pub fn new(text_type: TextType, current_font_variant: FontVariant) -> Self
	// {
	// 	Self::with_capacity(0, text_type, current_font_variant)
	// }

	/// Creates a new text line with a given capacity for its vec of tokens.
	pub fn with_capacity(size: usize, text_type: TextType, current_font_variant: FontVariant) -> Self
	{
		Self
		{
			tokens: Vec::with_capacity(size),
			width: 0.0,
			byte_count: 0,
			text_type: text_type,
			current_font_variant: current_font_variant,
			previous_font_variant: current_font_variant
		}
	}

	// Setters

	// /// Adds a token to the line.
	// pub fn add_token(&mut self, token: Token, space_widths: &SpaceWidths)
	// {
	// 	match token
	// 	{
	// 		Token::Text(text) => self.add_text(text, space_widths),
	// 		Token::FontTag(tag) => self.add_font_tag(tag)
	// 	}
	// }

	/// Adds a font tag to the line.
	pub fn add_font_tag(&mut self, tag: FontVariant)
	{
		// If there is at least 1 other token in the line
		if self.tokens.len() > 0
		{
			// Determine the type of the last tag in the line
			let last_index = self.tokens.len() - 1;
			match self.tokens[last_index]
			{
				// If it's a font tag
				Token::FontTag(_) => self.tokens[last_index] = Token::FontTag(tag),
				// If it's anything else
				_ =>
				{
					self.previous_font_variant = self.current_font_variant;
					self.tokens.push(Token::FontTag(tag));
				}
			}
		}
		// If this is the first token in the line, just add it to the line
		else { self.tokens.push(Token::FontTag(tag)); }
		self.current_font_variant = tag;
	}

	/// Adds text to the line.
	pub fn add_text(&mut self, text: TextToken)
	{
		// Update the current font variant so the line can know which font variants to use for width
		// calculations
		self.previous_font_variant = self.current_font_variant;
		// Adds the width and length of the token to the line's width and byte count before
		// adding the token itself to the line.
		self.width += text.width;
		self.byte_count += text.byte_count();
		// Add to line
		self.tokens.push(Token::Text(text));
	}

	/// Adds a space character to the line.
	pub fn add_space(&mut self, space_widths: &SpaceWidths)
	{
		// Adds the width and length of the token to the line's width and byte count before
		// adding the token itself to the line.
		self.width += space_widths.get_width_for(self.text_type, self.previous_font_variant);
		self.byte_count += SPACE.len();
		// Add to line
		self.tokens.push(Token::Space);
	}

	// /// Adds extra width to the line (usually used for adding the width of a space character to the line).
	// pub fn add_width(&mut self, width: f32) { self.width += width; }
	/// Shrinks the capacity of the vec of tokens to fit its size.
	pub fn shrink_to_fit(&mut self) { self.tokens.shrink_to_fit(); }

	// Getters

	/// Returns the vec of all the tokens in the line.
	pub fn tokens(&self) -> &Vec<Token> { &self.tokens }
	/// Returns the width of the line.
	pub fn width(&self) -> f32 { self.width }
	/// Returns the number of bytes that the line's chars will take up in string form
	pub fn byte_count(&self) -> usize { self.byte_count }
	// /// Returns the number of tokens in the line
	// pub fn len(&self) -> usize { self.tokens.len() }
	/// Returns whether or not the vec of tokens in this line is empty.
	pub fn is_empty(&self) -> bool { self.tokens.is_empty() }
	// /// Returns whether or not the vec of tokens in this line is not empty.
	// pub fn not_empty(&self) -> bool { self.tokens.len() > 0 }

	/// Returns the space width using the font data of the previous token
	pub fn get_last_space_width(&self, space_widths: &SpaceWidths) -> f32
	{
		space_widths.get_width_for(self.text_type, self.previous_font_variant)
	}
}

/// Keeps track of the width of spaces in spellbooks using `printpdf::Mm` units.
#[derive(Clone, Debug, PartialEq)]
pub struct SpaceWidths
{
	// Outer dimension represents font scales, inner dimension represents font variants
	widths: [[f32; FONTVARIANT_VARIANTS]; TEXTTYPE_VARIANTS]
}

/// Used for constructing empty width arrays in `SpaceWidths`.
const DEFAULT_WIDTHS: [f32; FONTVARIANT_VARIANTS] = [0.0; FONTVARIANT_VARIANTS];

impl SpaceWidths
{
	/// Constructs a new `SpaceWidths` object using font data.
	pub fn new(font_data: &FontData) -> Self
	{
		const TITLE: usize = TextType::Title as usize;
		const HEADER: usize = TextType::Header as usize;
		const BODY: usize = TextType::Body as usize;
		const TABLE_TITLE: usize = TextType::TableTitle as usize;
		const TABLE_BODY: usize = TextType::TableBody as usize;
		// Initialize an empty 2D array of widths
		let mut widths = [DEFAULT_WIDTHS; TEXTTYPE_VARIANTS];
		// Loop through each `TextType` variant to get the widths for each font variant with that text type's font
		// scale.
		// (use a loop in case the underlying numbers for `TextType` change)
		for i in 0..TEXTTYPE_VARIANTS
		{
			widths[i] = match i
			{
				TITLE =>
				Self::construct_widths_for(font_data.get_font_scale_for(TextType::Title), font_data),
				HEADER =>
				Self::construct_widths_for(font_data.get_font_scale_for(TextType::Header), font_data),
				BODY =>
				Self::construct_widths_for(font_data.get_font_scale_for(TextType::Body), font_data),
				TABLE_TITLE =>
				Self::construct_widths_for(font_data.get_font_scale_for(TextType::TableTitle), font_data),
				TABLE_BODY =>
				Self::construct_widths_for(font_data.get_font_scale_for(TextType::TableBody), font_data),
				_ => panic!("Invalid TextType variant / usize / index in `dnd_spellbook_maker::spellbook_gen_types::SpaceWidths::new`")
			}
		}
		SpaceWidths { widths: widths }
	}

	/// Gives the font widths for each font variant using a specific font scale.
	fn construct_widths_for(scale: &Scale, font_data: &FontData) -> [f32; FONTVARIANT_VARIANTS]
	{
		const REGULAR: usize = FontVariant::Regular as usize;
		const BOLD: usize = FontVariant::Bold as usize;
		const ITALIC: usize = FontVariant::Italic as usize;
		const BOLD_ITALIC: usize = FontVariant::BoldItalic as usize;
		// Initialize an empty array of widths
		let mut widths = DEFAULT_WIDTHS;
		// Loop through each `FontVariant` variant to get the widths for each one using the given font scale.
		// (use a loop in case the underlying numbers for `FontVariant` change)
		for i in 0..FONTVARIANT_VARIANTS
		{
			widths[i] = match i
			{
				REGULAR => calc_text_width
				(
					SPACE,
					font_data.get_size_data_for(FontVariant::Regular),
					scale,
					font_data.get_scalar_for(FontVariant::Regular)
				),
				BOLD => calc_text_width
				(
					SPACE,
					font_data.get_size_data_for(FontVariant::Bold),
					scale,
					font_data.get_scalar_for(FontVariant::Bold)
				),
				ITALIC => calc_text_width
				(
					SPACE,
					font_data.get_size_data_for(FontVariant::Italic),
					scale,
					font_data.get_scalar_for(FontVariant::Italic)
				),
				BOLD_ITALIC => calc_text_width
				(
					SPACE,
					font_data.get_size_data_for(FontVariant::BoldItalic),
					scale,
					font_data.get_scalar_for(FontVariant::BoldItalic)
				),
				_ => panic!("Invalid FontVariant / usize / index in `dnd_spellbook_maker::spellbook_gen_types::SpaceWidths::construct_widths_for`")
			}
		}
		widths
	}

	/// Gets the width of a space for a given `TextType` and `FontVariant`.
	pub fn get_width_for(&self, text_type: TextType, font_variant: FontVariant) -> f32
	{
		self.widths[text_type as usize][font_variant as usize]
	}

	// /// Gives all space width values in an unlabeled 2D array.
	// pub fn all_widths(&self) -> &[[f32; FONTVARIANT_VARIANTS]; TEXTTYPE_VARIANTS] { &self.widths }
}

/// Holds data about a column in a table in a spellbook.
#[derive(Clone, Debug, PartialEq)]
pub struct TableColumnData
{
	/// The starting x position of the text in the column.
	pub x_min: f32,
	/// The ending x position of the text in the column.
	pub x_max: f32,
	/// Whether or not the text in the column is centered.
	pub centered: bool
}

/// Calculates the width of some text based with given font data.
pub fn calc_text_width(text: &str, font_size_data: &Font, font_scale: &Scale, font_scalar: f32) -> f32
{
	let width = font_size_data.layout(text, *font_scale, point(0.0, 0.0))
		.map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
		.last()
		.unwrap_or(0.0);
	width * font_scalar
}

/// Calculates the height of some text based on given font data.
pub fn calc_text_height
(
	newline_amount: f32,
	lines: usize
)
-> f32
{
	// If there are no lines, return 0 for the height
	if lines == 0 { return 0.0; }
	// Calculate the amount of space every newline takes up
	let newlines_height = (lines as f32) * newline_amount;
	newlines_height
}

// /// Calculates the height of a single line of text using a certain font, scale, and size.
// pub fn line_height(font_size_data: &Font, font_scale: &Scale, font_size: f32) -> f32
// {
// 	// Calculate the value to scale the height of a single line of text by
// 	let font_scalar = font_size / 1000.0;
// 	// Calculate the height of a the lower half and the upper half of a line of text in this font
// 	let v_metrics = font_size_data.v_metrics(*font_scale);
// 	let line_height = (v_metrics.ascent - v_metrics.descent) * font_scalar;
// 	// Return the height of the line
// 	line_height
// }
