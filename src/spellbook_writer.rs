//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//	Spellbook PDF document generation
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::error::Error;

use printpdf::
{
	PdfDocument,
	text::TextItem,
	FontId,
	color::Color,
	graphics::{Point, Line, LinePoint},
	ops::{PdfPage, Op},
	units::Pt
};
use regex::{Regex, Match};

use crate::spellbook_gen_types::*;
use crate::spells;

const DEFAULT_SPELLBOOK_TITLE: &str = "Spellbook";
const TITLE_PAGE_NAME: &str = "Title Page";

const REGULAR_FONT_TAG: &str = "<r>";
const BOLD_FONT_TAG: &str = "<b>";
const ITALIC_FONT_TAG: &str = "<i>";
const BOLD_ITALIC_FONT_TAG: &str = "<bi>";
const ITALIC_BOLD_FONT_TAG: &str = "<ib>";

const REGULAR_TAG_NAME: &str = "r";
const BOLD_TAG_NAME: &str = "b";
const ITALIC_TAG_NAME: &str = "i";
const BOLD_ITALIC_TAG_NAME: &str = "bi";
const ITALIC_BOLD_TAG_NAME: &str = "ib";

const DOT: &str = "•";
const DOT_SPACE: &str = "• ";
const DASH: &str = "-";

/// All data needed to write spells to a pdf document.
#[derive(Clone, Debug)]
pub struct SpellbookWriter<'a>
{
	doc: PdfDocument,
	current_page_index: usize,
	current_page_num: i64,
	font_data: FontData<'a>,
	page_size_data: PageSizeData,
	page_number_data: Option<PageNumberData<'a>>,
	background: Option<BackgroundImage>,
	table_data: TableData,
	// Stored here so the width of various types of spaces doesn't need to be continually recalculated
	space_widths: SpaceWidths,
	// Regex patterns are stored since they consume lots of runtime being reconstructed continutally
	font_tag_regex: Regex,
	table_tag_regex: Regex,
	backslashes_regex: Regex,
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
		spells: &Vec<spells::Spell>,
		font_paths: FontPaths,
		font_sizes: FontSizes,
		font_scalars: FontScalars,
		spacing_options: SpacingOptions,
		text_colors: TextColorOptions,
		page_size_options: PageSizeOptions,
		page_number_options: Option<PageNumberOptions>,
		background: Option<(&str, XObjectTransform)>,
		table_options: TableOptions
	)
	-> Result<PdfDocument, Box<dyn Error>>
	{
		// Construct a spellbook writer
		let mut writer = SpellbookWriter::new
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
		// Turn the first page into the title page
		writer.make_title_page(title);
		// Add each spell to the spellbook
		for spell in spells { writer.add_spell(spell); }
		// Return the document that was created, its layers, and its pages
		Ok(writer.doc)
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
		text_colors: TextColorOptions,
		page_size_options: PageSizeOptions,
		page_number_options: Option<PageNumberOptions>,
		background: Option<(&str, XObjectTransform)>,
		table_options: TableOptions
	)
	-> Result<Self, Box<dyn Error>>
	{
		// Gets a new document and title page.
		let mut doc = PdfDocument::new(title);

		// Combined data for all font options along with font references to the pdf doc
		let font_data = FontData::new
		(
			&mut doc,
			font_paths,
			font_sizes,
			font_scalars,
			spacing_options,
			text_colors
		)?;

		// Data for text margins and page dimensions
		let page_size_data = PageSizeData::from(page_size_options);

		// Determine whether or not page numbers are desired
		let (page_number_data, starting_page_num) = match page_number_options
		{
			// If they are, then construct page number data from the options given
			Some(options) => (Some(PageNumberData::new(options, &font_data)?), options.starting_num()),
			// If no page number options were given, don't use page numbers
			None => (None, 1)
		};

		// Determine whether or not a background image is desired
		let background = match background 
		{
			// If it is, construct background image data from the options given
			Some((file_path, transform)) => Some(BackgroundImage::new(&mut doc, file_path, transform)?),
			// If no background image was given, don't use a background
			None => None
		};
		// Calculate the width of each variation of a space character
		let space_widths = SpaceWidths::new(&font_data);
		let table_data = TableData::from(table_options);
		// Create a regex pattern for font tags (that are not escaped)
		// Ex: "<r>", "<bi>", "<i>", "<b>", "<bi>"
		// let font_tag_pattern = format!
		// (
		// 	"(?:^|[^\\\\])({}|{}|{}|{}|{})",
		// 	REGULAR_FONT_TAG,
		// 	BOLD_FONT_TAG,
		// 	ITALIC_FONT_TAG,
		// 	BOLD_ITALIC_FONT_TAG,
		// 	ITALIC_BOLD_FONT_TAG
		// );
		// let font_tag_pattern = format!
		// (
		// 	"({}|{}|{}|{}|{})",
		// 	REGULAR_FONT_TAG,
		// 	BOLD_FONT_TAG,
		// 	ITALIC_FONT_TAG,
		// 	BOLD_ITALIC_FONT_TAG,
		// 	ITALIC_BOLD_FONT_TAG
		// );
		let font_tag_pattern = format!
		(
			"<({}|{}|{}|{}|{})>",
			REGULAR_TAG_NAME,
			BOLD_TAG_NAME,
			ITALIC_TAG_NAME,
			BOLD_ITALIC_TAG_NAME,
			ITALIC_BOLD_TAG_NAME
		);
		let font_tag_regex = Regex::new(&font_tag_pattern)
		.expect(format!
		(
			"Failed to build regex pattern \"{}\" in `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::new`",
			font_tag_pattern
		).as_str());
		// Create a regex pattern to find table tags which are used for inserting tables into spell descriptions
		// Ex: "[table][5]", "[table][0]", "[table][2]", etc.
		let table_tag_pattern = "\\[table\\]\\[[0-9]+\\]";
		let table_tag_regex = Regex::new(table_tag_pattern)
		.expect(format!
		(
			"Failed to build regex pattern \"{}\" in `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::new`",
			table_tag_pattern
		).as_str());
		// Create a regex pattern to find repeating backslashes which areused for finding escaped table tags
		let backslashes_pattern = "\\\\+";
		let backslashes_regex = Regex::new(backslashes_pattern)
		.expect(format!
		(
			"Failed to build regex pattern \"{}\" in `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::new`",
			backslashes_pattern
		).as_str());

		// Construct instance of self and return
		Ok(Self
		{
			doc: doc,
			current_page_index: 0,
			current_page_num: starting_page_num,
			font_data: font_data,
			page_size_data: page_size_data,
			page_number_data: page_number_data,
			background: background,
			space_widths: space_widths,
			table_data: table_data,
			font_tag_regex: font_tag_regex,
			table_tag_regex: table_tag_regex,
			backslashes_regex: backslashes_regex,
			x: page_size_data.x_min(),
			y: page_size_data.y_max()
		})
	}

	/// Turns the current page into a title page with the given title.
	fn make_title_page(&mut self, mut title: &str)
	{
		// Use the default spellbook title if none was given
		if title.is_empty() { title = DEFAULT_SPELLBOOK_TITLE; }
		// Create the actual title page
		self.make_new_page();
		// Create bookmark for title page
		self.doc.add_bookmark(TITLE_PAGE_NAME, self.current_page_index + 1);
		// Adds a background image to the page (if they are desired)
		self.add_background();
		// Store the page number data and set it to None so page numbers don't appear in any title pages created
		let page_number_data = self.page_number_data.clone();
		self.page_number_data = None;
		// Write the title to the page
		self.write_centered_textbox(title, self.x_min(), self.x_max(), self.y_bottom(), self.y_top());
		// Reset the page number data to what it was before
		self.page_number_data = page_number_data;
	}

	/// Adds a page / pages about a spell into the spellbook.
	fn add_spell(&mut self, spell: &spells::Spell)
	{
		// Make a new page for the spell
		self.make_new_page();
		// Add a bookmark for the first page of this spell
		self.doc.add_bookmark(&spell.name, self.current_page_index + 1);

		// Writes the spell name to the document
		self.set_current_text_type(TextType::Header);
		self.set_current_font_variant(FontVariant::Regular);
		self.x = self.x_min();
		self.y = self.y_top();
		self.write_textbox
		(&spell.name, self.x_min(), self.x_max(), self.y_bottom(), self.y_top(), false, &spell.tables);

		// Writes the level and school of the spell to the document
		self.y -= self.current_newline_amount();
		self.x = self.x_min();
		self.set_current_text_type(TextType::Body);
		self.set_current_font_variant(FontVariant::Italic);
		self.write_textbox
		(
			&spell.get_level_school_text(),
			self.x_min(),
			self.x_max(),
			self.y_bottom(),
			self.y_top(),
			false,
			&spell.tables
		);

		// Writes the casting time to the document
		self.y -= self.font_data.get_newline_amount_for(TextType::Header);
		self.x = self.x_min();
		self.set_current_font_variant(FontVariant::Bold);
		let casting_time = format!("Casting Time: <r> {}", spell.get_casting_time_text());
		self.write_textbox
		(&casting_time, self.x_min(), self.x_max(), self.y_bottom(), self.y_top(), false, &spell.tables);

		// Writes the range to the document
		self.y -= self.font_data.current_newline_amount();
		self.x = self.x_min();
		self.set_current_font_variant(FontVariant::Bold);
		let range = format!("Range: <r> {}", spell.range.to_string());
		self.write_textbox
		(&range, self.x_min(), self.x_max(), self.y_bottom(), self.y_top(), false, &spell.tables);

		// Writes the components to the document
		self.y -= self.font_data.current_newline_amount();
		self.x = self.x_min();
		self.set_current_font_variant(FontVariant::Bold);
		let components = format!("Components: <r> {}", spell.get_component_string());
		self.write_textbox
		(&components, self.x_min(), self.x_max(), self.y_bottom(), self.y_top(), false, &spell.tables);

		// Writes the duration to the document
		self.y -= self.font_data.current_newline_amount();
		self.x = self.x_min();
		self.set_current_font_variant(FontVariant::Bold);
		let duration = format!("Duration: <r> {}", &spell.duration.to_string());
		self.write_textbox
		(&duration, self.x_min(), self.x_max(), self.y_bottom(), self.y_top(), false, &spell.tables);

		// Get the upcast description prepared if there is one
		let upcast_description = if let Some(upcast_description) = &spell.upcast_description
		{
			// Adds different text at the start based on whether the spell is a cantrip or not
			let cantrip_upcast_prefix = "Cantrip Upgrade";
			let leveled_upcast_prefix = "Using a Higher-Level Spell Slot";
			let upcast_prefix = match &spell.level
			{
				spells::SpellField::Controlled(level) => match level
				{
					spells::Level::Cantrip => cantrip_upcast_prefix,
					_ => leveled_upcast_prefix
				},
				_ => leveled_upcast_prefix
			};
			// Create the upcast description with a newline and font tags
			format!("\n<bi> {}. <r> {}", upcast_prefix, &upcast_description)
		}
		else { String::new() };

		// Add the upcast description to the end of the rest of the spell description
		let description = format!("{}{}", &spell.description, upcast_description);
		
		// Writes the description to the document
		self.y -= self.font_data.get_newline_amount_for(TextType::Header);
		self.x = self.x_min();
		self.set_current_font_variant(FontVariant::Regular);
		self.write_textbox
		(&description, self.x_min(), self.x_max(), self.y_bottom(), self.y_top(), false, &spell.tables);
	}

	/// Writes text to the current page inside the given dimensions, starting at the x_min value and current y value.
	/// The text is left-aligned and if it goes below the y_min, it continues writing onto the next page (or a new
	/// page), continuing to stay within the given dimensions on the new page.
	/// `starting_tab` determines whether or not the first paragraph gets tabbed in on the first line or not.
	/// If `tables` is empty, table tags will be treated as normal tokens and parsed or skipped.
	/// This method can also process bullet points, tables, and font variant changes in the text.
	fn write_textbox
	(
		&mut self,
		text: &str,
		x_min: f32,
		x_max: f32,
		y_min: f32,
		y_max: f32,
		starting_tab: bool,
		tables: &Vec<spells::Table>
	)
	{
		// If either dimensional bounds overlap with each other, do nothing
		if x_min >= x_max || y_min >= y_max { return; }
		// Keeps track of whether or not a regular paragraph is currently being processed
		let mut in_paragraph = false;
		// Keeps track of whether or not a bullet point list is currently being processed
		let mut in_bullet_list = false;
		// Keeps track of whether or not a table is currently being processed
		let mut in_table = false;
		// The x position to reset the text to upon a newline (changes inside bullet lists)
		let mut x_reset = x_min;
		// The number of newlines to go down by at the start of a paragraph
		// Is 0.0 for the first paragraph (so the entire textbox doesn't get moved down by an extra newline)
		// Is 1.0 for all other paragraphs
		let mut paragraph_newline_scalar = 0.0;
		// The amount to tab the text in by at the start of a paragraph
		// Is 0.0 for the first non-bullet-point paragraph if `starting_tab` is false 
		// (to match the Player's Handbook formatting)
		// Is equal to `self.tab_amount()` for all other paragraphs
		let mut current_tab_amount = match starting_tab
		{
			true => self.tab_amount(),
			false => 0.0
		};
		// Split the text into paragraphs by newlines
		// Collects it into a vec so the `is_empty` method can be used without having to clone a new iterator.
		let paragraphs: Vec<_> = text.split('\n').collect();
		// If there is no text, do nothing
		if paragraphs.is_empty() { return; }
		// If there is text and the x position is beyond the x_max, reset the x position to x_min and go to a new line
		else if self.x > x_max { self.x = x_min; self.y -= self.current_newline_amount(); }
		// Loop through each paragraph
		for mut paragraph in paragraphs
		{
			// If a table was just being processed, move down an extra newline amount to keep the table separated
			// (to match the Player's Handbook Formatting)
			if in_table { self.y -= self.table_outer_vertical_margin(); }
			// Move the y position down by 0 or 1 newline amounts
			// 0 newlines for the first paragraph (so the entire textbox doesn't get moved down by an extra newline)
			// 1 newline for all other paragraphs
			else { self.y -= paragraph_newline_scalar * self.current_newline_amount(); }
			// Extract the first token from the paragraph to see if this paragraph is a bullet point or a table
			let (first_token, rest_of_paragraph) = match paragraph.split_once(char::is_whitespace)
			{
				Some((token_1, token_2)) => (token_1, token_2.trim()),
				None => (paragraph, "")
			};
			// If the paragraph starts with a bullet point symbol
			let lines = if first_token == DOT || first_token == DASH
			{
				// If this is the start of a bullet list (not currently in a bullet list and this is the first
				// bullet point)
				if !in_bullet_list
				{
					// Set the bullet point flag to signal that a bullet list is currently being processed
					in_bullet_list = true;
					// Zero the paragraph flag
					in_paragraph = false;
					// Set the value that the x position resets to so it lines up after the bullet point
					x_reset = self.calc_text_width(DOT_SPACE) + x_min;
					// If a table was being processed before, zero the table flag and don't go down annother extra
					// newline since that was already done above
					if in_table { in_table = false; }
					// If a table was not being processed before, move the y position down an extra newline amount
					else
					{
						// Move the y position down an extra newline amount to separate it from normal paragraphs
						// (to match the Player's Handbook formatting)
						// Moves the y position down 0 newlines on the first paragraph, 0 on all others.
						self.y -= paragraph_newline_scalar * self.current_newline_amount();
					}
				}
				// Reset the x position to the left side of the text box
				self.x = x_min;
				// Checks to see if the text should be applied to the next page or if a new page should be created.
				self.check_for_new_page();
				// Applies a bullet point to the page (using a dot even if a dash was used in the string)
				self.apply_text(DOT_SPACE);
				// Calculate the width that the rest of the text in the bullet point will have to fit inside
				let width = x_max - x_reset;
				// Get lines of the rest of the text in this bullet point
				self.get_textbox_lines(rest_of_paragraph, width, width)
			}
			else
			{
				// Determine whether the first token in this paragraph is a table tag or not
				match self.table_tag_check(first_token, tables.len())
				{
					// If the first token is a table tag, apply a table to the page and ignore following tokens in
					// this paragraph
					TableTagCheckResult::TableTag(table_index) =>
					{
						// If another table was not being processed before, move the y position down an extra
						// newline amount
						if !in_table
						{
							// Move the y position down an extra newline amount to separate it more from
							// normal paragraphs (to match the Player's Handbook formatting)
							// Moves the y position down 0 newlines on the first paragraph, 0 on all others.
							self.y -= paragraph_newline_scalar * self.current_newline_amount();
							// Set the table flag to signal that a table is being processed
							in_table = true;
						}
						// If this table is right after a bullet list (bullet flag still set)
						if in_bullet_list
						{
							// Set the value that the x position resets to so that it lines up with the left
							// side of the text box again
							x_reset = x_min;
							// Zero the bullet flag to signal that a bullet list isn't being currently
							// processed anymore
							in_bullet_list = false;
						}
						// Zero the paragraph flag
						in_paragraph = false;
						// Make it so the next paragraph after this doesn't get moved down an extra newline since
						// tables move the y position down the correct amount already
						paragraph_newline_scalar = 0.0;
						// Reset the x position to the left side of the textbox
						self.x = x_min;
						// TODO: Add code to put in a table
						self.write_table(&tables[table_index], x_min, x_max, y_min, y_max);
						// Skip the token loop below and move to the next paragraph
						continue;
					},
					// If this is an escaped table tag, remove the first backslash
					TableTagCheckResult::EscapedTableTag => paragraph = &paragraph[1..],
					// If this is not a table tag, do nothing
					_ => ()
				}
				// If this is a normal text paragraph
				// ...

				// If this paragraph is right after a bullet list (bullet flag still set)
				if in_bullet_list
				{
					// Move the y position down an extra newline amount to separate the bullet list from this
					// paragraph (to match the Player's Handbook formatting)
					self.y -= self.current_newline_amount();
					// Set the value that the x position resets to so that it lines up with the left side of the text
					// box again
					x_reset = x_min;
					// Zero the bullet flag to signal that a bullet list isn't being currently processed anymore
					in_bullet_list = false;
				}
				// Zero the table flag
				in_table = false;
				// Set the x position to be 0 or 1 tab amounts from the left side of the text box
				// 0 tab amounts for the first paragraph (to match the Player's Handbook formatting)
				// 1 tab amount for all other paragraphs
				self.x = x_min + current_tab_amount;
				// Set the paragraph flag
				in_paragraph = true;
				// Get the lines of text in this paragraph
				self.get_textbox_lines(paragraph, x_max - self.x, x_max - x_reset)
			};
			// Apply the lines of text of this paragraph to the spellbook
			self.apply_text_lines(&lines, x_reset);
			// Make it so all paragraphs after the first get moved down a newline amount before being processed
			paragraph_newline_scalar = 1.0;
			// If this was a paragraph, set the current tab amount to be the normal tab amount so all paragraphs
			// after the first are tabbed in on the first line
			if in_paragraph { current_tab_amount = self.tab_amount(); }
		}
		// If a table was the last thing that was applied to the page, move down an extra newline amount to keep
		// whatever comes next more separated from the table (to match the Player's Handbook formatting)
		if in_table { self.y -= self.current_newline_amount(); }
	}

	/// Returns whether a token is a table tag, an escaped table tag, or neither. Takes a token and the number of
	/// tables in the current spell as inputs.
	fn table_tag_check(&self, token: &str, table_count: usize) -> TableTagCheckResult
	{
		// If there is a table tag in this token (ex: "[table][5]", "[table][0]", etc.)
		if let Some(pat_match) = self.table_tag_regex.find(token)
		{
			// Get the index range of the table tag pattern patch
			let table_tag_range = pat_match.range();
			// If the table tag is at the end of the token
			if table_tag_range.end == token.len()
			{
				// Get a string slice of the table index (the 'x' in "[table][x]")
				let index_str = &token[table_tag_range.start + 8 .. token.len() - 1];
				// Convert the table index into a number
				let table_index = match index_str.parse::<usize>()
				{
					Ok(index) => index,
					// If the index wasn't a valid number, it's not a table token
					Err(_) => return TableTagCheckResult::NotTableTag
				};
				// If the table index is out of range, it's not a table token
				if table_index >= table_count { return TableTagCheckResult::NotTableTag; }
				// If the table tag is the whole token
				if table_tag_range.start == 0
				{
					// It's a table tag
					return TableTagCheckResult::TableTag(table_index);
				}
				// Check to see if this is an escaped table tag (a backslash or multiple backslashes
				// before the table tag)
				// If there is at least one backslash in the first token
				else if let Some(backslashes_match) = self.backslashes_regex.find(token)
				{
					// Get the index range of the backslash pattern match
					let backslashes_range = backslashes_match.range();
					// If the backslashes are at the start of the token and right before the table tag
					// (if the entire token is backslashes followed by a table tag)
					if backslashes_range.start == 0 && backslashes_range.end == table_tag_range.start
					{
						// It's an escaped table tag
						return TableTagCheckResult::EscapedTableTag;
					}
				}
			}
		}
		// In all other cases, it's not a table tag
		TableTagCheckResult::NotTableTag
	}

	/// Writes vertically and horizontally centered text into a fixed sized textbox.
	/// If the text is too big to fit in the textbox, it continues into the next page from the top of the page going
	/// to the bottom and staying within the same horizontal bounds.
	/// This method can also process font variant changes in the text.
	fn write_centered_textbox(&mut self, text: &str, x_min: f32, x_max: f32, y_min: f32, y_max: f32)
	{
		// If either dimensional bounds overlap with each other, do nothing
		if x_min >= x_max || y_min >= y_max { return; }
		// Calculates the width of the textbox to determine how many tokens can fit on each line
		let textbox_width = x_max - x_min;
		// Calculates the height of the textbox to determine where the text should go so it is vertically centered
		let textbox_height = y_max - y_min;
		// Split the text into lines that will fit horizontally within the textbox
		let lines = self.get_textbox_lines(text, textbox_width, textbox_width);
		// Calculate how many lines this text is going to be
		let max_lines = (textbox_height / self.current_newline_amount()).floor() as usize;
		// If There are more lines than can fit on the page, set the y value to the top of the textbox
		// (text on following pages will start at the top of the entire page but stay within the horizontal
		// boundries of the textbox)
		if lines.len() > max_lines { self.y = y_max; }
		// If all the lines can fit on one page, calculate what y value to start the text at so it is vertically
		// centered in the textbox and set the y value to that
		else { self.y = (y_max / 2.0) + (lines.len() - 1) as f32 / 2.0 * self.current_newline_amount(); }
		// Apply the text lines to the spellbook
		self.apply_centered_text_lines(&lines, x_min, x_max);
	}

	/// Parses a table and applies it to the spellbook.
	fn write_table(&mut self, table: &spells::Table, x_min: f32, x_max: f32, y_min: f32, y_max: f32)
	{
		let starting_text_type = *self.current_text_type();
		let starting_font_variant = *self.current_font_variant();
		// Set the text type to table body mode
		// No need to set the font variant, it resets at the start processing each cell
		self.set_current_text_type(TextType::TableBody);
		// Get the width of the widest cell in each column
		let max_column_widths = self.get_max_table_column_widths(&table.column_labels, &table.cells);
		// Calculate and assign widths to each column (as well as whether each column is centered or not)
		let column_width_data = self.get_table_column_width_data(&max_column_widths, x_min, x_max);
		// Calculate the width of the entire table
		let table_width = self.get_table_width(&column_width_data);
		// Get a vec of all data about columns needed for writing the table to the spellbook (computes x_min and
		// x_max values for each column and stores whether each column is centered or not)
		let column_data = self.get_column_data(&column_width_data, table_width);
		// Split each column label into lines that will fit within the width of their columns
		let column_label_lines =
		self.get_table_row_lines(&table.column_labels, &column_width_data, FontVariant::Bold);
		// Split each cell in the table into lines that will fit within the column each cell is in
		let cell_lines = self.get_table_cells_lines(&table.cells, &column_width_data);
		// Count the number of text lines in the column labels
		let label_line_count = self.get_line_count_for_row(&column_label_lines);
		// Count the number of text lines in each row in the table
		let cell_line_counts = self.get_table_row_line_counts(&cell_lines);
		// Calculate the height of the column label row
		let labels_height = self.calc_text_height(label_line_count);
		// Calculate the height of the each cell row in the table
		let row_heights = self.calc_table_row_heights(&cell_line_counts);
		// Change the text type and font variant to be in table title mode
		self.set_current_text_type(TextType::TableTitle);
		self.set_current_font_variant(FontVariant::Bold);
		// Split the table title into lines that will fit on the page
		let total_width = x_max - x_min;
		let title_lines = self.get_textbox_lines(&table.title, total_width, total_width);
		// Calculate the height of the title text (if there is any)
		let title_height =
		if title_lines.len() > 0 { self.calc_text_height(title_lines.len()) } else { 0.0 };
		// Calculates the height of the whole table to see if it can fit on the current page or even on a single page
		// Uses if-statements to add margin space between textboxes
		let table_height =
		title_height + if labels_height > 0.0 || cell_lines.len() > 0 { self.current_newline_amount() }
		else { 0.0 } + labels_height + row_heights.iter().sum::<f32>() +
		(((row_heights.len() - if labels_height > 0.0 {1} else {0}) as f32) * self.table_vertical_cell_margin());
		// Calculate the height of the entire page to use it to see if the table / title will fit on a single page
		let page_height = y_max - y_min;
		// If either the entire table or just the title can fit on a single page but not this page
		if (self.y - table_height < y_min && table_height <= page_height) ||
		(self.y - title_height < y_min && title_height <= page_height)
		{
			// Make a new page
			self.make_new_page();
			self.y = y_max;
		}
		// Apply the table to the spellbook
		self.apply_table
		(
			&title_lines,
			&column_label_lines,
			&cell_lines,
			&column_data,
			label_line_count,
			&cell_line_counts,
			x_min,
			x_max
		);
		// Reset the text type and font variant so it is the same as what it was before the table
		self.set_current_text_type(starting_text_type);
		self.set_current_font_variant(starting_font_variant);
	}

	/// Gets the widths of the widest cells in each column and returns those widths along with the index of the
	/// column that width belongs to so the vec can be sorted by width later and the widths can still be tracable
	/// to which column that is the width of.
	fn get_max_table_column_widths(&mut self, column_labels: &Vec<String>, cells: &Vec<Vec<String>>)
	-> Vec<(usize, f32)>
	{
		// Create a vec to hold the column widths and their associated indexes
		let mut column_widths = Vec::with_capacity(column_labels.len());
		// Loop through each column label to use its width as a starter value for the max width of that column
		for index in 0..column_labels.len()
		{
			// Set the font variant to bold for calculating the width of each column label
			// Reset it each time in case the font changes in one column label
			self.set_current_font_variant(FontVariant::Bold);
			// Calculate the width of that column label
			let width = self.calc_text_width(&column_labels[index]);
			// Add that width as a starter value for the max width of that column
			column_widths.push((index, width));
		}
		// Loop through each cell in the table to calculate its width and have it replace the max width of its column
		// if its bigger than the current max width of its column
		for row_index in 0..cells.len()
		{
			for column_index in 0..cells[row_index].len()
			{
				// Set the font variant to regular for the start of each cell
				self.set_current_font_variant(FontVariant::Regular);
				// Get a text line of this cell (or none if its empty)
				let cell_lines = self.get_textbox_lines
				(
					&cells[row_index][column_index],
					f32::INFINITY,
					f32::INFINITY
				);
				// Calculate the width of the cell (taking font switches into account) or use 0 if its empty
				let cell_width = if cell_lines.len() > 0 { cell_lines[0].width() } else { 0.0 };
				// If a max width for this column already exists
				if column_index < column_widths.len()
				{
					// Replace the max width of this column with this cell's width if its bigger than the current max
					// width of this column
					column_widths[column_index].1 = column_widths[column_index].1.max(cell_width);
				}
				// If this is a jagged table and a width hasn't been added for this column yet, push this width
				else { column_widths.push((column_index, cell_width)); }
			}
		}
		// Return the column widths and their associated indexes
		column_widths
	}

	/// Takes the widths of the widest cells in each column and the index of that column, returns a vec of structs
	/// that contain the width of each column and whether each column is centered or not.
	fn get_table_column_width_data
	(
		&self,
		max_column_widths: &Vec<(usize, f32)>,
		x_min: f32,
		x_max: f32
	)
	-> Vec<(f32, bool)>
	{
		// Keeps track of the number of columns in `usize` and `f32`
		let column_count = max_column_widths.len();
		let column_count_f32 = column_count as f32;
		// Vec that stores the data for each column (width and whether its centered or not)
		// It's pointless to use `default_column_width` as the default width value instead here of 0.0 in this vec
		// since `default_column_width` changes over the course of the loop and needs to be reassigned anyways
		let mut column_data = vec![(0.0, false); column_count];
		// Sort the max width of each column in order of least to greatest
		// MUST parse columns in order of thinnest to widest because the default column width widens as it goes, and
		// that might make it so a column that might've been made skinnier could've actually been wider if the
		// default column width was skinner than it when it was parsed and became wider than it afterwards
		let mut sorted_max_widths = max_column_widths.clone();
		sorted_max_widths.sort_by(|(_, a), (_, b)| a.partial_cmp(&b).expect(format!
		(
			"Failed to compare 2 `f32`s in `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::get_column_width_data`: {} and {}",
			a, b
		).as_str()));
		// Calculate the maximum width of a table within the given x and y boundries along with the outer margin
		// option
		let max_table_width = x_max - x_min - (self.table_outer_horizontal_margin() * 2.0);
		// The default column width that is used to determine the width of columns that are larger than their equal
		// share of the remaining width in the table (is the width of the table divided by the number of columns at
		// first)
		let mut default_column_width =
		(max_table_width - self.table_horizontal_cell_margin() * (column_count_f32 - 1.0)) / column_count_f32;
		// Keeps track of the width of the number of remaining columns while calculating column widths (until columns
		// that are wider than the default width are reached)
		let mut remaining_columns = column_count_f32 - 1.0;
		// Loop through each column max width in order of least to greatest to find the width of each column
		for (index, max_column_width) in sorted_max_widths
		{
			// If the column's widest cell is thinner than the default column width, use that max width for the
			// entire column's width
			if max_column_width < default_column_width
			{
				// Use the widest cell's width as the width for the whole column, and make the column have centered
				// text since it will only be 1 line
				column_data[index] = (max_column_width, true);
				// Increase the default column width by the amount of space that this column was given by default but
				// didn't use
				default_column_width += (default_column_width - max_column_width) / remaining_columns;
				// Decrease the number of columns left to find the width of (don't need to do this in the else
				// statement since this variable is only used for modifying the `default_column_width` variable)
				remaining_columns -= 1.0;
			}
			// If the column is wider than or as wide as the default column width at some point, give it the default
			// column width (default column width won't be affected once this point is reached since the widths that
			// are being iterated through are sorted)
			else { column_data[index].0 = default_column_width; }
		}
		// Return the data for this column
		column_data
	}

	/// Calculates the width of a table based on the width of its columns and the margin space between cells.
	fn get_table_width(&self, column_data: &Vec<(f32, bool)>) -> f32
	{
		// Adds up all of the column widths together
		let mut column_width_sum = 0.0;
		for column in column_data { column_width_sum += column.0; }
		// Returns the sum of the column widths plus the margin space between each cell
		column_width_sum + self.table_horizontal_cell_margin() * ((column_data.len() as f32) - 1.0)
	}

	/// Takes a vec of tuples containing column widths and bools of whether or not that column is centered, the width
	/// of the entire table, and returns a vec of data for each column (horizontal column bounds (x_min and x_max
	/// values) and the bool of whether or not that column has centered text).
	fn get_column_data(&self, column_width_data: &Vec<(f32, bool)>, table_width: f32)
	-> Vec<TableColumnData>
	{
		// Vec that holds the x_min and x_max values along with a bool that tells whether or not the column
		// text will be centered or not.
		let mut column_data = Vec::with_capacity(column_width_data.len());
		// Holds the x_min value for the next column
		// Starting value is where the text in the table should start to keep the table centered
		let mut current_x_min = (self.page_width() - table_width) / 2.0;
		// Loop through each column to calculate and store its x_min and x_max values
		for column in column_width_data
		{
			// Calculate the x_max value
			let x_max = current_x_min + column.0;
			// Store the x_min and x_max values for this column along with the bool for whether or not it's centered
			column_data.push(TableColumnData
			{
				x_min: current_x_min,
				x_max: x_max,
				centered: column.1
			});
			// Move the x_min value to the right for the next column
			current_x_min = x_max + self.table_horizontal_cell_margin();
		}
		// Return the column data
		column_data
	}

	/// Takes a 2D vec of cells from a table and the widths of each column in the table, divides each cell into
	/// lines, and returns a 3D vec of those lines for each cell along with the width of each line.
	fn get_table_cells_lines(&mut self, cells: &Vec<Vec<String>>, column_width_data: &Vec<(f32, bool)>)
	-> Vec<Vec<Vec<TextLine>>>
	{
		// Create the vec of lines to be returned along with their widths
		let mut lines: Vec<Vec<Vec<TextLine>>> = Vec::with_capacity(cells.len());
		// Loop through each row
		for table_row in cells
		{
			// Get the lines of each cell in this row
			let lines_in_row = self.get_table_row_lines(table_row, column_width_data, FontVariant::Regular);
			// Add the cell lines of this row to the vec to return
			lines.push(lines_in_row);
		}
		// Return the lines of each cell
		lines
	}

	/// Takes a Vec of cells from a table and the widths of each column in the table, divides each cell into lines
	/// that with within the bounds of that cell's column, and returns a 2D vec containing the lines of each cell.
	/// `start_font_variant` is what the current font variant gets set to at the start of each cell before it gets
	/// divided into lines so every cell can use the same default font variant.
	fn get_table_row_lines
	(
		&mut self,
		row: &Vec<String>,
		column_width_data: &Vec<(f32, bool)>,
		start_font_variant: FontVariant
	)
	-> Vec<Vec<TextLine>>
	{
		// Create a vec of the lines of each cell in the row
		let mut lines = Vec::with_capacity(row.len());
		// Set the font variant for this cell (only need to do it once since `get_textbox_lines` resets the font
		// variant itself at the end of the method)
		self.set_current_font_variant(start_font_variant);
		// Loop through each cell in the row
		for column_index in 0..row.len()
		{
			// Split this cell into lines and add its lines to the return vec
			lines.push(self.get_textbox_lines
			(
				&row[column_index],
				column_width_data[column_index].0,
				column_width_data[column_index].0
			));
		}
		// Return the cell lines in this row
		lines
	}

	/// Returns the number of lines in each row in a table. Used for calculating the height of a row.
	fn get_table_row_line_counts(&self, cells: &Vec<Vec<Vec<TextLine>>>) -> Vec<usize>
	{
		let mut row_line_counts = Vec::with_capacity(cells.len());
		for row in cells
		{
			row_line_counts.push(self.get_line_count_for_row(row));
		}
		row_line_counts
	}

	/// Returns the number of lines in a row.
	fn get_line_count_for_row(&self, row: &Vec<Vec<TextLine>>) -> usize
	{
		let mut max_lines = 0;
		for cell in row
		{
			max_lines = std::cmp::max(cell.len(), max_lines);
		}
		max_lines
	}

	/// Calculates the height of each row in a table and returns the height for each of those rows.
	fn calc_table_row_heights(&self, row_line_counts: &Vec<usize>) -> Vec<f32>
	{
		// Keeps track of the height of each row
		let mut row_heights = Vec::with_capacity(row_line_counts.len());
		// Add the height of each row to the return vec
		for line_count in row_line_counts
		{
			row_heights.push(self.calc_text_height(*line_count));
		}
		row_heights
	}

	/// Applies a parsed table to the spellbook.
	fn apply_table
	(
		&mut self,
		title_lines: &Vec<TextLine>,
		column_label_lines: &Vec<Vec<TextLine>>,
		cell_lines: &Vec<Vec<Vec<TextLine>>>,
		column_data: &Vec<TableColumnData>,
		label_line_count: usize,
		row_line_counts: &Vec<usize>,
		x_min: f32,
		x_max: f32
	)
	{
		// If there's no column data, no nothing
		if column_data.len() < 1 { return; }
		let color_line_x_min = column_data[0].x_min - self.table_outer_horizontal_margin();
		let color_line_x_max = column_data[column_data.len() - 1].x_max + self.table_outer_horizontal_margin();
		// Reset font settings in case it changed in the middle of the title
		self.set_current_text_type(TextType::TableTitle);
		self.set_current_font_variant(FontVariant::Bold);
		// Write the title text to the spellbook
		self.apply_centered_text_lines(title_lines, x_min, x_max);
		// If there are no table cells or column labels, do nothing else
		if cell_lines.len() < 1 && column_label_lines.len() < 1 { return; }
		// Move the y position down from the title to the top of the table
		else if title_lines.len() > 0 { self.y -= self.table_vertical_cell_margin(); }
		// Go into table body text mode
		self.set_current_text_type(TextType::TableBody);
		// Save the current page index and y value so they can be reset after the color lines are applied
		let starting_page_index = self.current_page_index();
		let starting_y = self.y;
		// Apply the off row color lines
		self.apply_table_color_lines(label_line_count, row_line_counts, color_line_x_min, color_line_x_max);
		// Set the page index and y value back to what they were at the top of the table
		self.current_page_index = starting_page_index;
		self.y = starting_y;
		// Apply the text inside the cells to the spellbook
		self.apply_table_cells(column_label_lines, cell_lines, column_data);
	}

	/// Applies background color lines to every other row in a table.
	fn apply_table_color_lines
	(
		&mut self,
		label_line_count: usize,
		row_line_counts: &Vec<usize>,
		x_min: f32,
		x_max: f32
	)
	{
		// Keeps track of whether or not to put a line on this row (true to put a line)
		let mut off_row = false;
		// Moves the y position by a bit when a line is applied
		let y_adjuster = self.current_font_size() * self.table_off_row_color_lines_y_adjust_scalar();
		// Makes the y position move down each time a new line is being traversed
		// Makes it so the y position doesn't go down on the first line but goes down every row after that
		let mut newline_scalar = 0.0;
		// If there are column labels, loop through each line in the column labels to pass over the space it will use
		if label_line_count > 0
		{
			// Move over the space that the label row will take up
			// Note: Tried applying 1 large line for each row, but there were positioning and sizing issues that
			// happened whenever a row spanned multiple pages.
			// Positioning issues likely has to do with subtracting too much space from remaining space to pass over
			// or apply color to.
			for _ in 0..label_line_count
			{
				// Check to see if a new page needs to be made
				self.check_for_new_page();
				// Move the y position down a newline amount (unless its the first row)
				self.y -= self.current_newline_amount() * newline_scalar;
				// Make it so the y position goes down every line after the first
				newline_scalar = 1.0;
			}
			// Move the y position down by the amount of space between rows
			self.y -= self.table_vertical_cell_margin();
			// Make it so the next row will have a color line
			off_row = true;
		}
		// Loop through each row to pass over space or apply color lines
		for line_count in row_line_counts
		{
			// Make it so the first line in each row doesn't make the y position move down at all
			newline_scalar = 0.0;
			// If this is an off row, apply the color line
			if off_row
			{
				// Loop through each line in the row and apply a color line for that line
				for _ in 0..*line_count
				{
					// Move the y position down a newline amount (unless its the first row)
					self.y -= self.current_newline_amount() * newline_scalar;
					// Check to see if a new page needs to be made
					self.check_for_new_page();
					// Make it so the y position goes down every line after the first
					newline_scalar = 1.0;
					// Apply a color line
					self.apply_table_color_line(self.current_newline_amount(), x_min, x_max, y_adjuster);
				}
			}
			// If this is not an off row, pass over the space this row will use
			else
			{
				// Loop through each line in the row to pass over that space
				for _ in 0..*line_count
				{
					// Move the y position down a newline amount (unless its the first row)
					self.y -= self.current_newline_amount() * newline_scalar;
					// Check to see if a new page needs to be made
					self.check_for_new_page();
					// Make it so the y position goes down every line after the first
					newline_scalar = 1.0;
				}
			}
			// Move the y position down by the amount of space between rows
			self.y -= self.table_vertical_cell_margin();
			// Make it so the next row will get the opposite of what this row got
			off_row = !off_row;
		}
	}

	/// Applies a single table color line to the table.
	fn apply_table_color_line(&mut self, line_height: f32, x_min: f32, x_max: f32, y_adjust: f32)
	{
		// Precalculate the y value of the line
		let adjusted_y = self.y + y_adjust;
		// Creates the points of each end of the line (a bit higher than normal to compensate for all lines being a
		// bit off vertically)
		let point_1 = LinePoint
		{
			p: Point
			{
				x: Mm(x_min).into(),
				y: Mm(adjusted_y).into()
			},
			bezier: false
		};
		let point_2 = LinePoint
		{
			p: Point
			{
				x: Mm(x_max).into(),
				y: Mm(adjusted_y).into()
			},
			bezier: false
		};
		// Create the line
		let line = Line
		{
			points: vec![point_1, point_2],
			is_closed: false
		};
		// Create the operations for adding the line to the page
		let ops = vec!
		[
			// Set the color of the line
			Op::SetOutlineColor
			{
				col: self.table_off_row_color().clone()
			},
			// Set the thickness of the line
			Op::SetOutlineThickness
			{
				pt: Pt(line_height * self.table_off_row_color_lines_height_scalar())
			},
			// Apply the line to the page
			Op::DrawLine
			{
				line: line
			}
		];
		// Add the operations to the page
		self.doc.pages[self.current_page_index].ops.extend_from_slice(&ops);
	}

	/// Applies the text within the cells of a table to the spellbook.
	fn apply_table_cells
	(
		&mut self,
		column_label_lines: &Vec<Vec<TextLine>>,
		cell_lines: &Vec<Vec<Vec<TextLine>>>,
		column_data: &Vec<TableColumnData>
	)
	{
		// Makes it so the first line doesn't move down at all at the start
		let mut row_vertical_adjuster = 0.0;
		// If there are column labels
		if column_label_lines.len() > 0
		{
			// Apply the column labels to the document
			self.apply_table_row(column_label_lines, column_data, FontVariant::Bold);
			// Make it so the next row moves down at the start
			row_vertical_adjuster = self.table_vertical_cell_margin();
		}
		// Loop through each row to apply it
		for row in cell_lines
		{
			// Move down a cell margin (unless this is the first row)
			self.y -= row_vertical_adjuster;
			// Make it so all future rows will move down at the start
			row_vertical_adjuster = self.table_vertical_cell_margin();
			// Apply to the document
			self.apply_table_row(&row, column_data, FontVariant::Regular);
		}
	}

	/// Applies a row of cells from a table to the spellbook.
	fn apply_table_row
	(
		&mut self,
		row: &Vec<Vec<TextLine>>,
		column_data: &Vec<TableColumnData>,
		starting_font_variant: FontVariant
	)
	{
		// Saves the current page index and y position so each cell can reset to it so it can start its text at the
		// top of the row
		let row_start_page_index = self.current_page_index;
		let row_start_y = self.y;
		// Keeps track of the page and y position of where the row ends so it can be set to there after all the cells
		// have been applied
		let mut row_end_page_index = self.current_page_index;
		let mut row_end_y = self.y;
		// Loop through each cell to apply them
		for i in 0..row.len()
		{
			// Reset the font variant for this row
			self.set_current_font_variant(starting_font_variant);
			// Apply the text in this cell to the document
			self.apply_table_cell(&row[i], &column_data[i]);
			// If this cell ended on a new page no cell in this row has been to before
			if self.current_page_index > row_end_page_index
			{
				// Set this position to where the end of the row is
				row_end_page_index = self.current_page_index;
				row_end_y = self.y;
			}
			// If this cell ended on the same page as the previous longest cell
			else if self.current_page_index == row_end_page_index
			{
				// Set the end of row y position to the greater of the two y positions between the previous end
				// position and the current y position
				row_end_y = row_end_y.min(self.y);
			}
			// Reset the page and y position back to the start of the row for the next cell
			self.current_page_index = row_start_page_index;
			self.y = row_start_y;
		}
		// Set the page and y position to the end of the row for the next row
		self.current_page_index = row_end_page_index;
		self.y = row_end_y;
	}

	/// Applies a single cell from a table to the spellbook.
	fn apply_table_cell(&mut self, cell: &Vec<TextLine>, column_data: &TableColumnData)
	{
		// If the column this cell is in is a centered text column
		if column_data.centered
		{
			// Write this cell's text to the document in a centered textbox
			self.apply_centered_text_lines(cell, column_data.x_min, column_data.x_max);
		}
		else
		{
			// Set the x position to the left side of the cell
			self.x = column_data.x_min;
			// Write this cell's text to the document in a left-aligned textbox
			self.apply_text_lines(cell, column_data.x_min);
		}
	}

	
	/// Applies lines to a text box so that the text is left aligned.
	/// `x_reset` is the value that the x position gets reset to after it applies each line.
	/// `y_min` is the minimum y value on the page.
	fn apply_text_lines(&mut self, text_lines: &Vec<TextLine>, x_reset: f32)
	{
		// The number of newlines to go down by before each line is printed
		// Is 0.0 for the first line (so the textbox doesn't get moved down by an extra newline)
		// Is 1.0 for all other lines
		let mut newline_scalar = 0.0;
		// Loop through each line to apply it to the document
		for line in text_lines
		{
			if line.is_empty() { continue; }
			// Move the y position down by 0 or 1 newline amounts
			// 0 newlines for the first line (so the textbox doesn't get moved down by an extra newline)
			// 1 newline for all other lines
			self.y -= newline_scalar * self.current_newline_amount();
			// Make it so all lines after the first will move down 1 newline amount before being applied to the page
			newline_scalar = 1.0;
			// Apply the line to the page
			self.apply_text_line(line);
			self.x = x_reset;
		}
	}

	/// Takes a string along with a maximum width for lines to fit into, separates the string into lines of tokens
	/// that fit within the max width, and returns a vec of those lines.
	fn get_textbox_lines(&mut self, text: &str, first_line_width: f32, textbox_width: f32) -> Vec<TextLine>
	{
		// Get all tokens separated by whitespace
		// Collects it into a vec so the `is_empty` method can be used without having to clone a new iterator.
		let tokens: Vec<_> = text.split_whitespace().collect();
		// If there is no text, do nothing
		if tokens.is_empty() { return Vec::new(); }
		// Store the font variant at the start so the current font variant can be reset to it after constructing the
		// lines of text since the current font variant will change while calculating line widths
		let start_font_variant = *self.current_font_variant();
		// Keeps track of the current max textbox width
		// Uses `first_line_width` for the first line and `textbox_width` for all lines after that
		let mut current_line_max_width = first_line_width;
		// Vec containing each line of text to write to the textbox
		let mut lines: Vec<TextLine> = Vec::with_capacity(1);
		// Keeps track of the next line of tokens to fill up and add to the vec of lines
		let mut line = TextLine::with_capacity
		(
			tokens.len(),
			*self.current_text_type(),
			*self.current_font_variant()
		);
		// Loop through each token to measure how many lines there will be and how long each line is
		for i in 0..tokens.len()
		{
			// Find every font tag in the token
			let font_tag_matches: Vec<Match> = self.font_tag_regex.find_iter(tokens[i]).collect();
			// If there are no font tags in the token, just add it to the line / next line
			if font_tag_matches.is_empty()
			{
				self.add_text_token_to_lines
				(
					tokens[i],
					&mut line,
					&mut lines,
					&mut current_line_max_width,
					textbox_width,
					tokens.len() - i,
					true
				);
			}
			// If there are any font tags within the token, parse them along with any normal text in the token
			else
			{
				// Keeps track of whether any actual text from this token has been added yet or not
				// This is for making it so only one space gets added before the token at the start upon the first
				// text being added, and for making it so following text tokens are forced into being hyphenated if
				// they are too big to fit on the line instead of being moved to the next line if that would make
				// them fit so that the whole token isn't separated by any unnecessary whitespace
				let mut text_added = false;
				// Keeps track of the ending index of the previous font tag so the next part of the token knows where
				// the next part of the token starts
				let mut previous_end = 0;
				// Loop through each font tag match
				for j in 0..font_tag_matches.len()
				{
					// If the font tag is escaped (has a backslash before it)
					// Add all text before the tag (except the last backslash), the tag, and the following text to
					// the line as a normal text token
					if font_tag_matches[j].start() > 0 &&
					tokens[i].chars().nth(font_tag_matches[j].start() - 1) == Some('\\')
					{
						// Get the index of the next font tag
						let next_index = j + 1;
						// Determine where the end index of this text token is
						// If there is a font tag after this one
						let end = if next_index < font_tag_matches.len()
						{
							// If the next font tag also has a backslash before it
							if tokens[i].chars().nth(font_tag_matches[next_index].start() - 1) == Some('\\')
							{
								// End this text token right before the next backslash so the next font tag can deal
								// with it
								font_tag_matches[next_index].start() - 1
							}
							// If there is not backslash before the next font tag, end this token right before the
							// font tag
							else { font_tag_matches[next_index].start() }
						}
						// If this is the last font tag, end this text token at the end of the whole token
						else { tokens[i].len() };
						// Create the text token that will get added to the line from the text before the font tag to
						// the text after it (excluding the escaping backslash)
						let text_token = String::from(&tokens[i][previous_end..font_tag_matches[j].start() - 1]) +
						&tokens[i][font_tag_matches[j].start()..end];
						// Add the text token to the line / next line(s)
						self.add_text_token_to_lines
						(
							&text_token,
							&mut line,
							&mut lines,
							&mut current_line_max_width,
							textbox_width,
							tokens.len() - i,
							!text_added
						);
						// Confirm that some actual text from this token has been added
						text_added = true;
						// Move up the previous end index to the end of this part of the token
						previous_end = end;
						// Skip over adding the font tag as a font tag and changing the previous end again
						continue;
					}
					// If there is any normal text that needs to be processed before this font tag
					else if font_tag_matches[j].start() > previous_end
					{
						// Process and add it
						self.add_text_token_to_lines
						(
							&tokens[i][previous_end..font_tag_matches[j].start()], 
							&mut line, 
							&mut lines, 
							&mut current_line_max_width, 
							textbox_width, 
							tokens.len() - i,
							!text_added
						);
						// Mark that some normal text has been added to the line
						text_added = true;
					}
					// Add the font tag to the line
					let _ = self.add_font_change_to_line
					(
						&tokens[i][font_tag_matches[j].start()..font_tag_matches[j].end()],
						&mut line
					);
					// Move up the previous end to the end of this font tag
					previous_end = font_tag_matches[j].end();
				}
				// If there is any text left after the last font tag, add it to the line / next line(s)
				if previous_end < tokens[i].len()
				{
					self.add_text_token_to_lines
					(
						&tokens[i][previous_end..],
						&mut line,
						&mut lines,
						&mut current_line_max_width,
						textbox_width,
						tokens.len() - i,
						!text_added
					);
				}
			}
		}
		// Make sure the line doesn't have any excess capacity in its vec
		line.shrink_to_fit();
		// Push the remaining text in the last line to the vec of lines
		lines.push(line);
		// Set the font variant back to what it's supposed to be at the start of the text
		self.set_current_font_variant(start_font_variant);
		// Return the lines of text
		lines
	}

	/// Constructs and returns a text token without a precalculated width.
	// fn get_text_token(&self, token: &str, font_variant: FontVariant) -> TextToken
	// {
	// 	let font_size_data = self.font_data.get_size_data_for(font_variant);
	// 	let scalar = self.font_data.get_scalar_for(font_variant);
	// 	TextToken::new(token, font_size_data, self.current_font_scale(), scalar)
	// }

	/// Adds a font change token to a line
	fn add_font_change_to_line(&mut self, token: &str, line: &mut TextLine) -> Result<(), String>
	{
		// Determine what kind of font tag it is
		match token
		{
			REGULAR_FONT_TAG =>
			{
				// Add the font tag to the line
				line.add_font_tag(FontVariant::Regular);
				// Change the font variant to the variant of this font tag
				self.set_current_font_variant(FontVariant::Regular);
			},
			BOLD_FONT_TAG =>
			{
				line.add_font_tag(FontVariant::Bold);
				self.set_current_font_variant(FontVariant::Bold);
			},
			ITALIC_FONT_TAG =>
			{
				line.add_font_tag(FontVariant::Italic);
				self.set_current_font_variant(FontVariant::Italic);
			},
			BOLD_ITALIC_FONT_TAG | ITALIC_BOLD_FONT_TAG =>
			{
				line.add_font_tag(FontVariant::BoldItalic);
				self.set_current_font_variant(FontVariant::BoldItalic);
			},
			// If it wasn't a valid font tag, return an error
			_ => return Err(String::from("Token must be a font tag."))
		}
		Ok(())
	}

	/// Adds a token of text to a line that is under construction and possibly adds that
	/// line and any other new ones created to a vec of lines being constructed.
	fn add_text_token_to_lines
	(
		&mut self,
		mut token: &str,
		line: &mut TextLine,
		lines: &mut Vec<TextLine>,
		current_line_max_width: &mut f32,
		textbox_width: f32,
		remaining_tokens: usize,
		token_start: bool
	)
	{
		// Declare a width variable that will be calculated when the tokens is hyphenated
		let width;
		// Hyphenate the token if it's too long to fit on a line and compute its width
		(token, width) = self.hyphenate_token
		(
			token,
			current_line_max_width,
			textbox_width,
			line,
			lines,
			!token_start
		);
		// If the line is currently empty
		if line.width() == 0.0
		{
			// Put the token into the line
			let text_token = TextToken::with_width(token, width);
			line.add_text(text_token);
		}
		// If the line is not empty
		else if line.width() > 0.0
		{
			// Calculate the width of the current token with a space in front of it (if a space is desired)
			let padded_width = if token_start { line.get_last_space_width(self.space_widths()) + width }
			else { width };
			// If adding this token to the line would make it go outside the textbox,
			// apply the current line and set it to just the current token
			if line.width() + padded_width > *current_line_max_width
			{
				// Make sure the line doesn't have any excess capacity in its vec
				line.shrink_to_fit();
				// Add the current line to the vec of lines
				lines.push(line.clone());
				// Create a new line with the capacity of the number of remaining tokens
				*line = TextLine::with_capacity
				(
					remaining_tokens,
					*self.current_text_type(),
					*self.current_font_variant()
				);
				// Add the token to the start of the new line
				let text_token = TextToken::with_width(token, width);
				line.add_text(text_token);
				// Set the max width width to the textbox width in case the previous line was the first
				// line
				*current_line_max_width = textbox_width;
			}
			// If this token can fit on the line, add it to the line
			else
			{
				// Add this token to the line
				let text_token = TextToken::with_width(token, width);
				// Add a space before the token if desired
				if token_start { line.add_space(self.space_widths()); }
				line.add_text(text_token);
			}
		}
		// If the line has a negative width
		else
		{
			panic!("Line width is less than 0.0 in `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::add_text_token_to_lines`");
		}
	}

	/// If the given token is too wide to fit on a single line within the given textbox constraints, hyphenate it and
	/// apply it to the spellbook until the end of it is reached and it can fit in a single line without being
	/// hyphenated.
	/// Takes the current max width (which might be shorter on the first line of a textbox) and sets it to the
	/// textbox width if the token is hyphenated and a line is applied.
	/// Takes the current line and the vec of lines being processed to modify them if the token is hyphenated.
	/// Returns the token and its calculated width if it was short enough to fit on a line, otherwise it returns the
	/// end of the hyphenated token and its width.
	fn hyphenate_token<'t>
	(
		&mut self,
		mut token: &'t str,
		current_line_max_width: &mut f32,
		textbox_width: f32,
		current_line: &mut TextLine,
		lines: &mut Vec<TextLine>,
		force_hyphenate: bool
	)
	-> (&'t str, f32)
	{
		// Calculate the width of the token
		let mut width = self.calc_text_width(token);
		// If the line is empty and the token is wider than the current line
		if current_line.width() == 0.0 && width > *current_line_max_width
		{
			// Hyphenate the token using the current line's width
			(token, width) = self.hyphenate_once
			(
				token,
				width,
				*current_line_max_width,
				current_line,
				lines,
				false
			);
		}
		// If the token is wider than the textbox width or a hyphenation is being forced
		else if width > textbox_width || force_hyphenate
		{
			// Hyphenate the token using the remaining width on the current line
			let remaining_width =
			*current_line_max_width - current_line.width() - current_line.get_last_space_width(self.space_widths());
			(token, width) = self.hyphenate_once
			(
				token,
				width,
				remaining_width,
				current_line,
				lines,
				!force_hyphenate
			);
		}
		// If the token fits on the current line and doesn't need to be hyphenated, just return it and its width
		else { return (token, width); }
		// Reset the current line width to the width of the textbox since a line just had to have been applied to get
		// to this point and it is no longer the first line (which is the only line that could have a different
		// width than the rest)
		*current_line_max_width = textbox_width;
		// Hyphenate the token until just the end of it remains and it can fit on a single line
		while width > textbox_width
		{
			(token, width) = self.hyphenate_once(token, width, textbox_width, current_line, lines, false);
		}
		// Return the end of the token and its width
		(token, width)
	}

	/// Hyphenates it a single time, applies and resets the current line, and returns the rest of the hyphenated
	/// token along with its width if the token is too big to fit on a line. Otherwise it just returns the token the
	/// way it is along with its width.
	fn hyphenate_once<'t>
	(
		&mut self,
		mut token: &'t str,
		mut width: f32,
		textbox_width: f32,
		current_line: &mut TextLine,
		lines: &mut Vec<TextLine>,
		add_space: bool
	)
	-> (&'t str, f32)
	{
		// If its small enough to fit on the line, return it the way it is along with its width
		if width <= textbox_width { return (token, width); }
		// Hyphenates the string and gets the hyphenated part as a `TextToken` and an index for where the rest of it
		// starts in the string
		let (hyphenated_token, index) = self.get_hyphen_str(token, width, textbox_width);
		// If the token could be hyphenated to fit on the line (if the returned index is 0, that means the token was
		// either too close to the end of the line to be hyphenated or has characters that are too wide to fit in the
		// textbox)
		if index > 0
		{
			// If the token was hyphenated
			if index < token.len()
			{
				// Add a space before the token if spaces are allowed and the line already has text in it
				if add_space && current_line.width() > 0.0 { current_line.add_space(&self.space_widths()) }
				// Add the hyphenated part of the token to the current line
				current_line.add_text(hyphenated_token);
				// Make sure there isn't any extra capacity in the line's token vec
				current_line.shrink_to_fit();
			}
			// If the returned index was the length of the token (which means it could fit on the line without being
			// hyphenated)
			else
			{
				// Return the token the way it is along with its width
				return (token, width);
			}
		}
		// Apply the line
		lines.push(current_line.clone());
		// Empty the line
		*current_line = TextLine::with_capacity(1, *self.current_text_type(), *self.current_font_variant());
		// Chop off the hyphenated part from the token
		token = &token[index..];
		// Recalculate the width of the token
		width = self.calc_text_width(token);
		// Return the token and its width
		(token, width)
	}

	/// Takes a string that is too wide to fit on a single line in a textbox and finds the cutoff / delimiter index
	/// so that `&text[0..index] + '-'` fits inside the textbox, along with that hyphenated string itself
	fn get_hyphen_str(&self, text: &str, token_width: f32, textbox_width: f32) -> (TextToken, usize)
	{
		// Keeps track of the last hyphenated part of the text that was measured
		#[allow(unused_assignments)]
		let mut hyphenated_string = String::new();
		// Keeps track of the width of the hyphenated part of the text
		let mut hyphen_str_width = token_width;
		// If the string can fit in the textbox, return an empty text token and the inputted token's length
		if hyphen_str_width <= textbox_width { return (TextToken::empty(), text.len()); }
		// Lower and upper possible bounds for what the index could be
		let mut lower_bound = 0;
		let mut upper_bound = text.len();
		// The current index being tested
		let mut index = upper_bound / 2;
		// Do - While loop until index and last_index are equal
		// Binary search for the index where the text plus a hyphen at the end is as long as possible without going
		// outside the textbox
		while
		{
			// Store index in last index so the loop can know when to end
			let last_index = index;
			// Get a string of the start of the text up to the index with a hyphen at the end
			hyphenated_string = format!("{}-", &text[0..index]);
			// Calculate the width of the hyphenated string
			hyphen_str_width = self.calc_text_width(&hyphenated_string);
			// If the width is exactly the width of the textbox, return the current hyphen string data
			if hyphen_str_width == textbox_width
			{
				let new_token = TextToken::with_width(&hyphenated_string, hyphen_str_width);
				return (new_token, index);
			}
			// If the width is less than the width of the textbox
			else if hyphen_str_width < textbox_width
			{
				// Increase the lower bound to the index
				lower_bound = index;
				// Increase the index to be between the lower bound and upper bound
				index += (upper_bound - lower_bound) / 2;
			}
			// If the width is greater than the width of the textbox
			else
			{
				// Decrease the upper bound to the index
				upper_bound = index;
				// Decrease the index to be between the lower and upper bound
				index -= (upper_bound - lower_bound) / 2;
			}
			// Do - While condition
			index != last_index
		}{}
		// If the index is 0, set the return token to be empty
		let new_token = if index == 0 { TextToken::empty() }
		// Otherwise set the return token to be the part of the string that was hyphenated in the last loop iteration
		else { TextToken::with_width(&hyphenated_string, hyphen_str_width) };
		// Return the token and index
		(new_token, index)
	}

	/// Applies lines of text to the spellbook so that each line is centered horizontally.
	fn apply_centered_text_lines
	(
		&mut self,
		text_lines: &Vec<TextLine>,
		x_min: f32,
		x_max: f32
	)
	{
		let textbox_width = x_max - x_min;
		// The number of newlines to go down by before each line is printed
		// Is 0.0 for the first line (so the textbox doesn't get moved down by an extra newline)
		// Is 1.0 for all other lines
		let mut newline_scalar = 0.0;
		// Loop through each line to apply it to the document
		for line in text_lines
		{
			if line.is_empty() { continue; }
			// Move the y position down by 0 or 1 newline amounts
			// 0 newlines for the first line (so the textbox doesn't get moved down by an extra newline)
			// 1 newline for all other lines
			self.y -= newline_scalar * self.current_newline_amount();
			// Make it so all lines after the first will move down 1 newline amount before being applied to the page
			newline_scalar = 1.0;
			// Calculate where to set the x position so that the line is horizontally centered in the textbox and set
			// the x value to that
			self.x = (textbox_width / 2.0) - (line.width() / 2.0) + x_min;
			// Apply the line to the page
			self.apply_text_line(line);
		}
	}

	/// Applies a single line of text to the current page in the spellbook.
	/// Moves text down to a new page if the y position below the min y value.
	fn apply_text_line(&mut self, line: &TextLine)
	{
		// If the line is empty, do nothing
		if line.is_empty() { return; }
		// Checks to see if the text should can fit on this page or needs to move to a new page.
		self.check_for_new_page();
		// Get the tokens to loop through
		let tokens = line.tokens();
		// Holds the next text to apply to the page
		// (before a font change or at the end of looping through the tokens to find font changes)
		let mut next_text = String::with_capacity(line.byte_count());
		// Loop through all of the tokens to find font tags
		for index in 0..tokens.len()
		{
			match &tokens[index]
			{
				// If the current token is a font tag
				Token::FontTag(font_variant) =>
				{
					// If the font tag is different than the current font, apply previous text and switch font
					if *font_variant != *self.current_font_variant()
					{
						self.apply_text(&next_text);
						// Switch the font to the current font tag
						self.set_current_font_variant(*font_variant);
						// next_text = next_text.split_off(index+1);
						// Empty the next text string but reserve
						next_text = String::with_capacity(next_text.capacity() - next_text.len());
					}
				},
				// If the token is anything else, add it to the next string of text to be applied
				_ =>
				{
					next_text += &tokens[index].as_spellbook_string();
				}
			}
		}
		// Apply all remaining text on the line to the page
		self.apply_text(&next_text);
	}

	/// Checks if the current layer should move to the next page if the text y position is below given `y_min` value.
	/// Sets the y position to the top of the page if the function moves the text to a new page.
	/// Creates a new page if the page index goes beyond the number of layers that exist.
	fn check_for_new_page(&mut self)
	{
		// If the y level is below the bottom of where text is allowed on the page, go to a new page
		if self.y < self.y_min() { self.move_to_new_page(); }
	}

	// Move to a new page. Sets the y position to the top of the page and creates a new page if needed.
	fn move_to_new_page(&mut self)
	{
		// Increase the current page index to the layer for the next page
		self.current_page_index += 1;
		// If the index is beyond the number of layers in the document
		if self.current_page_index >= self.doc.pages.len()
		{
			// Create a new page
			self.make_new_page();
		}
		// Move the y position of the text to the top of the page
		self.y = self.y_top();
	}

	/// Adds a new page to the pdf document, including the background image and page number if options for those were
	/// given. Sets `current_page_index` to the new page.
	fn make_new_page(&mut self)
	{
		// Create a new page
		let page = PdfPage::new(Mm(self.page_width()), Mm(self.page_height()), Vec::new());
		// Add the page to the document
		self.doc.pages.push(page);
		// Update the current page index to point to the new page
		self.current_page_index = self.doc.pages.len() - 1;
		// Add a background image (if there is a background to add)
		self.add_background();
		// Adds a page number to the new page (if there are page numbers)
		self.add_page_number();
		// Increases the page number count by 1
		self.current_page_num += 1;
	}

	/// Adds the background image to the current layer (if a background image was given to use).
	fn add_background(&mut self)
	{
		// If there is a background image
		if let Some(background) = &self.background
		{
			// Construct an operation to add the background image to the page
			let add_image_op = Op::UseXobject
			{
				id: background.image_id().clone(),
				transform: background.transform().clone()
			};
			// Add the operation to the page
			self.doc.pages[self.current_page_index].ops.push(add_image_op);
		}
	}

	/// Adds the page number to the current layer (if page number options were given).
	fn add_page_number(&mut self)
	{
		// Determine whether there are page numbers in this spellbook
		match &self.page_number_data
		{
			// If there are page numbers
			Some(data) =>
			{
				// Convert the current page number into a string
				let text = self.current_page_num.to_string();
				// Determine the x position of the page number based on if it will be on the left or right side of the
				// page
				let x = match data.current_side()
				{
					HSide::Left => data.side_margin(),
					HSide::Right =>
					{
						// Calculate the width of the page number text
						let text_width = self.calc_page_number_width(&text);
						// Set the x value to be based on the width of the text and the page margin
						self.page_width() - data.side_margin() - text_width
					}
				};
				// Apply the page number to the page using custom text parameters according to the page number data.
				self.apply_text_with
				(
					&text,
					x,
					data.bottom_margin(),
					data.font_size(),
					data.font_id().clone(),
					data.color().clone()
				);
			},
			// Do nothing if there are no page numbers
			None => ()
		};

		// Have the page number for the next page flip sides if there are page numbers.
		// Have to do this in a separate match statement because of mutable and immutable reference borrowing.
		match &mut self.page_number_data
		{
			Some(data) => if data.flips_sides() { data.flip_side(); }
			None => ()
		};
	}

	/// Writes a line of text to a page.
	fn apply_text(&mut self, text: &str)
	{
		self.apply_text_with
		(
			text,
			self.x,
			self.y,
			self.current_font_size(),
			self.current_font_id().clone(),
			self.current_text_color().clone()
		);
	}

	/// Writes a line of text to a page using a specific parameters rather than the current SpellbookWriter
	/// Settings.
	fn apply_text_with(&mut self, text: &str, x: f32, y: f32, font_size: f32, font: FontId, color: Color)
	{
		// If there is no text to apply, do nothing
		if text.is_empty() { return; }
		// Create a vec of the operations to add the text to the page
		let ops = vec!
		[
			// Start a text section (required for text ops)
			Op::StartTextSection,
			// Place the text cursor in the correct x and y values
			Op::SetTextCursor
			{
				pos: Point
				{
					// Convert from Mm to Pt
					x: Mm(x).into(),
					y: Mm(y).into()
				}
			},
			// Set the font size
			Op::SetFontSize
			{
				size: Pt(font_size),
				font: font.clone()
			},
			// Set the color of the text
			Op::SetFillColor
			{
				col: color
			},
			// Apply the text to the page
			Op::WriteText
			{
				items: vec![TextItem::Text(String::from(text))],
				font: font
			},
			// End the text section
			Op::EndTextSection
		];
		// Add the operations to the page
		self.doc.pages[self.current_page_index].ops.extend_from_slice(&ops);
		// Move the x position to be at the end of the newly applied line
		self.x += self.calc_text_width(&text);
	}

	/// Calculates the width of some text using the current state of this object's font data field.
	fn calc_text_width(&self, text: &str) -> f32
	{
		calc_text_width(text, self.current_size_data(), self.current_font_scale(), self.current_scalar())
	}

	/// Calculates the height of a certain number of lines of text using the current state of this object's font data
	/// field.
	fn calc_text_height(&self, lines: usize) -> f32
	{
		calc_text_height
		(
			self.current_newline_amount(),
			lines
		)
	}

	// /// Returns half the height of a single line with the current text / font state.
	// fn half_line_height(&self) -> f32 { self.line_height() / 2.0 }

	// /// Returns the height of a single line with the current text / font state.
	// fn line_height(&self) -> f32
	// {
	// 	line_height(self.current_size_data(), self.current_font_scale(), self.current_font_size())
	// }

	/// Calculates the text width of a page number.
	fn calc_page_number_width(&self, page_number_text: &str) -> f32
	{
		// Attempt to retrive all necessary data, panic if there is no page number data
		let font_size_data = self.page_number_font_size_data()
		.expect("Called `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::calc_page_number_width` with no page number data");
		let font_scale = self.page_number_font_scale()
		.expect("Called `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::calc_page_number_width` with no page number data.");
		let font_scalar = self.page_number_font_scalar()
		.expect("Called `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::calc_page_number_width` with no page number data.");
		// Return the width of the page number
		calc_text_width(page_number_text, font_size_data, font_scale, font_scalar)
	}

	// General Field Getters

	// fn document(&self) -> &PdfDocument { &self.doc }
	// fn layers(&self) -> &Vec<PdfLayerReference> { &self.layers }
	// fn pages(&self) -> &Vec<PdfPageIndex> { &self.pages }
	fn current_page_index(&self) -> usize { self.current_page_index }
	// fn current_page_num(&self) -> i64 { self.current_page_num }
	// fn font_data(&self) -> &FontData { &self.font_data }
	// fn page_size_data(&self) -> &PageSizeData { &self.page_size_data }
	// fn page_number_data(&self) -> &Option<PageNumberData> { &self.page_number_data }
	// fn background(&self) -> &Option<BackgroundImage> { &self.background }
	// fn table_data(&self) -> &TableData { &self.table_data }
	fn space_widths(&self) -> &SpaceWidths { &self.space_widths }
	// /// Current x position of the text
	// fn x(&self) -> &f32 { &self.x }
	// /// Current y position of the text
	// fn y(&self) -> &f32 { &self.y }

	// Font Getters

	/// The current font variant being used to write text (regular, bold, italic, bold-italic).
	fn current_font_variant(&self) -> &FontVariant { self.font_data.current_font_variant() }
	/// The current type of text being written.
	fn current_text_type(&self) -> &TextType { self.font_data.current_text_type() }
	// /// `FontIds` for each font variant (regular, bold, italic, bold-italic).
	// fn all_font_ids(&self) -> &FontIds { self.font_data.all_font_ids() }
	// /// Font sizes for each type of text.
	// fn all_font_sizes(&self) -> &FontSizes { self.font_data.all_font_sizes() }
	// /// Scalar values for each font variant (regular, bold, italic, bold-italic).
	// fn all_scalars(&self) -> &FontScalars { self.font_data.all_scalars() }
	// /// Size data for each font variant (regular, bold, italic, bold-italic).
	// fn all_size_data(&self) -> &FontSizeData { self.font_data.all_size_data() }
	// /// Font scale sizing data for each type of text.
	// fn all_scales(&self) -> &FontScales { self.font_data.all_scales() }
	// /// All spacing options that were originally passed to this object.
	// fn all_spacing_options(&self) -> &SpacingOptions { self.font_data.all_spacing_options() }
	// /// RGB color values for each type of text.
	// fn all_text_colors(&self) -> &TextColors { self.font_data.all_text_colors() }
	/// Tab size in pringpdf Mm.
	fn tab_amount(&self) -> f32 { self.font_data.tab_amount() }
	/// The font object for the current font variant being used.
	fn current_font_id(&self) -> &FontId { self.font_data.current_font_id() }
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
	fn current_text_color(&self) -> &Color { self.font_data.current_text_color() }

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
	/// The highest point text with the current font state can be on a page.
	fn y_top(&self) -> f32 { self.y_max() - self.current_newline_amount() / 2.0 }
	/// The lowest point text with the current font state can be on a page.
	fn y_bottom(&self) -> f32 { self.y_min() + self.current_newline_amount() / 2.0  }
	// // Dimensions that text can fit inside
	// pub fn text_width(&self) -> f32 { self.page_size_data.text_width() }
	// pub fn text_height(&self) -> f32 { self.page_size_data.text_height() }

	// Page Number Getters

	// /// The side of the page (left or right) the page number starts on.
	// fn starting_page_number_side(&self) -> Option<HSide>
	// {
	// 	match &self.page_number_data
	// 	{
	// 		Some(data) => Some(data.starting_side()),
	// 		None => None
	// 	}
	// }

	// /// Whether or not the page number flips sides every page.
	// fn page_number_flips_sides(&self) -> Option<bool>
	// {
	// 	match &self.page_number_data
	// 	{
	// 		Some(data) => Some(data.flips_sides()),
	// 		None => None
	// 	}
	// }

	// /// The starting page number.
	// fn starting_page_number(&self) -> Option<i64>
	// {
	// 	match &self.page_number_data
	// 	{
	// 		Some(data) => Some(data.starting_num()),
	// 		None => None
	// 	}
	// }

	// /// The font variant the page numbers use.
	// fn page_number_font_variant(&self) -> Option<FontVariant>
	// {
	// 	match &self.page_number_data
	// 	{
	// 		Some(data) => Some(data.font_variant()),
	// 		None => None
	// 	}
	// }

	// /// The font size of the page numbers.
	// fn page_number_font_size(&self) -> Option<f32>
	// {
	// 	match &self.page_number_data
	// 	{
	// 		Some(data) => Some(data.font_size()),
	// 		None => None
	// 	}
	// }

	// /// The amount of space between newlines for page numbers in case of overflow.
	// fn page_number_newline_amount(&self) -> Option<f32>
	// {
	// 	match &self.page_number_data
	// 	{
	// 		Some(data) => Some(data.newline_amount()),
	// 		None => None
	// 	}
	// }

	// /// The amount of space between the side of the page and the page number in printpdf Mm.
	// fn page_number_side_margin(&self) -> Option<f32>
	// {
	// 	match &self.page_number_data
	// 	{
	// 		Some(data) => Some(data.side_margin()),
	// 		None => None
	// 	}
	// }
	
	// /// The amount of space between the bottom of the page and the page number in printpdf Mm.
	// fn page_number_bottom_margin(&self) -> Option<f32>
	// {
	// 	match &self.page_number_data
	// 	{
	// 		Some(data) => Some(data.bottom_margin()),
	// 		None => None
	// 	}
	// }

	// /// All of the original page number options that were inputted.
	// fn page_number_options(&self) -> Option<&PageNumberOptions>
	// {
	// 	match &self.page_number_data
	// 	{
	// 		Some(data) => Some(data.options()),
	// 		None => None
	// 	}
	// }

	// /// The current side of the page (left or right) the page number is on.
	// fn current_page_number_side(&self) -> Option<HSide>
	// {
	// 	match &self.page_number_data
	// 	{
	// 		Some(data) => Some(data.current_side()),
	// 		None => None
	// 	}
	// }

	// /// Returns the font ref to the current font type bring used for page numbers.
	// fn page_number_font_ref(&self) -> Option<&IndirectFontRef>
	// {
	// 	match &self.page_number_data
	// 	{
	// 		Some(data) => Some(data.font_ref()),
	// 		None => None
	// 	}
	// }

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

	// /// The text color of the page number.
	// fn page_number_color(&self) -> Option<&Color>
	// {
	// 	match &self.page_number_data
	// 	{
	// 		Some(data) => Some(data.color()),
	// 		None => None
	// 	}
	// }

	// Table Getters

	/// Space between columns in printpdf Mm.
	fn table_horizontal_cell_margin(&self) -> f32 { self.table_data.horizontal_cell_margin() }
	/// Space between rows in printpdf Mm.
	fn table_vertical_cell_margin(&self) -> f32 { self.table_data.vertical_cell_margin() }
	/// Minimum space between sides of table and sides of pages in printpdf Mm.
	fn table_outer_horizontal_margin(&self) -> f32 { self.table_data.outer_horizontal_margin() }
	/// Space above and below table from other text / tables in printpdf Mm.
	fn table_outer_vertical_margin(&self) -> f32 { self.table_data.outer_vertical_margin() }
	/// Scalar value to adjust off-row color lines to line up with the rows vertically.
	fn table_off_row_color_lines_y_adjust_scalar(&self) -> f32
	{ self.table_data.off_row_color_lines_y_adjust_scalar() }
	/// Scalar value to determine the height of off-row color lines.
	fn table_off_row_color_lines_height_scalar(&self) -> f32
	{ self.table_data.off_row_color_lines_height_scalar() }
	// RGB value of the color of the off-row color lines.
	fn table_off_row_color(&self) -> &Color { self.table_data.off_row_color() }

	// Space Width Getters

	// fn get_current_space_width(&self) -> f32
	// { self.space_widths.get_width_for(*self.current_text_type(), *self.current_font_variant()) }

	// Font Setters

	/// Sets the current font variant that is being used to write text to the spellbook.
	fn set_current_font_variant(&mut self, font_type: FontVariant)
	{ self.font_data.set_current_font_variant(font_type); }
	/// Sets the current type of text that is being written to the spellbook.
	fn set_current_text_type(&mut self, text_type: TextType) { self.font_data.set_current_text_type(text_type); }

	// Page Number Setters

	// /// Flips the side of the page that page numbers appear on.
	// fn flip_page_number_side(&mut self)
	// {
	// 	match &mut self.page_number_data
	// 	{
	// 		Some(ref mut data) => data.flip_side(),
	// 		None => ()
	// 	}
	// }
}
