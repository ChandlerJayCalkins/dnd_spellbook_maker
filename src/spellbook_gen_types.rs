//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//	Input types for SpellbookWriter
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;
use std::{rc::Rc, cell::{RefCell, Ref}};
use std::error::Error;

pub use image::DynamicImage;
pub use rusttype::{Font, Scale};
pub use printpdf::{PdfDocumentReference, IndirectFontRef, Color, Rgb};

pub use crate::spellbook_options::*;

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
pub enum TextType
{
	Title,
	Header,
	Body,
	TableTitle,
	TableBody
}

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

		/// Put the bytes into a struct to reuse them if new font refs need to be created when a new pdf document is created.
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
	pub fn all_font_sizes(&self) -> &FontSizes { &self.font_sizes }
	pub fn all_scalars(&self) -> &FontScalars { &self.scalars }
	pub fn all_size_data(&self) -> &FontSizeData { &self.size_data }
	pub fn all_scales(&self) -> &FontScales { &self.scales }
	pub fn all_spacing_options(&self) -> &SpacingOptions { &self.spacing_options }
	pub fn all_text_colors(&self) -> &TextColors { &self.text_colors }
	pub fn tab_amount(&self) -> f32 { self.spacing_options.tab_amount() }

	/// Returns a vec of bytes that were used to construct certain fields for a specific font variant.
	pub fn get_bytes_for(&self, font_variant: FontVariant) -> &Vec<u8>
	{
		match font_variant
		{
			FontVariant::Regular => &self.font_bytes.regular,
			FontVariant::Bold => &self.font_bytes.bold,
			FontVariant::Italic => &self.font_bytes.italic,
			FontVariant::BoldItalic => &self.font_bytes.bold_italic
		}
	}

	/// Returns a vec of bytes that were used to construct certain fields for the current font variant.
	pub fn current_bytes(&self) -> &Vec<u8>
	{
		match self.current_font_variant
		{
			FontVariant::Regular => &self.font_bytes.regular,
			FontVariant::Bold => &self.font_bytes.bold,
			FontVariant::Italic => &self.font_bytes.italic,
			FontVariant::BoldItalic => &self.font_bytes.bold_italic
		}
	}

	/// Returns the font ref for a specific font variant.
	pub fn get_font_ref_for(&self, font_variant: FontVariant) -> &IndirectFontRef
	{
		match font_variant
		{
			FontVariant::Regular => &self.font_refs.regular,
			FontVariant::Bold => &self.font_refs.bold,
			FontVariant::Italic => &self.font_refs.italic,
			FontVariant::BoldItalic => &self.font_refs.bold_italic
		}
	}

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

	/// Returns the font size of a specific text type.
	pub fn get_font_size_for(&self, text_type: TextType) -> f32
	{
		match text_type
		{
			TextType::Title => self.font_sizes.title_font_size(),
			TextType::Header => self.font_sizes.header_font_size(),
			TextType::Body => self.font_sizes.body_font_size(),
			TextType::TableTitle => self.font_sizes.table_title_font_size(),
			TextType::TableBody => self.font_sizes.table_body_font_size()
		}
	}

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

	/// Returns the font the RGB values for the font color of a specific text type.
	pub fn get_text_color_for(&self, text_type: TextType) -> &Color
	{
		match text_type
		{
			TextType::Title => &self.text_colors.title_color,
			TextType::Header => &self.text_colors.header_color,
			TextType::Body => &self.text_colors.body_color,
			TextType::TableTitle => &self.text_colors.table_title_color,
			TextType::TableBody => &self.text_colors.table_body_color
		}
	}

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
		// Dimensions that text can fit inside
		pub fn text_width(&self) -> f32 { self.text_width }
		pub fn text_height(&self) -> f32 { self.text_height }
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

	pub fn starting_side(&self) -> HSide { self.options.starting_side() }
	pub fn flips_sides(&self) -> bool { self.options.flips_sides() }
	pub fn starting_num(&self) -> i64 { self.options.starting_num() }
	pub fn font_variant(&self) -> FontVariant { self.options.font_variant() }
	pub fn font_size(&self) -> f32 { self.options.font_size() }
	pub fn newline_amount(&self) -> f32 { self.options.newline_amount() }
	pub fn side_margin(&self) -> f32 { self.options.side_margin() }
	pub fn bottom_margin(&self) -> f32 { self.options.bottom_margin() }
	pub fn options(&self) -> &PageNumberOptions { &self.options }
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
