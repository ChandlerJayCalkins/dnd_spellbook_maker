//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//	Input types for generate_spellbook
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fmt;

pub use printpdf::{xobject::XObjectTransform, units::Mm};

/// Conveys which variant of a font is being used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum FontVariant
{
	Regular = 0,
	Bold = 1,
	Italic = 2,
	BoldItalic = 3
}
/// This must always be the same as the number of variants in `FontVariant`
pub const FONTVARIANT_VARIANTS: usize = 4;

impl fmt::Display for FontVariant
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		match self
		{
			Self::Regular => write!(f, "Regular"),
			Self::Bold => write!(f, "Bold"),
			Self::Italic => write!(f, "Italic"),
			Self::BoldItalic => write!(f, "Bold Italic")
		}
	}
}

/// File paths to all the font files needed for `create_spellbook()`.
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
	pub fn new
	(
		title_font_size: f32,
		header_font_size: f32,
		body_font_size: f32, 
		table_title_font_size: f32,
		table_body_font_size: f32
	)
	-> Result<Self, String>
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
	pub fn new
	(
		tab_amount: f32,
		title_newline_amount: f32,
		header_newline_amount: f32,
		body_newline_amount: f32,
		table_title_newline_amount: f32,
		table_body_newline_amount: f32
	)
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
pub struct TextColorOptions
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
	pub fn new
	(
		width: f32,
		height: f32,
		left_margin: f32,
		right_margin: f32,
		top_margin: f32,
		bottom_margin: f32
	)
	-> Result<Self, String>
	{
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
		else if left_margin <= 0.0 || right_margin <= 0.0 || left_margin + right_margin >= width
		{
			// Return an error
			Err(String::from("Invalid horizontal page margin."))
		}
		// If either vertical margin is below 0 or they are combined too big for there to be any text on the page
		else if top_margin <= 0.0 || bottom_margin <= 0.0 || top_margin + bottom_margin >= height
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
	starting_num: i64,
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
	pub fn new
	(
		starting_side: HSide,
		flips_sides: bool,
		starting_num: i64,
		font_variant: FontVariant,
		font_size: f32,
		newline_amount: f32,
		color: (u8, u8, u8),
		side_margin: f32,
		bottom_margin: f32
	)
	-> Result<Self, String>
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
	pub fn starting_num(&self) -> i64 { self.starting_num }
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
	pub fn new
	(
		horizontal_cell_margin: f32,
		vertical_cell_margin: f32,
		outer_horizontal_margin: f32,
		outer_vertical_margin: f32,
		off_row_color_lines_y_adjust_scalar: f32,
		off_row_color_lines_height_scalar: f32,
		off_row_color: (u8, u8, u8)
	)
	-> Result<Self, String>
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
