//! Library for making pdf documents of spells that a 5th edition D&D character has.
//!
//! See repository for documentation on spell files.
//!
//! Repository at <https://github.com/ChandlerJayCalkins/dnd_spellbook_maker>.

use std::fs;
//extern crate image;
use image::DynamicImage;
pub use printpdf::{Mm, PdfDocumentReference, ImageTransform, ImageRotation};
use printpdf::{PdfDocument, PdfLayerReference, IndirectFontRef, Color, Rgb, Point, Line, PdfPageIndex, Image};
use rusttype::{Font, Scale, point};
pub mod spells;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//	Input types for generate_spellbook
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Conveys which variant of a font is being used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontVariant
{
	Regular,
	Bold,
	Italic,
	BoldItalic
}

/// File paths to all the font files needed for `generate_spellbook()`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontPaths
{
	pub regular: String,
	pub bold: String,
	pub italic: String,
	pub bold_italic: String
}

/// Data for what font sizes to use and how large tabs and various newline sizes should be.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FontSizes
{
	title_font_size: f32,
	header_font_size: f32,
	body_font_size: f32,
	table_title_font_size: f32,
	table_body_font_size: f32
}

impl FontSizes
{
	/// Constructor
	///
	/// # Parameters
	///
	/// - `title_font_size` Cover page text font size.
	/// - `header_font_size` Spell name font size.
	/// - `body_font_size` Spell fields and description font size.
	/// - `table_title_font_size` Table header text font size.
	/// - `table_body_font_size` Table cell text font size.
	///
	/// # Output
	///
	/// - `Ok` A `FontSizes` object.
	/// - `Err` An error message saying which parameter was invalid. Occurs for negative values.
	pub fn new(title_font_size: f32, header_font_size: f32, body_font_size: f32, table_title_font_size: f32,
	table_body_font_size: f32, page_number_font_size: f32) -> Result<Self, String>
	{
		// Makes sure no values are below 0
		if title_font_size < 0.0 { Err(String::from("Invalid title_font_size.")) }
		else if header_font_size < 0.0 { Err(String::from("Invalid header_font_size.")) }
		else if body_font_size < 0.0 { Err(String::from("Invalid body_font_size.")) }
		else if table_title_font_size < 0.0 { Err(String::from("Invalid table_title_font_size.")) }
		else if table_body_font_size < 0.0 { Err(String::from("Invalid table_body_font_size.")) }
		else
		{
			Ok(Self
			{
				title_font_size: title_font_size,
				header_font_size: header_font_size,
				body_font_size: body_font_size,
				table_title_font_size: table_title_font_size,
				table_body_font_size: table_body_font_size
			})
		}
	}

	// Getters

	pub fn title_font_size(&self) -> f32 { self.title_font_size }
	pub fn header_font_size(&self) -> f32 { self.header_font_size }
	pub fn body_font_size(&self) -> f32 { self.body_font_size }
	pub fn table_title_font_size(&self) -> f32 { self.table_title_font_size }
	pub fn table_body_font_size(&self) -> f32 { self.table_body_font_size }
}

/// Scalar values to convert rusttype font units to printpdf millimeters (Mm).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FontScalars
{
	regular: f32,
	bold: f32,
	italic: f32,
	bold_italic: f32
}

impl FontScalars
{
	/// Constructor
	///
	/// # Parameters
	///
	/// - `regular` Scalar value for regular font.
	/// - `bold` Scalar value for bold font.
	/// - `italic` Scalar value for italic font.
	/// - `bold_italic` Scalar value for bold-italic font.
	///
	/// # Output
	///
	/// - `Ok` A `FontScalar` object.
	/// - `Err` An error message saying which parameter was invalid. Occurs for negative values.
	pub fn new(regular: f32, bold: f32, italic: f32, bold_italic: f32) -> Result<Self, String>
	{
		// Makes sure no values are below 0
		if regular < 0.0 { Err(String::from("Invalid regular scalar.")) }
		else if bold < 0.0 { Err(String::from("Invalid bold scalar.")) }
		else if italic < 0.0 { Err(String::from("Invalid italic scalar.")) }
		else if bold_italic < 0.0 { Err(String::from("Invalid bold_italic scalar.")) }
		else
		{
			Ok(Self
			{
				regular: regular,
				bold: bold,
				italic: italic,
				bold_italic: bold_italic
			})
		}
	}

	// Getters

	pub fn regular_scalar(&self) -> f32 { self.regular }
	pub fn bold_scalar(&self) -> f32 { self.bold }
	pub fn italic_scalar(&self) -> f32 { self.italic }
	pub fn bold_italic_scalar(&self) -> f32 { self.bold_italic }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SpacingOptions
{
	tab_amount: f32,
	title_newline_amount: f32,
	header_newline_amount: f32,
	body_newline_amount: f32,
	table_title_newline_amount: f32,
	table_body_newline_amount: f32
}

impl SpacingOptions
{
	/// Constructor
	///
	/// Parameters
	///
	/// - `tab_amount` Tab size in printpdf Mm.
	/// - `title_newline_amount` Newline size for title text in printpdf Mm.
	/// - `header_newline_amount` Newline size for spell header text in printpdf Mm.
	/// - `body_newline_amount` Newline size for spell fields and description in printpdf Mm.
	/// - `table_title_newline_amount` Newline size for table title text in printpdf Mm.
	/// - `table_body_newline_amount` Newline size for table cell text in printpdf Mm.
	///
	/// Output
	///
	/// - `Ok` A `SpacingOptions` object.
	/// - `Err` An error message saying which parameter was invalid. Occurs for negative values.
	pub fn new(tab_amount: f32, title_newline_amount: f32, header_newline_amount: f32, body_newline_amount: f32,
	table_title_newline_amount: f32, table_body_newline_amount: f32)
	-> Result<Self, String>
	{
		// Makes sure no values are below 0
		if tab_amount < 0.0 { Err(String::from("Invalid tab_amount.")) }
		else if title_newline_amount < 0.0 { Err(String::from("Invalid title_newline_amount.")) }
		else if header_newline_amount < 0.0 { Err(String::from("Invalid header_newline_amount.")) }
		else if body_newline_amount < 0.0 { Err(String::from("Invalid body_newline_amount.")) }
		else if table_title_newline_amount < 0.0 { Err(String::from("Invalid table_title_newline_amount.")) }
		else if table_body_newline_amount < 0.0 { Err(String::from("Invalid table_body_newline_amount.")) }
		else
		{
			Ok(Self
			{
				tab_amount: tab_amount,
				title_newline_amount: title_newline_amount,
				header_newline_amount: header_newline_amount,
				body_newline_amount: body_newline_amount,
				table_title_newline_amount: table_title_newline_amount,
				table_body_newline_amount: table_body_newline_amount
			})
		}
	}

	// Getters

	pub fn tab_amount(&self) -> f32 { self.tab_amount }
	pub fn title_newline_amount(&self) -> f32 { self.title_newline_amount }
	pub fn header_newline_amount(&self) -> f32 { self.header_newline_amount }
	pub fn body_newline_amount(&self) -> f32 { self.body_newline_amount }
	pub fn table_title_newline_amount(&self) -> f32 { self.table_title_newline_amount }
	pub fn table_body_newline_amount(&self) -> f32 { self.table_body_newline_amount }
}

/// RGB colors for types of text in the spellbook.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TextColors
{
	/// Cover page text.
	pub title_color: (u8, u8, u8),
	/// Spell name text.
	pub header_color: (u8, u8, u8),
	/// Spells fields and description.
	pub body_color: (u8, u8, u8),
	/// Title labels above tables in spell descriptions.
	pub table_title_color: (u8, u8, u8),
	/// Cell text in spell description tables.
	pub table_body_color: (u8, u8, u8)
}

/// Data for determining the size of the page and the margins between sides of the pages and text.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PageSizeOptions
{
	width: f32,
	height: f32,
	left_margin: f32,
	right_margin: f32,
	top_margin: f32,
	bottom_margin: f32
}

impl PageSizeOptions
{
	/// Constructor
	///
	/// # Parameters
	///
	/// - `width` Width of the page in printpdf Mm. Standard is 210.
	/// - `height` Height of the page in printpdf Mm. Standard is 297.
	/// - `left_margin` Space between text and left side of page.
	/// - `right_margin` Space between text and right side of page.
	/// - `top_margin` Space between text and top of page.
	/// - `bottom_margin` Space between text and bottom of page.
	///
	/// # Output
	///
	/// - `Ok` A PageSizeOptions object.
	/// - `Err` An error message saying which parameter(s) was / were invalid. Occurs for negative or overlapping values.
	pub fn new(width: f32, height: f32, left_margin: f32, right_margin: f32, top_margin: f32,
	bottom_margin: f32) -> Result<Self, String>
	{
		// Determines the minimum page dimension between width and height
		let min_dim = width.min(height);
		// If the width is below 0, return an error
		if width <= 0.0
		{
			Err(String::from("Invalid page width."))
		}
		// If the height is below 0, return an error
		else if height <= 0.0
		{
			Err(String::from("Invalid page height."))
		}
		// If either horizontal margin is below 0 or they are combined too big for there to be any text on the page
		else if left_margin <= 0.0 || right_margin <= 0.0 || left_margin + right_margin >= min_dim
		{
			// Return an error
			Err(String::from("Invalid horizontal page margin."))
		}
		// If either vertical margin is below 0 or they are combined too big for there to be any text on the page
		else if top_margin <= 0.0 || bottom_margin <= 0.0 || top_margin + bottom_margin >= min_dim
		{
			// Return an error
			Err(String::from("Invalid vertical page margin."))
		}
		// If it's all ok, construct and return
		else
		{
			Ok(Self
			{
				width: width,
				height: height,
				left_margin: left_margin,
				right_margin: right_margin,
				top_margin: top_margin,
				bottom_margin: bottom_margin
			})
		}
	}

	// Getters
	pub fn width(&self) -> f32 { self.width }
	pub fn height(&self) -> f32 { self.height }
	pub fn left_margin(&self) -> f32 { self.left_margin }
	pub fn right_margin(&self) -> f32 { self.right_margin }
	pub fn top_margin(&self) -> f32 { self.top_margin }
	pub fn bottom_margin(&self) -> f32 { self.bottom_margin }

	/// Returns whether or not all of the margins are equal for this object.
	pub fn has_same_margins(&self) -> bool
	{
		return self.left_margin == self.right_margin && self.left_margin == self.top_margin &&
			self.left_margin == self.bottom_margin
	}
}

/// Horizontal Side, used for determining the side of the page a page number goes on.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum HSide
{
	Left,
	Right
}

/// Allows usage of the `!` operator on `HSide`s.
impl std::ops::Not for HSide
{
	type Output = Self;

	/// Flips to the opposite side.
	fn not(self) -> Self::Output
	{
		match self
		{
			Self::Left => Self::Right,
			Self::Right => Self::Left
		}
	}
}

/// Parameters for determining page number behavior.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageNumberOptions
{
	starting_side: HSide,
	flips_sides: bool,
	starting_num: i32,
	font_variant: FontVariant,
	font_size: f32,
	newline_amount: f32,
	color: (u8, u8, u8),
	side_margin: f32,
	bottom_margin: f32
}

impl PageNumberOptions
{
	/// Constructor
	///
	/// # Parameters
	///
	/// - `starting_side` Whether or not the page numbers start on the left side.
	/// If the page numbers do not flip sides, this determines what side all page numbers are on.
	/// - `flips_sides` Whether or not the page numbers flip sides every page.
	/// - `starting_num` What number to have the page numbers start on for the first page.
	/// - `font_variant` The font variant of the page numbers (regular, bold, italic, bold-italic).
	/// - `font_size` The font size of the page numbers.
	/// - `newline_amount` The newline size for page numbers (in printpdf Mm) in case they overflow.
	/// - `color` The RGB value of the page numbers.
	/// - `side_margin` The distance between the page numbers and the side of the page.
	/// - `bottom_margin` The distance between the page numbers and the bottom of the page.
	///
	/// # Output
	///
	/// - `Ok` A PageNumberOptions object.
	/// - `Err` An error message saying which parameter was invalid. Occurs for negative margin values.
	pub fn new(starting_side: HSide, flips_sides: bool, starting_num: i32, font_variant: FontVariant, font_size: f32,
	newline_amount: f32, color: (u8, u8, u8), side_margin: f32, bottom_margin: f32) -> Result<Self, String>
	{
		if font_size < 0.0
		{
			Err(String::from("Invalid font size."))
		}
		else if newline_amount < 0.0
		{
			Err(String::from("Invalid newline amount."))
		}
		// If the side margin is less than 0, return an error
		else if side_margin < 0.0
		{
			Err(String::from("Invalid side margin."))
		}
		// If the bottom margin is less than 0, return an error
		else if bottom_margin < 0.0
		{
			Err(String::from("Invalid bottom margin."))
		}
		// If both of those values are ok, construct and return
		else
		{
			Ok(Self
			{
				starting_side: starting_side,
				flips_sides: flips_sides,
				starting_num: starting_num,
				font_variant: font_variant,
				font_size: font_size,
				newline_amount: newline_amount,
				color: color,
				side_margin: side_margin,
				bottom_margin: bottom_margin
			})
		}
	}

	// Getters
	pub fn starting_side(&self) -> HSide { self.starting_side }
	pub fn flips_sides(&self) -> bool { self.flips_sides }
	pub fn starting_num(&self) -> i32 { self.starting_num }
	pub fn font_variant(self) -> FontVariant { self.font_variant }
	pub fn font_size(&self) -> f32 { self.font_size }
	pub fn newline_amount(&self) -> f32 { self.newline_amount }
	pub fn color(&self) -> (u8, u8, u8) { self.color }
	pub fn side_margin(&self) -> f32 { self.side_margin }
	pub fn bottom_margin(&self) -> f32 { self.bottom_margin }
}

/// Options for tables.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TableOptions
{
	horizontal_cell_margin: f32,
	vertical_cell_margin: f32,
	outer_horizontal_margin: f32,
	outer_vertical_margin: f32,
	off_row_color_lines_y_adjust_scalar: f32,
	off_row_color_lines_height_scalar: f32,
	// RGB
	off_row_color: (u8, u8, u8)
}

impl TableOptions
{
	/// Constructor
	///
	/// # Parameters
	///
	/// - `horizontal_cell_margin` Space between columns in printpdf Mm.
	/// - `vertical_cell_margin` Space between rows in printpdf Mm.
	/// - `outer_horizontal_margin` Minimum space between sides of table and sides of pages.
	/// - `outer_vertical_margin` Space above and below table from other text / tables.
	/// - `off_row_color_lines_y_adjust_scalar` Scalar value to adjust off-row color lines to line up with the rows vertically.
	/// - `off_row_color_lines_height_scalar` Scalar value to determine the height of off-row color lines.
	/// - `off_row_color` RGB value of the color of the off-row color lines.
	///
	/// # Output
	///
	/// - `Ok` A TableOptions object.
	/// - `Err` An error message saying which parameter was invalid. Occurs for negative values.
	pub fn new(horizontal_cell_margin: f32, vertical_cell_margin: f32, outer_horizontal_margin: f32,
	outer_vertical_margin: f32, off_row_color_lines_y_adjust_scalar: f32, off_row_color_lines_height_scalar: f32,
	off_row_color: (u8, u8, u8)) -> Result<Self, String>
	{
		// Makes sure none of the float values are below 0
		if horizontal_cell_margin < 0.0 { Err(String::from("Invalid horizontal_cell_margin.")) }
		else if vertical_cell_margin < 0.0 { Err(String::from("Invalid vertical_cell_margin.")) }
		else if outer_horizontal_margin < 0.0 { Err(String::from("Invalid outer_horizontal_margin.")) }
		else if outer_vertical_margin < 0.0 { Err(String::from("Invalid outer_vertical_margin.")) }
		else if off_row_color_lines_y_adjust_scalar < 0.0
		{ Err(String::from("Invalid off_row_color_lines_y_adjust_scalar.")) }
		else if off_row_color_lines_height_scalar < 0.0
		{ Err(String::from("Invalid off_row_color_lines_height_scalar.")) }
		else
		{
			Ok(Self
			{
				horizontal_cell_margin: horizontal_cell_margin,
				vertical_cell_margin: vertical_cell_margin,
				outer_horizontal_margin: outer_horizontal_margin,
				outer_vertical_margin: outer_vertical_margin,
				off_row_color_lines_y_adjust_scalar: off_row_color_lines_y_adjust_scalar,
				off_row_color_lines_height_scalar: off_row_color_lines_height_scalar,
				off_row_color: off_row_color
			})
		}
	}

	// Getters

	pub fn horizontal_cell_margin(&self) -> f32 { self.horizontal_cell_margin }
	pub fn vertical_cell_margin(&self) -> f32 { self.vertical_cell_margin }
	pub fn outer_horizontal_margin(&self) -> f32 { self.outer_horizontal_margin }
	pub fn outer_vertical_margin(&self) -> f32 { self.outer_vertical_margin }
	pub fn off_row_color_lines_y_adjust_scalar(&self) -> f32 { self.off_row_color_lines_y_adjust_scalar }
	pub fn off_row_color_lines_height_scalar(&self) -> f32 { self.off_row_color_lines_height_scalar }
	// RGB
	pub fn off_row_color(&self) -> (u8, u8, u8) { self.off_row_color }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//	Input types for pdf writing functions
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO
// 1. Add TextColors struct to FontData struct.
// 2. Finish writing SpellbookWritier constrcutor and then write a test to make sure it works.
// 3. Rewrite `write_spell_description` function to be combined with `write_textbox` so tokens get parsed and written
// at the same time. Make it so text gets written when it either switches fonts or gets too long to fit on the page.

/// Conveys the type of text that is being used.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TextType
{
	Title,
	Header,
	Body,
	TableTitle,
	TableBody
}

/// Holds references to each font type of a font.
#[derive(Clone, Debug, PartialEq, Eq)]
struct FontRefs
{
	pub regular: IndirectFontRef,
	pub bold: IndirectFontRef,
	pub italic: IndirectFontRef,
	pub bold_italic: IndirectFontRef
}

/// Holds size data for each font type of a font.
#[derive(Clone, Debug)]
struct FontSizeData<'a>
{
	pub regular: Font<'a>,
	pub bold: Font<'a>,
	pub italic: Font<'a>,
	pub bold_italic: Font<'a>
}

/// Holds scale size data for each type of text.
#[derive(Clone, Debug, PartialEq)]
struct FontScales
{
	pub title: Scale,
	pub header: Scale,
	pub body: Scale,
	pub table_title: Scale,
	pub table_body: Scale
}

/// Keeps track of the current font variant being used, the current type of text, and other data needed to use fonts.
#[derive(Clone, Debug)]
struct FontData<'a>
{
	current_font_variant: FontVariant,
	current_text_type: TextType,
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
struct BytesToFontSizeDataConversionError(String);

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
	pub fn new(doc: &PdfDocumentReference, font_paths: FontPaths, font_sizes: FontSizes, font_scalars: FontScalars,
	spacing_options: SpacingOptions, text_colors: TextColors) -> Result<Self, Box<dyn std::error::Error>>
	{
		// Read the data from the font files
		let regular_font_bytes = fs::read(&font_paths.regular)?;
		let bold_font_bytes = fs::read(&font_paths.bold)?;
		let italic_font_bytes = fs::read(&font_paths.italic)?;
		let bold_italic_font_bytes = fs::read(&font_paths.bold_italic)?;

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
			font_refs: font_refs,
			font_sizes: font_sizes,
			scalars: font_scalars,
			size_data: size_data,
			scales: scales,
			spacing_options: spacing_options,
			text_colors: text_colors
		})
	}

	// Getters

	pub fn current_font_variant(&self) -> &FontVariant { &self.current_font_variant }
	pub fn current_text_type(&self) -> &TextType { &self.current_text_type }
	pub fn all_font_refs(&self) -> &FontRefs { &self.font_refs }
	pub fn all_font_sizes(&self) -> &FontSizes { &self.font_sizes }
	pub fn all_scalars(&self) -> &FontScalars { &self.scalars }
	pub fn all_size_data(&self) -> &FontSizeData { &self.size_data }
	pub fn all_scales(&self) -> &FontScales { &self.scales }
	pub fn all_spacing_options(&self) -> &SpacingOptions { &self.spacing_options }
	pub fn all_text_colors(&self) -> &TextColors { &self.text_colors }
	pub fn tab_amount(&self) -> f32 { self.spacing_options.tab_amount() }

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

	/// Returns the font color of the current text type being used.
	pub fn current_text_color(&self) -> &(u8, u8, u8)
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
struct PageSizeData
{
	width: f32,
	height: f32,
	// Left
	x_min: f32,
	// Right
	x_max: f32,
	// Bottom
	y_min: f32,
	// Top
	y_max: f32
}

/// Allows page limit coordinates to be constructed from the `PageSizeOptions` user input type.
impl From<PageSizeOptions> for PageSizeData
{
	/// Converts a `PageSizeOptions` object into a `PageSizeData` object for usage in spellbook writing.
	fn from(data: PageSizeOptions) -> Self
	{
		Self
		{
			width: data.width(),
			height: data.height(),
			x_min: data.left_margin(),
			x_max: data.width() - data.right_margin(),
			y_min: data.bottom_margin(),
			y_max: data.height() - data.top_margin()
		}
	}
}

impl PageSizeData
{
		// Getters

		pub fn width(&self) -> f32 { self.width }
		pub fn height(&self) -> f32 { self.height }
		/// Left
		pub fn x_min(&self) -> f32 { self.x_min }
		/// Right
		pub fn x_max(&self) -> f32 { self.x_max }
		/// Bottom
		pub fn y_min(&self) -> f32 { self.y_min }
		/// Top
		pub fn y_max(&self) -> f32 { self.y_max }
}

/// Holds all page number data needed for writing them into spellbooks.
#[derive(Clone, Debug)]
struct PageNumberData<'a>
{
	options: PageNumberOptions,
	current_side: HSide,
	font_ref: &'a IndirectFontRef,
	font_scalar: f32,
	font_size_data: &'a Font<'a>,
	font_scale: Scale
}

impl <'a> PageNumberData<'a>
{
	/// Constructor
	///
	/// # Parameters
	///
	/// - `options` Options for how the page numbers should be displayed.
	/// - `font_data` Data for how fonts are displayed in the spellbook.
	fn new(options: PageNumberOptions, font_data: &'a FontData<'_>) -> Self
	{
		// Gets copies of all of the font data the page numbers need based on the font variant they will use.
		let (font_ref, font_scalar, font_size_data) = match options.font_variant()
		{
			FontVariant::Regular =>
			(
				&font_data.all_font_refs().regular,
				font_data.all_scalars().regular_scalar(),
				&font_data.all_size_data().regular
			),
			FontVariant::Bold =>
			(
				&font_data.all_font_refs().bold,
				font_data.all_scalars().bold_scalar(),
				&font_data.all_size_data().bold
			),
			FontVariant::Italic =>
			(
				&font_data.all_font_refs().italic,
				font_data.all_scalars().italic_scalar(),
				&font_data.all_size_data().italic
			),
			FontVariant::BoldItalic =>
			(
				&font_data.all_font_refs().bold_italic,
				font_data.all_scalars().bold_italic_scalar(),
				&font_data.all_size_data().bold_italic
			)
		};

		// Calculates the font scale data based on the font size option.
		let font_scale = Scale::uniform(options.font_size());
		
		// Construct and return
		Self
		{
			options: options,
			current_side: options.starting_side(),
			font_ref: font_ref,
			font_scalar: font_scalar,
			font_size_data: font_size_data,
			font_scale: font_scale
		}
	}

	// Getters

	pub fn starting_side(&self) -> HSide { self.options.starting_side() }
	pub fn flips_sides(&self) -> bool { self.options.flips_sides() }
	pub fn starting_num(&self) -> i32 { self.options.starting_num() }
	pub fn font_variant(&self) -> FontVariant { self.options.font_variant() }
	pub fn font_size(&self) -> f32 { self.options.font_size() }
	pub fn newline_amount(&self) -> f32 { self.options.newline_amount() }
	pub fn color(&self) -> (u8, u8, u8) { self.options.color() }
	pub fn side_margin(&self) -> f32 { self.options.side_margin() }
	pub fn bottom_margin(&self) -> f32 { self.options.bottom_margin() }
	pub fn options(&self) -> &PageNumberOptions { &self.options }
	pub fn current_side(&self) -> HSide { self.current_side }
	pub fn font_ref(&self) -> &IndirectFontRef { self.font_ref }
	pub fn font_scalar(&self) -> f32 { self.font_scalar }
	pub fn font_size_data(&self) -> &Font { self.font_size_data }
	pub fn font_scale(&self) -> &Scale { &self.font_scale }

	// Setters

	/// Flips the horizontal side of the page (left or right) the page number is currently on.
	pub fn flip_side(&mut self) { self.current_side = !self.current_side; }
}

/// Holds the background image and the transform data for it (positioning, size, rotation, etc.)
#[derive(Clone, Copy, Debug, PartialEq)]
struct BackgroundImage<'a>
{
	pub image: &'a DynamicImage,
	pub transform: &'a ImageTransform
}

/// All data needed to write spells to a pdf document.
// Can't derive clone or debug unfortunately.
struct SpellbookWriter<'a>
{
	document: PdfDocumentReference,
	layers: Vec<PdfLayerReference>,
	current_layer_number: i32,
	layer_name_prefix: &'a str,
	font_data: FontData<'a>,
	page_size_data: PageSizeData,
	page_number_data: Option<PageNumberData<'a>>,
	background: Option<BackgroundImage<'a>>,
	table_options: TableOptions,
	// Current x position of text
	x: f32,
	// Current y position of text
	y: f32
}

impl <'a> SpellbookWriter<'a>
{
	/// Constructor
	///
	/// # Parameters
	///
	/// - `font_paths` File paths to all of the font variants (regular, bold, italic, bold-italic).
	/// - `font_sizes` Font sizes for each type of text in the spellbook (except page numbers).
	/// - `font_scalars` Scalar values to make sure text width can be calculated correctly for each font variant.
	/// - `spacing_options` Tab size and newline sizes for each type of text (except page numbers).
	/// - `text_colors` The RGB color values for each type of text (except page numbers).
	/// - `page_size_options` Page width, height, and margin values.
	/// - `page_number_options` Settings for how page numbers look (`None` for no page numbers).
	/// - `table_options` Sizing and color options for tables in spell descriptions.
	pub fn from_options(font_paths: FontPaths, font_sizes: FontSizes, font_scalars: FontScalars,
	spacing_options: SpacingOptions, text_colors: TextColors, page_size_options: PageSizeOptions,
	page_number_options: Option<PageNumberOptions>, table_options: TableOptions) -> Self
	{
		todo!()
	}

	// General Field Getters

	pub fn document(&self) -> &PdfDocumentReference { &self.document }
	pub fn layers(&self) -> &Vec<PdfLayerReference> { &self.layers }
	pub fn layer_name_prefix(&self) -> &str { self.layer_name_prefix }
	pub fn font_data(&self) -> &FontData { &self.font_data }
	pub fn background(&self) -> Option<BackgroundImage> { self.background }
	pub fn page_size_data(&self) -> &PageSizeData { &self.page_size_data }
	pub fn page_number_data(&self) -> &Option<PageNumberData> { &self.page_number_data }
	pub fn table_options(&self) -> &TableOptions { &self.table_options }
	/// Current x position of the text
	pub fn x(&self) -> &f32 { &self.x }
	/// Current y position of the text
	pub fn y(&self) -> &f32 { &self.y }

	// Layer Getters

	pub fn current_layer(&self) -> &PdfLayerReference
	{
		&self.layers.last().expect("Empty spellbook: no layers found. There should at least be a cover layer.")
	}

	// Font Getters

	/// The current font variant being used to write text (regular, bold, italic, bold-italic).
	pub fn current_font_variant(&self) -> &FontVariant { self.font_data.current_font_variant() }
	/// The current type of text being written.
	pub fn current_text_type(&self) -> &TextType { self.font_data.current_text_type() }
	/// `IndirectFontRefs` for each font variant (regular, bold, italic, bold-italic).
	pub fn all_font_refs(&self) -> &FontRefs { self.font_data.all_font_refs() }
	/// Font sizes for each type of text.
	pub fn all_font_sizes(&self) -> &FontSizes { self.font_data.all_font_sizes() }
	/// Scalar values for each font variant (regular, bold, italic, bold-italic).
	pub fn all_scalars(&self) -> &FontScalars { self.font_data.all_scalars() }
	/// Size data for each font variant (regular, bold, italic, bold-italic).
	pub fn all_size_data(&self) -> &FontSizeData { self.font_data.all_size_data() }
	/// Font scale sizing data for each type of text.
	pub fn all_scales(&self) -> &FontScales { self.font_data.all_scales() }
	/// All spacing options that were originally passed to this object.
	pub fn all_spacing_options(&self) -> &SpacingOptions { self.font_data.all_spacing_options() }
	/// RGB color values for each type of text.
	pub fn all_text_colors(&self) -> &TextColors { self.font_data.all_text_colors() }
	/// Tab size in pringpdf Mm.
	pub fn tab_amount(&self) -> f32 { self.font_data.tab_amount() }
	/// The font object for the current font variant being used.
	pub fn current_font_ref(&self) -> &IndirectFontRef { self.font_data.current_font_ref() }
	/// Font size of the current type of text being used.
	pub fn current_font_size(&self) -> f32 { self.font_data.current_font_size() }
	/// Scalar value of the current font variant being used (regular, bold, italic, bold-italic).
	pub fn current_scalar(&self) -> f32 { self.font_data.current_scalar() }
	/// Size data of the current font variant being used (regular, bold, italic, bold-italic).
	pub fn current_size_data(&self) -> &Font { self.font_data.current_size_data() }
	/// Scale sizing data of the current type of text being used.
	pub fn current_font_scale(&self) -> &Scale { self.font_data.current_font_scale() }
	/// Newline size in printpdf Mm of the current type of text being used.
	pub fn current_newline_amount(&self) -> f32 { self.font_data.current_newline_amount() }
	/// RGB color values for the current type of text being used.
	pub fn current_text_color(&self) -> &(u8, u8, u8) { self.font_data.current_text_color() }

	// Page Size Getters

	pub fn page_width(&self) -> f32 { self.page_size_data.width() }
	pub fn page_height(&self) -> f32 { self.page_size_data.height() }
	/// Left
	pub fn x_min(&self) -> f32 { self.page_size_data.x_min() }
	/// Right
	pub fn x_max(&self) -> f32 { self.page_size_data.x_max() }
	/// Bottom
	pub fn y_min(&self) -> f32 { self.page_size_data.y_min() }
	/// Top
	pub fn y_max(&self) -> f32 { self.page_size_data.y_max() }

	// Page Number Getters

	/// The side of the page (left or right) the page number starts on.
	pub fn starting_page_number_side(&self) -> Option<HSide>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.starting_side()),
			None => None
		}
	}

	/// Whether or not the page number flips sides every page.
	pub fn page_number_flips_sides(&self) -> Option<bool>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.flips_sides()),
			None => None
		}
	}

	/// The starting page number.
	pub fn starting_page_number(&self) -> Option<i32>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.starting_num()),
			None => None
		}
	}

	/// The font variant the page numbers use.
	pub fn page_number_font_variant(&self) -> Option<FontVariant>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.font_variant()),
			None => None
		}
	}

	/// The font size of the page numbers.
	pub fn page_number_font_size(&self) -> Option<f32>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.font_size()),
			None => None
		}
	}

	/// The amount of space between newlines for page numbers in case of overflow.
	pub fn page_number_newline_amount(&self) -> Option<f32>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.newline_amount()),
			None => None
		}
	}

	/// RGB color values for page numbers.
	pub fn page_number_color(&self) -> Option<(u8, u8, u8)>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.color()),
			None => None
		}
	}

	/// The amount of space between the side of the page and the page number in printpdf Mm.
	pub fn page_number_side_margin(&self) -> Option<f32>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.side_margin()),
			None => None
		}
	}
	
	/// The amount of space between the bottom of the page and the page number in printpdf Mm.
	pub fn page_number_bottom_margin(&self) -> Option<f32>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.bottom_margin()),
			None => None
		}
	}

	/// All of the original page number options that were inputted.
	pub fn page_number_options(&self) -> Option<&PageNumberOptions>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.options()),
			None => None
		}
	}

	/// The current side of the page (left or right) the page number is on.
	pub fn current_page_number_side(&self) -> Option<HSide>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.current_side()),
			None => None
		}
	}

	/// Returns the font ref to the current font type bring used for page numbers.
	pub fn page_number_font_ref(&self) -> Option<&IndirectFontRef>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.font_ref()),
			None => None
		}
	}

	/// Returns the scalar value of the font type being used for page numbers.
	pub fn page_number_font_scalar(&self) -> Option<f32>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.font_scalar()),
			None => None
		}
	}

	/// Returns the size data of the current font type being used for page numbers.
	pub fn page_number_font_size_data(&self) -> Option<&Font>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.font_size_data()),
			None => None
		}
	}

	/// The font scale size data for the page numbers.
	pub fn page_number_font_scale(&self) -> Option<&Scale>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.font_scale()),
			None => None
		}
	}

	// Table Getters

	/// Space between columns in printpdf Mm.
	pub fn table_horizontal_cell_margin(&self) -> f32 { self.table_options.horizontal_cell_margin() }
	/// Space between rows in printpdf Mm.
	pub fn table_vertical_cell_margin(&self) -> f32 { self.table_options.vertical_cell_margin() }
	/// Minimum space between sides of table and sides of pages in printpdf Mm.
	pub fn table_outer_horizontal_margin(&self) -> f32 { self.table_options.outer_horizontal_margin() }
	/// Space above and below table from other text / tables in printpdf Mm.
	pub fn table_outer_vertical_margin(&self) -> f32 { self.table_options.outer_vertical_margin() }
	/// Scalar value to adjust off-row color lines to line up with the rows vertically.
	pub fn table_off_row_color_lines_y_adjust_scalar(&self) -> f32
	{ self.table_options.off_row_color_lines_y_adjust_scalar() }
	/// Scalar value to determine the height of off-row color lines.
	pub fn table_off_row_color_lines_height_scalar(&self) -> f32 { self.table_options.off_row_color_lines_height_scalar() }
	// RGB value of the color of the off-row color lines.
	pub fn table_off_row_color(&self) -> (u8, u8, u8) { self.table_options.off_row_color() }

	// Font Setters

	/// Sets the current font variant that is being used to write text to the spellbook.
	pub fn set_current_font_variant(&mut self, font_type: FontVariant)
	{ self.font_data.set_current_font_variant(font_type); }
	/// Sets the current type of text that is being written to the spellbook.
	pub fn set_current_text_type(&mut self, text_type: TextType) { self.font_data.set_current_text_type(text_type); }

	// Page Number Setters

	/// Flips the side of the page that page numbers appear on.
	pub fn flip_page_number_side(&mut self)
	{
		match &mut self.page_number_data
		{
			Some(ref mut data) => data.flip_side(),
			None => ()
		}
	}
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//	Utility Functions
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Saves spellbooks to a file as a pdf document.
///
/// #### Parameters
/// - `doc` A spellbook that gets returned from `generate_spellbook()`.
/// - `file_name` The name to give to the file that the spellbook will be saved to.
/// 
/// #### Output
/// Returns a Result enum.
/// - `Ok` Returns nothing.
/// - `Err` Returns any errors that occurred.
pub fn save_spellbook(doc: PdfDocumentReference, file_name: &str) -> Result<(), Box<dyn std::error::Error>>
{
	let file = fs::File::create(file_name)?;
	doc.save(&mut std::io::BufWriter::new(file))?;
	Ok(())
}

/// Error for when a file name could not be retrieved when processing spell files in `get_all_spells_in_folder()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpellFileNameReadError;
// Makes the struct displayable
impl std::fmt::Display for SpellFileNameReadError
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(f, "Couldn't find a file name.")
	}
}
// Makes the struct officially an error
impl std::error::Error for SpellFileNameReadError {}

/// Returns a vec of spells from every spell file in a folder.
///
/// It only uses files that end in the `.spell` extension.
/// 
/// #### Parameters
/// - `folder_path` The file path to the folder to extract every spell from.
/// 
/// #### Output
/// Returns a Result enum.
/// - `Ok` Returns a vec of spell objects that can be inputted into `generate_spellbook()`.
/// - `Err` Returns any errors that occurred.
pub fn get_all_spells_in_folder(folder_path: &str) -> Result<Vec<spells::Spell>, Box<dyn std::error::Error>>
{
	// Gets a list of every file in the folder
	let file_paths = fs::read_dir(folder_path)?;
	// Create a list of the spells that will be returned (can't figure out how to count the number of files in file_paths to build vec with exact capacity)
	let mut spell_list = Vec::new();
	// Loop through each file in the folder
	for file_path in file_paths
	{
		// Attempt to get a path to the file in an option
		let file_name_option = file_path?.path();
		// Attempt to turn the path into a string
		let file_name = match file_name_option.to_str()
		{
			// If an str of the path was retrieved successfully, obtain it
			Some(name) => name,
			// If an str of the path could not be gotten, return an error
			None => return Err(Box::new(SpellFileNameReadError))
		};
		// If the file is a spell file
		if file_name.ends_with(".spell")
		{
			// Read the file, turn it into a spell, and push it to the spell_list vec
			spell_list.push(spells::Spell::from_file(file_name)?);
		}
	}
	// Return the list of spells
	Ok(spell_list)
}

/// Returns a vec of spells from every json spell file in a folder.
///
/// It only uses files that end in the `.json` extension.
/// 
/// #### Parameters
/// - `folder_path` The file path to the folder to extract every spell from.
/// 
/// #### Output
/// Returns a Result enum.
/// - `Ok` Returns a vec of spell objects that can be inputted into `generate_spellbook()`.
/// - `Err` Returns any errors that occurred.
pub fn get_all_json_spells_in_folder(folder_path: &str) -> Result<Vec<spells::Spell>, Box<dyn std::error::Error>>
{
	// Gets a list of every file in the folder
	let file_paths = fs::read_dir(folder_path)?;
	// Create a list of the spells that will be returned (can't figure out how to count the number of files in file_paths to build vec with exact capacity)
	let mut spell_list = Vec::new();
	// Loop through each file in the folder
	for file_path in file_paths
	{
		// Attempt to get a path to the file in an option
		let file_name_option = file_path?.path();
		// Attempt to turn the path into a string
		let file_name = match file_name_option.to_str()
		{
			// If an str of the path was retrieved successfully, obtain it
			Some(name) => name,
			// If an str of the path could not be gotten, return an error
			None => return Err(Box::new(SpellFileNameReadError))
		};
		// If the file is a json file
		if file_name.ends_with(".json")
		{
			// Read the file, turn it into a spell, and push it to the spell_list vec
			spell_list.push(spells::Spell::from_json_file(file_name)?);
		}
	}
	// Return the list of spells
	Ok(spell_list)
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//	Tests
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests
{
	use super::*;
	use std::path::Path;

	// Makes sure that creating valid spell files works
	#[test]
	fn create_spell_files()
	{
		// Path to hand-made spell files that are compared to generated spells
		let comparison_folder = String::from("spells/necronomicon/");

		// Create the spells (necronomicon spell duplicates)
		let hell_spell = spells::Spell
		{
			name: String::from("HELL SPELL AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A"),
			level: spells::SpellField::Custom(String::from("100TH-LEVEL")),
			school: spells::SpellField::Custom(String::from("SUPER NECROMANCY")),
			is_ritual: true,
			casting_time: spells::SpellField::Controlled(spells::CastingTime::Reaction(String::from("THAT YOU TAKE WHEN YOU FEEL LIKE CASTING SPELLS AND DOING MAGIC AND AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A"))),
			range: spells::SpellField::Controlled(spells::Range::Yourself(Some(spells::AOE::Cylinder(spells::Distance::Miles(63489), spells::Distance::Miles(49729))))),
			has_v_component: true,
			has_s_component: true,
			m_components: Some(String::from("UNLIMITED POWAHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H")),
			duration: spells::SpellField::Controlled(spells::Duration::Years(57394, true)),
			description: String::from("<ib> CASTING SPELLS AND CONJURING ABOMINATIONS <b> AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA <r> THIS SPELL ISN'T FOR <i> weak underpowered feeble wizards -_-. <r> THIS SPELL IS FOR ONLY THE MOST POWERFUL OF ARCHMAGES AND NECROMANCERS WHO CAN WIELD THE MIGHTIEST OF <bi> ARCANE ENERGY <r> WITH THE FORTITUDE OF A <ib> MOUNTAIN. <b> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
<table> <title> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A \\A \\\\A \\\\\\A \\<title> \\\\<title> \\\\\\<title> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A <title>
COLUMN OF CHAOS | COLUMN OF NECROMANCY
<row> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A | A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
<row> B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B | B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B | B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B
<row> POWER | WIZARDRY
<row> C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C | C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C
<table>
MORE MAGIC SPELLS AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
<table> <title> THIS TABLE AGAIN A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A \\A \\\\A \\\\\\A \\<title> \\\\<title> \\\\\\<title> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A <title>
COLUMN OF CHAOS | COLUMN OF NECROMANCY
<row> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A | A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
<row> B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B | B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B | B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B
<row> POWER | WIZARDRY
<row> C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C | C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C
<table>
YOU CAN'T HANDLE THIS SPELL A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A"),
			upcast_description: Some(String::from("HELL ON EARTH")),
			tables: Vec::new()
		};
		let power_word_scrunch = spells::Spell
		{
			name: String::from("Power Word Scrunch"),
			level: spells::SpellField::Controlled(spells::Level::Level9),
			school: spells::SpellField::Controlled(spells::MagicSchool::Transmutation),
			is_ritual: false,
			casting_time: spells::SpellField::Controlled(spells::CastingTime::Actions(1)),
			range: spells::SpellField::Controlled(spells::Range::Dist(spells::Distance::Feet(60))),
			has_v_component: true,
			has_s_component: false,
			m_components: None,
			duration: spells::SpellField::Controlled(spells::Duration::Instant),
			description: String::from("Choose 1 target creature or object within range. That target gets scrunched.
- Scrunching has these effects <table> <title> Scrunching Effects <title>
Target | Effect
<row> Creature | Flesh Ball
<row> Object | Ball of that object's material
<row> Creature not made of flesh | Ball of that creature's material
<table>
- Scrunch balls (balls produced from scrunching) can be thrown and do 1d6 bludgeoning damage on hit.
Scrunch ball funny lol."),
			upcast_description: None,
			tables: Vec::new()
		};
		let the_ten_hells = spells::Spell
		{
			name: String::from("The Ten Hells"),
			level: spells::SpellField::Controlled(spells::Level::Level9),
			school: spells::SpellField::Controlled(spells::MagicSchool::Necromancy),
			is_ritual: true,
			casting_time: spells::SpellField::Controlled(spells::CastingTime::Actions(1)),
			range: spells::SpellField::Controlled(spells::Range::Yourself(Some(spells::AOE::Sphere(spells::Distance::Feet(90))))),
			has_v_component: true,
			has_s_component: false,
			m_components: Some(String::from("the nail or claw of a creature from an evil plane")),
			duration: spells::SpellField::Controlled(spells::Duration::Instant),
			description: String::from("Choose any number of creatures made of tangible matter within range. Those creatures must all make a constitution savint throw against your spell save DC. All creatures that fail this saving throw get turned inside out, immediately die, and have their souls eternally damned to all nine hells simultaneously.
Creatures that succeed the saving throw take 20d4 scrunching damage."),
			upcast_description: None,
			tables: Vec::new()
		};

		// Create vec of test spells and their file names (without extension or path)
		let spell_list = vec![(hell_spell, "hell_spell"), (power_word_scrunch, "power_word_scrunch"), (the_ten_hells, "the_ten_hells")];
		// Test to make sure spell files can be created properly
		spell_file_test(&spell_list, true, "spells/tests/spell/", &comparison_folder);
		json_file_test(&spell_list, false, "spells/tests/json/", &comparison_folder);
	}

	// Creates spell files from a list of spells into the output folder and compares them to the same hand-crafted spells in the comparison folder
	fn spell_file_test(spell_list: &Vec<(spells::Spell, &str)>, compress: bool, output_folder: &str, comparison_folder: &str)
	{
		const FILE_EXTENSION: &str = ".spell";

		// If the output folder doesn't exist yet
		if !Path::new(&output_folder).exists()
		{
			// Create it
			fs::create_dir(&output_folder).unwrap();
		}

		// Create vec of file paths to each spell that is going to be generated
		let mut spell_paths = Vec::with_capacity(spell_list.len());
		for spell in spell_list
		{
			// Get file path to the spell that's about to be generated and add it to the vec
			let spell_path = output_folder.to_owned() + spell.1 + FILE_EXTENSION;
			spell_paths.push(spell_path.clone());
			// Generate the spell file for this spell
			spell.0.to_file(&spell_path, compress).unwrap();
		}

		// Create a list of just the spells and the file names from the spell_list parameter
		let real_spell_list: Vec<spells::Spell> = spell_list.into_iter().map(|(s, _)| s.clone()).collect();
		// Read all of the generated spell files into spell objects and put them into a list
		let test_spell_list = get_all_spells_in_folder(&output_folder).unwrap();
		// Compare if the spell objects are the same
		assert_eq!(real_spell_list, test_spell_list);

		// Loop through spell file paths
		for (spell, spell_path) in spell_list.iter().zip(spell_paths.iter())
		{
			// Read all of the bytes from the original spell that the generated one was based on
			let real_spell_bytes = fs::read(&(comparison_folder.to_owned() + spell.1 + FILE_EXTENSION)).unwrap();
			// Read all of the bytes from the generated spell file
			let test_spell_bytes = fs::read(&spell_path).unwrap();
			// Compare the bytes from both files to make sure they are the same
			assert_eq!(real_spell_bytes, test_spell_bytes);
		}
	}

	// Creates json files from a list of spells into the output folder and compares them to the same hand-crafted spells in the comparison folder
	fn json_file_test(spell_list: &Vec<(spells::Spell, &str)>, compress: bool, output_folder: &str, comparison_folder: &str)
	{
		const FILE_EXTENSION: &str = ".json";

		// If the output folder doesn't exist yet
		if !Path::new(&output_folder).exists()
		{
			// Create it
			fs::create_dir(&output_folder).unwrap();
		}

		// Create vec of file paths to each spell that is going to be generated
		let mut spell_paths = Vec::with_capacity(spell_list.len());
		for spell in spell_list
		{
			// Get file path to the spell that's about to be generated and add it to the vec
			let spell_path = output_folder.to_owned() + spell.1 + FILE_EXTENSION;
			spell_paths.push(spell_path.clone());
			// Generate the json file for this spell
			spell.0.to_json_file(&spell_path, compress).unwrap();
		}

		// Create a list of just the spells and the file names from the spell_list parameter
		let real_spell_list: Vec<spells::Spell> = spell_list.into_iter().map(|(s, _)| s.clone()).collect();
		// Read all of the generated spell files into spell objects and put them into a list
		let test_spell_list = get_all_json_spells_in_folder(&output_folder).unwrap();
		// Compare if the spell objects are the same
		assert_eq!(real_spell_list, test_spell_list);

		// Loop through spell file paths
		for (spell, spell_path) in spell_list.iter().zip(spell_paths.iter())
		{
			// Read all of the bytes from the original spell that the generated one was based on
			let real_spell_bytes = fs::read(&(comparison_folder.to_owned() + spell.1 + FILE_EXTENSION)).unwrap();
			// Read all of the bytes from the generated spell file
			let test_spell_bytes = fs::read(&spell_path).unwrap();
			// Compare the bytes from both files to make sure they are the same
			assert_eq!(real_spell_bytes, test_spell_bytes);
		}
	}

	// Creates 2 spellbooks that combined contain every spell from the official d&d 5e player's handbook
	#[test]
	fn players_handbook()
	{
		// Spellbook names
		let spellbook_name_1 = "Every Sepll in the 2014 Dungeons & Dragons 5th Edition Player's Handbook: Part 1";
		let spellbook_name_2 = "Every Sepll in the 2014 Dungeons & Dragons 5th Edition Player's Handbook: Part 2";
		// List of every spell in the player's handbook folder
		let spell_list = get_all_spells_in_folder("spells/players_handbook_2014").unwrap();
		// Split that vec into 2 vecs
		let spell_list_1 = spell_list[..spell_list.len()/2].to_vec();
		let spell_list_2 = spell_list[spell_list.len()/2..].to_vec();
		// File paths to the fonts needed
		let font_paths = FontPaths
		{
			regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
			bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
			italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
			bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
		};
		// Parameters for determining font sizes
		let font_sizes = FontSizes::new(32.0, 24.0, 12.0, 7.5, 12.0, 8.0, 5.0).unwrap();
		// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
		let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
		// Parameters for determining tab and newline sizes
		let spacing_options = SpacingOptions::new(7.5, 12.0, 8.0, 5.0).unwrap();
		// Colors for each type of text
		let text_colors = TextColors
		{
			title_color: (0, 0, 0),
			header_color: (115, 26, 26),
			body_color: (0, 0, 0)
		};
		// Parameters for determining the size of the page and the text margins on the page
		let page_size_options = PageSizeOptions::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
		// Parameters for determining page number behavior
		let page_number_options = PageNumberOptions::new(HSide::Left, false, 1, 5.0, 4.0).unwrap();
		// Parameters for table margins / padding and off-row color / scaling
		let table_options = TableOptions::new(16.0, 10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
		// File path to the background image
		let background_path = "img/parchment.jpg";
		// Image transform data for the background image
		let background_transform = ImageTransform
		{
			translate_x: Some(Mm(0.0)),
			translate_y: Some(Mm(0.0)),
			scale_x: Some(1.95),
			scale_y: Some(2.125),
			..Default::default()
		};
		// Creates the spellbooks
		let doc_1 = generate_spellbook
		(
			spellbook_name_1, &spell_list_1, &font_paths, &font_sizes, &font_scalars, &spacing_options, &text_colors,
			&page_size_options, &Some(page_number_options), &Some((background_path, &background_transform)),
			&table_options
		).unwrap();
		let doc_2 = generate_spellbook
		(spellbook_name_2, &spell_list_2, &font_paths, &font_sizes, &font_scalars, &spacing_options, &text_colors,
			&page_size_options, &Some(page_number_options), &Some((background_path, &background_transform)),
			&table_options
		).unwrap();
		// Saves the spellbooks as pdf documents
		let _ = save_spellbook(doc_1, "Player's Handbook 2014 Spells 1.pdf").unwrap();
		let _ = save_spellbook(doc_2, "Player's Handbook 2014 Spells 2.pdf").unwrap();
	}

	// Create a spellbook with every spell from the xanathar's guide to everything source book
	#[test]
	fn xanathars_guide_to_everything()
	{
		// Spellbook's name
		let spellbook_name = "Every Sepll in the Dungeons & Dragons 5th Edition Source Material Book \"Xanathar's Guide to Everything\"";
		// List of every spell in this folder
		let spell_list = get_all_spells_in_folder("spells/xanathars_guide_to_everything").unwrap();
		// File paths to the fonts needed
		let font_paths = FontPaths
		{
			regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
			bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
			italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
			bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
		};
		// Parameters for determining font sizes
		let font_sizes = FontSizes::new(32.0, 24.0, 12.0, 7.5, 12.0, 8.0, 5.0).unwrap();
		// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
		let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
		// Parameters for determining tab and newline sizes
		let spacing_options = SpacingOptions::new(7.5, 12.0, 8.0, 5.0).unwrap();
		// Colors for each type of text
		let text_colors = TextColors
		{
			title_color: (0, 0, 0),
			header_color: (115, 26, 26),
			body_color: (0, 0, 0)
		};
		// Parameters for determining the size of the page and the text margins on the page
		let page_size_options = PageSizeOptions::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
		// Parameters for determining page number behavior
		let page_number_options = PageNumberOptions::new(HSide::Left, false, 1, 5.0, 4.0).unwrap();
		// Parameters for table margins / padding and off-row color / scaling
		let table_options = TableOptions::new(16.0, 10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
		// File path to the background image
		let background_path = "img/parchment.jpg";
		// Image transform data for the background image
		let background_transform = ImageTransform
		{
			translate_x: Some(Mm(0.0)),
			translate_y: Some(Mm(0.0)),
			scale_x: Some(1.95),
			scale_y: Some(2.125),
			..Default::default()
		};
		// Creates the spellbook
		let doc = generate_spellbook
		(
			spellbook_name, &spell_list, &font_paths, &font_sizes, &font_scalars, &spacing_options, &text_colors,
			&page_size_options, &Some(page_number_options), &Some((background_path, &background_transform)),
			&table_options
		).unwrap();
		// Saves the spellbook to a pdf document
		let _ = save_spellbook(doc, "Xanathar's Guide to Everything Spells.pdf");
	}

	// Create a spellbook with every spell from the tasha's cauldron of everything source book
	#[test]
	fn tashas_cauldron_of_everything()
	{
		// Spellbook's name
		let spellbook_name = "Every Sepll in the Dungeons & Dragons 5th Edition Source Material Book \"Tasha's Cauldron of Everything\"";
		// List of every spell in this folder
		let spell_list = get_all_spells_in_folder("spells/tashas_cauldron_of_everything").unwrap();
		// File paths to the fonts needed
		let font_paths = FontPaths
		{
			regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
			bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
			italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
			bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
		};
		// Parameters for determining font sizes
		let font_sizes = FontSizes::new(32.0, 24.0, 12.0, 7.5, 12.0, 8.0, 5.0).unwrap();
		// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
		let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
		// Parameters for determining tab and newline sizes
		let spacing_options = SpacingOptions::new(7.5, 12.0, 8.0, 5.0).unwrap();
		// Colors for each type of text
		let text_colors = TextColors
		{
			title_color: (0, 0, 0),
			header_color: (115, 26, 26),
			body_color: (0, 0, 0)
		};
		// Parameters for determining the size of the page and the text margins on the page
		let page_size_options = PageSizeOptions::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
		// Parameters for determining page number behavior
		let page_number_options = PageNumberOptions::new(HSide::Left, false, 1, 5.0, 4.0).unwrap();
		// Parameters for table margins / padding and off-row color / scaling
		let table_options = TableOptions::new(16.0, 10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
		// File path to the background image
		let background_path = "img/parchment.jpg";
		// Image transform data for the background image
		let background_transform = ImageTransform
		{
			translate_x: Some(Mm(0.0)),
			translate_y: Some(Mm(0.0)),
			scale_x: Some(1.95),
			scale_y: Some(2.125),
			..Default::default()
		};
		// Creates the spellbook
		let doc = generate_spellbook
		(
			spellbook_name, &spell_list, &font_paths, &font_sizes, &font_scalars, &spacing_options, &text_colors,
			&page_size_options, &Some(page_number_options), &Some((background_path, &background_transform)),
			&table_options
		).unwrap();
		// Saves the spellbook to a pdf document
		let _ = save_spellbook(doc, "Tasha's Cauldron of Everything Spells.pdf");
	}

	// Create a spellbook with every spell from the strixhaven: a curriculum of chaos source book
	#[test]
	fn strixhaven()
	{
		// Spellbook's name
		let spellbook_name =
		"Every Sepll in the Dungeons & Dragons 5th Edition Source Material Book \"Strixhaven: A Curriculum of Chaos\"";
		// List of every spell in this folder
		let spell_list = get_all_spells_in_folder("spells/strixhaven").unwrap();
		// File paths to the fonts needed
		let font_paths = FontPaths
		{
			regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
			bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
			italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
			bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
		};
		// Parameters for determining font sizes
		let font_sizes = FontSizes::new(32.0, 24.0, 12.0, 7.5, 12.0, 8.0, 5.0).unwrap();
		// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
		let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
		// Parameters for determining tab and newline sizes
		let spacing_options = SpacingOptions::new(7.5, 12.0, 8.0, 5.0).unwrap();
		// Colors for each type of text
		let text_colors = TextColors
		{
			title_color: (0, 0, 0),
			header_color: (115, 26, 26),
			body_color: (0, 0, 0)
		};
		// Parameters for determining the size of the page and the text margins on the page
		let page_size_options = PageSizeOptions::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
		// Parameters for determining page number behavior
		let page_number_options = PageNumberOptions::new(HSide::Left, false, 1, 5.0, 4.0).unwrap();
		// Parameters for table margins / padding and off-row color / scaling
		let table_options = TableOptions::new(16.0, 10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
		// File path to the background image
		let background_path = "img/parchment.jpg";
		// Image transform data for the background image
		let background_transform = ImageTransform
		{
			translate_x: Some(Mm(0.0)),
			translate_y: Some(Mm(0.0)),
			scale_x: Some(1.95),
			scale_y: Some(2.125),
			..Default::default()
		};
		// Creates the spellbook
		let doc = generate_spellbook
		(
			spellbook_name, &spell_list, &font_paths, &font_sizes, &font_scalars, &spacing_options, &text_colors,
			&page_size_options, &Some(page_number_options), &Some((background_path, &background_transform)),
			&table_options
		).unwrap();
		// Saves the spellbook to a pdf document
		let _ = save_spellbook(doc, "Strixhaven A Curriculum of Chaos Spells.pdf");
	}

	// Stress testing the text formatting
	#[test]
	fn necronomicon()
	{
		// Spellbook's name
		let spellbook_name =
		"THE NECROBOMBINOMICON AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A";
		// List of every spell in the stress test folder
		let spell_list = get_all_spells_in_folder("spells/necronomicon").unwrap();
		// File paths to the fonts needed
		let font_paths = FontPaths
		{
			regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
			bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
			italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
			bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
		};
		// Parameters for determining font sizes
		let font_sizes = FontSizes::new(32.0, 24.0, 12.0, 7.5, 12.0, 8.0, 5.0).unwrap();
		// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
		let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
		// Parameters for determining tab and newline sizes
		let spacing_options = SpacingOptions::new(7.5, 12.0, 8.0, 5.0).unwrap();
		// Colors for each type of text
		let text_colors = TextColors
		{
			title_color: (0, 0, 0),
			header_color: (115, 26, 26),
			body_color: (0, 0, 0)
		};
		// Parameters for determining the size of the page and the text margins on the page
		let page_size_options = PageSizeOptions::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
		// Parameters for determining page number behavior
		let page_number_options = PageNumberOptions::new(HSide::Left, true, 1, 5.0, 4.0).unwrap();
		// Parameters for table margins / padding and off-row color / scaling
		let table_options = TableOptions::new(16.0, 10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
		// Creates the spellbook
		let doc = generate_spellbook
		(
			spellbook_name, &spell_list, &font_paths, &font_sizes, &font_scalars, &spacing_options, &text_colors,
			&page_size_options, &Some(page_number_options), None, &table_options
		).unwrap();
		// Saves the spellbook to a pdf document
		let _ = save_spellbook(doc, "NECRONOMICON.pdf");
	}

	// For creating spellbooks for myself and friends while I work on creating a ui to use this library
	// #[test]
	// fn create_spellbook()
	// {
	// 	// Spellbook's name
	// 	let spellbook_name = "A Spellcaster's Spellbook";
	// 	// Vec of spells that will be added to spellbook
	// 	let mut spell_list = Vec::new();
	// 	// Vec of paths to spell files that will be read from
	// 	let spell_paths = vec!
	// 	[
	// 		"spells/players_handbook_2014/prestidigitation.spell",
	// 		"spells/players_handbook_2014/mending.spell",
	// 		"spells/players_handbook_2014/mage_hand.spell",
	// 		"spells/players_handbook_2014/fire_bolt.spell",
	// 		"spells/strixhaven/silvery_barbs.spell",
	// 		"spells/players_handbook_2014/color_spray.spell",
	// 		"spells/players_handbook_2014/magic_missile.spell",
	// 		"spells/xanathars_guide_to_everything/ice_knife.spell",
	// 		"spells/players_handbook_2014/mage_armor.spell",
	// 		"spells/players_handbook_2014/unseen_servant.spell",
	// 		"spells/players_handbook_2014/detect_magic.spell",
	// 		"spells/players_handbook_2014/alarm.spell",
	// 		"spells/players_handbook_2014/cloud_of_daggers.spell",
	// 		"spells/players_handbook_2014/scorching_ray.spell"
	// 	];
	// 	// Attempt to loop through each spell file and convert it into a spell struct
	// 	for path in spell_paths
	// 	{
	// 		println!("{}", path);
	// 		// Convert spell file into spell struct and add it to spell_list vec
	// 		spell_list.push(spells::Spell::from_file(path).unwrap());
	// 	}
	// 	// File paths to the fonts needed
	// 	let font_paths = FontPaths
	// 	{
	// 		regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
	// 		bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
	// 		italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
	// 		bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
	// 	};
	// 	// Parameters for determining font sizes
	// 	let font_sizes = FontSizes::new(32.0, 24.0, 12.0, 7.5, 12.0, 8.0, 5.0).unwrap();
	// 	// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
	// 	let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
	// 	// Parameters for determining tab and newline sizes
	// 	let spacing_options = SpacingOptions::new(7.5, 12.0, 8.0, 5.0).unwrap();
	// 	// Colors for each type of text
	// 	let text_colors = TextColors
	// 	{
	// 		title_color: (0, 0, 0),
	// 		header_color: (115, 26, 26),
	// 		body_color: (0, 0, 0)
	// 	};
	// 	// Parameters for determining the size of the page and the text margins on the page
	// 	let page_size_options = PageSizeOptions::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
	// 	// Parameters for determining page number behavior
	// 	let page_number_options = PageNumberOptions::new(HSide::Left, false, 1, 5.0, 4.0).unwrap();
	// 	// Parameters for table margins / padding and off-row color / scaling
	// 	let table_options = TableOptions::new(16.0, 10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
	// 	// File path to the background image
	// 	let background_path = "img/parchment.jpg";
	// 	// Image transform data for the background image
	// 	let background_transform = ImageTransform
	// 	{
	// 		translate_x: Some(Mm(0.0)),
	// 		translate_y: Some(Mm(0.0)),
	// 		scale_x: Some(1.95),
	// 		scale_y: Some(2.125),
	// 		..Default::default()
	// 	};
	// 	// Creates the spellbook
	// 	let doc = generate_spellbook
	// 	(
	// 		spellbook_name, &spell_list, &font_paths, &font_sizes, &font_scalars, &spacing_options, &text_colors,
	// 		&page_size_options, &Some(page_number_options), &Some((background_path, &background_transform)),
	// 		&table_options
	// 	).unwrap();
	// 	// Saves the spellbook to a pdf document
	// 	let _ = save_spellbook(doc, "Spellbook.pdf");
	// }

	// #[test]
	// // Creates json files for every existing spell file except the spells in the necronomicon and test folders
	// fn convert_to_json()
	// {
	// 	let spell_folders = ["spells/players_handbook_2014", "spells/strixhaven", "spells/tashas_cauldron_of_everything", "spells/xanathars_guide_to_everything"];
	// 	for folder in spell_folders
	// 	{
	// 		// Gets a list of every file in the folder
	// 		let file_paths = fs::read_dir(folder).unwrap();
	// 		// Loop through each file in the folder
	// 		for file_path in file_paths
	// 		{
	// 			// Attempt to get a path to the file
	// 			let file_name = file_path.unwrap().path();
	// 			let file_name = file_name.to_str().unwrap();
	// 			// If the file is a spell file
	// 			if file_name.ends_with(".spell")
	// 			{
	// 				let spell = spells::Spell::from_file(file_name).unwrap();
	// 				let mut json_file_name = String::from(file_name);
	// 				json_file_name.truncate(json_file_name.len() - 5);
	// 				json_file_name += "json";
	// 				let _ = spell.to_json_file(&json_file_name, false);
	// 			}
	// 		}
	// 	}
	// }
}
