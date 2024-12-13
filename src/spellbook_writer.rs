//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//	Spellbook PDF document generation
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;
use std::cell::Ref;
use std::error::Error;
use std::ops::Range;

extern crate image;
use rusttype::point;
use printpdf::
{
	PdfDocumentReference,
	PdfDocument,
	PdfLayerReference,
	IndirectFontRef,
	Color,
	Rgb,
	Point,
	Line,
	PdfPageIndex,
	Image
};
use regex::Regex;

use crate::spellbook_gen_types::*;
use crate::spells;

const LAYER_NAME_PREFIX: &str = "Page";
const DEFAULT_SPELLBOOK_TITLE: &str = "Spellbook";
const TITLE_LAYER_NAME: &str = "Title Layer";
const TITLE_PAGE_NAME: &str = "Title Page";

const REGULAR_FONT_TAG: &str = "<r>";
const BOLD_FONT_TAG: &str = "<b>";
const ITALIC_FONT_TAG: &str = "<i>";
const BOLD_ITALIC_FONT_TAG: &str = "<bi>";
const ITALIC_BOLD_FONT_TAG: &str = "<ib>";

/// All data needed to write spells to a pdf document.
// Can't derive clone or debug unfortunately.
pub struct SpellbookWriter<'a>
{
	doc: PdfDocumentReference,
	layers: Vec<PdfLayerReference>,
	pages: Vec<PdfPageIndex>,
	current_page_index: usize,
	current_page_num: i64,
	font_data: FontData<'a>,
	page_size_data: PageSizeData,
	page_number_data: Option<PageNumberData<'a>>,
	background: Option<BackgroundImage>,
	table_options: TableOptions,
	// Stored here so the width of various types of spaces doesn't need to be continually recalculated
	space_widths: SpaceWidths,
	// Regex patterns are stored since they consume lots of runtime being reconstructed continutally
	escaped_font_tag_regex: Regex,
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
		background: Option<(&str, ImageTransform)>,
		table_options: TableOptions
	)
	-> Result<(PdfDocumentReference, Vec<PdfLayerReference>, Vec<PdfPageIndex>), Box<dyn Error>>
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
		Ok((writer.doc, writer.layers, writer.pages))
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
		background: Option<(&str, ImageTransform)>,
		table_options: TableOptions
	)
	-> Result<Self, Box<dyn Error>>
	{
		// Gets a new document and title page.
		let (doc, title_page, title_layer) =
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
			Some((file_path, transform)) => Some(BackgroundImage::new(file_path, transform)?),
			// If no background image was given, don't use a background
			None => None
		};
		// Calculate the width of each variation of a space character
		let space_widths = SpaceWidths::new(&font_data);
		// Create a regex pattern for escaped font tags (font tags preceeded by backslashes)
		// Ex: "\<r>", "\\\<bi>", "\\<i>", etc.
		// Use this regex pattern to remove the first backslash from escaped font tags so that font tags are allowed
		// to actually appear in spell text AND not affect the font at all
		let escaped_font_tag_pattern = format!
		(
			"(\\\\)+({}|{}|{}|{}|{})",
			REGULAR_FONT_TAG,
			BOLD_FONT_TAG,
			ITALIC_FONT_TAG,
			BOLD_ITALIC_FONT_TAG,
			ITALIC_BOLD_FONT_TAG
		);
		let escaped_font_tag_regex = Regex::new(&escaped_font_tag_pattern)
		.expect(format!
		(
			"Failed to build regex pattern \"{}\" in `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::new`",
			escaped_font_tag_pattern
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
			layers: vec![title_layer],
			pages: vec![title_page],
			current_page_index: 0,
			current_page_num: starting_page_num,
			font_data: font_data,
			page_size_data: page_size_data,
			page_number_data: page_number_data,
			background: background,
			space_widths: space_widths,
			table_options: table_options,
			escaped_font_tag_regex: escaped_font_tag_regex,
			table_tag_regex: table_tag_regex,
			backslashes_regex: backslashes_regex,
			x: page_size_data.x_min(),
			y: page_size_data.y_max()
		})
	}

	/// Creates a new pdf document with a given title and width / height dimensions and returns the reference to
	/// it and layer for the title page. Returns the pdf document and the layer for the first page.
	fn create_new_doc(title: &str, width: f32, height: f32)
	-> (PdfDocumentReference, PdfPageIndex, PdfLayerReference)
	{
		// Create the pdf document and the first page
		let (doc, title_page, title_layer_index) =
		// If no title was given for the spellbook (the given title string is empty)
		if title.is_empty()
		{
			// Create pdf document with a default title
			PdfDocument::new(DEFAULT_SPELLBOOK_TITLE, Mm(width), Mm(height), TITLE_LAYER_NAME)
		}
		else
		{
			// Create pdf document with the given title
			PdfDocument::new(title, Mm(width), Mm(height), TITLE_LAYER_NAME)
		};

		// Get PdfLayerReference (title_layer_ref) from PdfLayerIndex (title_layer_index)
		let title_layer_ref = doc.get_page(title_page).get_layer(title_layer_index);

		(doc, title_page, title_layer_ref)
	}

	/// Turns the current page into a title page with the given title.
	fn make_title_page(&mut self, title: &str)
	{
		// Use the default spellbook title if none was given
		if title.is_empty() { let title = DEFAULT_SPELLBOOK_TITLE; }
		// Create bookmark for title page
		self.doc.add_bookmark(TITLE_PAGE_NAME, self.pages[self.current_page_index]);
		// Adds a background image to the page (if they are desired)
		self.add_background();
		// Store the page number data and set it to None so page numbers don't appear in any title pages created
		let page_number_data = self.page_number_data.clone();
		self.page_number_data = None;
		// Write the title to the page
		self.write_centered_textbox(title, self.x_min(), self.x_max(), self.y_min(), self.y_max());
		// Reset the page number data to what it was before
		self.page_number_data = page_number_data;
	}

	/// Adds a page / pages about a spell into the spellbook.
	fn add_spell(&mut self, spell: &spells::Spell)
	{
		// Make a new page for the spell
		self.make_new_page();
		// Add a bookmark for the first page of this spell
		self.doc.add_bookmark(spell.name.clone(), self.pages[self.current_page_index]);

		// Writes the spell name to the document
		self.x = self.x_min();
		self.y = self.y_max();
		self.set_current_text_type(TextType::Header);
		self.set_current_font_variant(FontVariant::Regular);
		self.write_textbox
		(&spell.name, self.x_min(), self.x_max(), self.y_min(), self.y_max(), false, &spell.tables);

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
			self.y_min(),
			self.y_max(),
			false,
			&spell.tables
		);

		// Writes the casting time to the document
		self.y -= self.font_data.get_newline_amount_for(TextType::Header);
		self.x = self.x_min();
		self.set_current_font_variant(FontVariant::Bold);
		let casting_time = format!("Casting Time: <r> {}", spell.get_casting_time_text());
		self.write_textbox
		(&casting_time, self.x_min(), self.x_max(), self.y_min(), self.y_max(), false, &spell.tables);

		// Writes the range to the document
		self.y -= self.font_data.current_newline_amount();
		self.x = self.x_min();
		self.set_current_font_variant(FontVariant::Bold);
		let range = format!("Range: <r> {}", spell.range.to_string());
		self.write_textbox
		(&range, self.x_min(), self.x_max(), self.y_min(), self.y_max(), false, &spell.tables);

		// Writes the components to the document
		self.y -= self.font_data.current_newline_amount();
		self.x = self.x_min();
		self.set_current_font_variant(FontVariant::Bold);
		let components = format!("Components: <r> {}", spell.get_component_string());
		self.write_textbox
		(&components, self.x_min(), self.x_max(), self.y_min(), self.y_max(), false, &spell.tables);

		// Writes the duration to the document
		self.y -= self.font_data.current_newline_amount();
		self.x = self.x_min();
		self.set_current_font_variant(FontVariant::Bold);
		let duration = format!("Duration: <r> {}", &spell.duration.to_string());
		self.write_textbox
		(&duration, self.x_min(), self.x_max(), self.y_min(), self.y_max(), false, &spell.tables);
		
		// Writes the description to the document
		self.y -= self.font_data.get_newline_amount_for(TextType::Header);
		self.x = self.x_min();
		self.set_current_font_variant(FontVariant::Regular);
		self.write_textbox
		(&spell.description, self.x_min(), self.x_max(), self.y_min(), self.y_max(), false, &spell.tables);

		// Writes the upcast description to the document if there is one
		if let Some(upcast_description) = &spell.upcast_description
		{
			self.y -= self.font_data.current_newline_amount();
			self.x = self.x_min() + self.tab_amount();
			self.set_current_font_variant(FontVariant::BoldItalic);
			let upcast_description = format!("Using a Higher-Level Spell Slot. <r> {}", &upcast_description);
			self.write_textbox
			(&upcast_description, self.x_min(), self.x_max(), self.y_min(), self.y_max(), true, &spell.tables);
		}
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
		let mut current_x_min = x_min;
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
		for paragraph in paragraphs
		{
			// Move the y position down by 0 or 1 newline amounts
			// 0 newlines for the first paragraph (so the entire textbox doesn't get moved down by an extra newline)
			// 1 newline for all other paragraphs
			self.y -= paragraph_newline_scalar * self.current_newline_amount();
			// If a table was just being processed, move down an extra newline amount to keep the table separated
			// (to match the Player's Handbook Formatting)
			if in_table { self.y -= self.current_newline_amount(); }
			// Split the paragraph into tokens by whitespace
			let mut tokens: Vec<_> = paragraph.split_whitespace().collect();
			// If there is no text in this paragraph, skip to the next one
			if tokens.is_empty() { continue; }
			// Whether or not this is the first line in the paragraph
			let mut first_line = true;
			// The current line of text being processed
			let mut line = String::new();
			// The width of the current line being processed
			let mut line_width: f32 = 0.0;
			// If the paragraph starts with a bullet point symbol
			if tokens[0] == "•" || tokens[0] == "-"
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
					current_x_min = self.calc_text_width("• ") + x_min;
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
				// If the bullet point symbol is a dash, make it a dot
				if tokens[0] == "-" { tokens[0] = "•"; }
				// Reset the x position to the left side of the text box
				self.x = x_min;
			}
			else
			{
				// If there are any tables to parse
				if tables.len() > 0
				{
					// If there is a table tag in this first token (ex: "[table][5]", "[table][0]", etc.)
					if let Some(pat_match) = self.table_tag_regex.find(tokens[0])
					{
						// Get the index range of the table tag pattern patch
						let table_tag_range = pat_match.range();
						// If the table tag is at the end of the first token
						if table_tag_range.end == tokens[0].len()
						{
							// If the table tag is the whole first token, write a table to the document
							if table_tag_range.start == 0
							{
								// Get a string slice of the table index (the 'x' in "[table][x]")
								let index_str = &tokens[0][8 .. tokens[0].len() - 1];
								// Convert the table index into a number
								let table_index = match index_str.parse::<usize>()
								{
									Ok(index) => index,
									// If the index wasn't a valid number, skip over this table token
									Err(_) => continue
								};
								// If the index is out of bounds of the tables vec, skip over this table token
								if table_index >= tables.len() { continue; }
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
									current_x_min = x_min;
									// Zero the bullet flag to signal that a bullet list isn't being currently
									// processed anymore
									in_bullet_list = false;
								}
								// Zero the paragraph flag
								in_paragraph = false;
								// Make it so all paragraphs after the first get moved down a newline amount before
								// being processed
								paragraph_newline_scalar = 1.0;
								// Reset the x position to the left side of the textbox
								self.x = x_min;
								// Store the current text type and font variant being used so they can be reset to
								// what they were before the table
								let current_text_type = *self.current_text_type();
								let current_font_variant = *self.current_font_variant();
								// TODO: Add code to put in a table
								self.parse_table(&tables[table_index], x_min, x_max, y_min, y_max);
								self.apply_text(tokens[0], y_min);
								// Reset the text type and font variant to what they were before the table
								self.set_current_text_type(current_text_type);
								self.set_current_font_variant(current_font_variant);
								// Skip the token loop below and move to the next paragraph
								continue;
							}
							// Check to see if this is an escaped table tag (a backslash or multiple backslashes
							// before the table tag)
							// If there is at least one backslash in the first token
							else if let Some(backslashes_match) = self.backslashes_regex.find(tokens[0])
							{
								// Get the index range of the backslash pattern match
								let backslashes_range = backslashes_match.range();
								// If the backslashes are at the start of the token and right before the table tag
								// (if the entire token is backslashes followed by a table tag)
								if backslashes_range.start == 0 && backslashes_range.end == table_tag_range.start
								{
									// Remove the first backslash from the token
									tokens[0] = &tokens[0][1..];
								}
							}
						}
					}
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
					current_x_min = x_min;
					// Zero the bullet flag to signal that a bullet list isn't being currently processed anymore
					in_bullet_list = false;
				}
				// If this paragraph is right after a table (table flag still set), zero the table flag
				if in_table { in_table = false; }
				// Set the x position to be 0 or 1 tab amounts from the left side of the text box
				// 0 tab amounts for the first paragraph (to match the Player's Handbook formatting)
				// 1 tab amount for all other paragraphs
				self.x = x_min + current_tab_amount;
				// Set the paragraph flag
				in_paragraph = true;
			}
			// Make it so all paragraphs after the first get moved down a newline amount before being processed
			paragraph_newline_scalar = 1.0;
			// TODO: Make it so single tokens that are too long to fit on a line get hyphenated
			// Loop through each token after the first to check if it does something special, if it should be
			// written to the current line, or cause the current line to be wrtten then add it to the next line.
			for mut token in tokens
			{
				// Determine if the current token is a special token
				match token
				{
					// If It's a regular font tag, write the current line to the page and switch the current font
					// variant to regular
					REGULAR_FONT_TAG =>
					{
						self.switch_font_variant(FontVariant::Regular, &mut line, &mut line_width, y_min);
					},
					// If It's a bold font tag, write the current line to the page and switch the current font
					// variant to bold
					BOLD_FONT_TAG =>
					{
						self.switch_font_variant(FontVariant::Bold, &mut line, &mut line_width, y_min);
					},
					// If It's a italic font tag, write the current line to the page and switch the current font
					// variant to italic
					ITALIC_FONT_TAG =>
					{
						self.switch_font_variant(FontVariant::Italic, &mut line, &mut line_width, y_min);
					},
					// If It's a bold-italic font tag, write the current line to the page and switch the current font
					// variant to bold-italic
					BOLD_ITALIC_FONT_TAG | ITALIC_BOLD_FONT_TAG =>
					{
						self.switch_font_variant(FontVariant::BoldItalic, &mut line, &mut line_width, y_min);
					},
					// If it's not a special token
					_ =>
					{
						// Calculate the width of the token
						let mut width = self.calc_text_width(token);
						// If the token is an escaped font tag, remove the first backslash from it so font tags can
						// actually appear in spell text without affecting the font
						if self.is_escaped_font_tag(token)
						{
							token = &token[1..];
							// Also recalculate the width of the token
							width = self.calc_text_width(token);
						}
						// If the next line to be written is currently empty
						if line_width == 0.0
						{
							// If the token is too wide to fit on a single line in the textbox
							if current_x_min + width > x_max
							{
								// Hyphenate the token and get the new starting index and width
								let (index, new_width) =
								self.hyphenate_and_apply_token(token, width, x_max - self.x, y_min, current_x_min);
								// Zero the first line flag since at least 1 line has been written
								first_line = false;
								// Remove the part of the token that was hyphenated
								token = &token[index..];
								// Store the new width of this token
								width = new_width;
								// Calculate the maximum line width based on where the text starts and ends
								let max_line_width = x_max - current_x_min;
								// Keep hyphenating the token until the only part that remains can fit on a single
								// line
								while width > max_line_width
								{
									// Hyphenate the token and get the new starting index and width
									let (index, new_width) = self.hyphenate_and_apply_token
									(
										token,
										width,
										max_line_width,
										y_min,
										current_x_min
									);
									// Remove the part of the token that was hyphenated
									token = &token[index..];
									// Store the new width of this token
									width = new_width;
								}
							}
							// If it's the first token on the first line in a tabbed paragraph and it can't fit on
							// that first line
							else if self.x + width > x_max && self.x == x_min + current_tab_amount && first_line
							{
								// Hyphenate the token and get the new starting index and width
								let (index, new_width) = self.hyphenate_and_apply_token
								(
									token,
									width,
									x_max - x_min - current_tab_amount,
									y_min,
									current_x_min
								);
								// Zero the first line flag since at least 1 line has been written
								first_line = false;
								// Remove the part of the token that was hyphenated
								token = &token[index..];
								// Store the new width of this token
								width = new_width;
								// Calculate the maximum line width based on where the text starts and ends
								let max_line_width = x_max - current_x_min;
								// Keep hyphenating the token until the only part that remains can fit on a single
								// line
								while width > max_line_width
								{
									// Hyphenate the token and get the new starting index and width
									let (index, new_width) = self.hyphenate_and_apply_token
									(
										token,
										width,
										max_line_width,
										y_min,
										current_x_min
									);
									// Remove the part of the token that was hyphenated
									token = &token[index..];
									// Store the new width of this token
									width = new_width;
								}
							}
							else if self.x + width > x_max
							{
								// Set the x position back to the new-line reset point
								self.x = current_x_min;
								// Move the y position down a line
								self.y -= self.current_newline_amount();
							}
							// Put this token at the start of the line
							line = String::from(token);
							// Assign this token's width to the width of the current line
							line_width = width;
						}
						// If the current line is not empty
						else if line_width > 0.0
						{
							// Store a space string
							let space = SPACE;
							// Calculate the width of a space character
							let space_width = self.calc_text_width(space);
							// Calculate the width of the current token with a space in front of it
							// (which is how it would be added to the line)
							let padded_width = space_width + width;
							// Calculate where the line would end if the token was added onto this line
							let new_line_end = self.x + line_width + padded_width;
							// If this token would make the line go past the right side boundry of the textbox
							if new_line_end > x_max
							{
								// If the current token is too wide to fit on a single line in the textbox
								if current_x_min + width > x_max
								{
									// Calculate the maximum line width based on where the text starts and ends
									let max_line_width = x_max - current_x_min;
									// Get a hyphenated part of the token and the index for where the hyphen cuts off
									// in the token
									let (hyphenated_token, index) = self.get_hyphen_str
									(
										token,
										width,
										max_line_width - (self.x - current_x_min) - space_width - line_width
									);
									let hyphenated_token = hyphenated_token.text();
									// If the token can be made to fit on the current line by hyphenating it
									// (might not fit if the current line is near the end or something)
									if index != 0
									{
										// Add a space and the hyphenated token to the end of the line
										line += space;
										line += &hyphenated_token;
									}
									// Apply the line to the spellbook
									self.apply_text(&line, y_min);
									// Zero the first line flag since at least one ine has been applied now
									first_line = false;
									// Set the x position back to the new-line reset point
									self.x = current_x_min;
									// Move the y position down a line
									self.y -= self.current_newline_amount();
									// Take off the part of the token that was hyphenated
									token = &token[index..];
									// Calculate the width of the new shorter token
									width = self.calc_text_width(token);
									// Keep hyphenating the token until the only part that remains can fit on a
									// single line
									while width > max_line_width
									{
										// Hyphenate the token and get the new starting index and width
										let (index, new_width) = self.hyphenate_and_apply_token
										(
											token,
											width,
											max_line_width,
											y_min,
											current_x_min
										);
										// Remove the part of the token that was hyphenated
										token = &token[index..];
										// Store the new width of this token
										width = new_width;
									}
									// Set the line to whatever is left in the token
									line = String::from(token);
									// Store the width of the token as the width of the new line
									line_width = width;
								}
								else
								{
									// Apply the current line
									self.apply_text(&line, y_min);
									// Zero the first line flag since at least one line has been applied now
									first_line = false;
									// Set the x position back to the new-line reset point
									self.x = current_x_min;
									// Move the y position down a line
									self.y -= self.current_newline_amount();
									// Empty the line and put the current token in it to be at the start of the next line
									line = String::from(token);
									// Set the new line width to the width of the current line
									line_width = width;
								}
							}
							// If the token doesn't make the line too wide, add the token to the line
							else
							{
								// Add the token to the line
								line += format!(" {}", token).as_str();
								// Add the width of a space and this token to the line
								line_width += padded_width;
							}
						}
						// If the line width is less than 0, shouldn't be possible
						else { panic!
						("Line width is less than 0.0 in `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::write_textbox`"); }
					}
				};
			}
			// If the current line is empty, move the y position back up a newline amount
			if line_width <= 0.0 { self.y += self.current_newline_amount(); }
			// Write any remaining text to the page
			else { self.apply_text(&line, y_min); }
			// If this was a paragraph, set the current tab amount to be the normal tab amount so all paragraphs
			// after the first are tabbed in on the first line
			if in_paragraph { current_tab_amount = self.tab_amount(); }
		}
		// If a table was the last thing that was applied to the page, move down an extra newline amount to keep
		// whatever comes next more separated from the table (to match the Player's Handbook formatting)
		if in_table { self.y -= self.current_newline_amount(); }
	}

	/// Hyphenates a token, writes the hyphated part of the token to the spellbook, resets the x and y positions to
	/// a new line, and returns the new starting index of the token along with its new width.
	fn hyphenate_and_apply_token
	(
		&mut self, token: &str,
		token_width: f32,
		max_line_width: f32,
		y_min: f32,
		current_x_min: f32
	)
	-> (usize, f32)
	{
		// Get a hyphenated part of the token and the index for where the hyphen cuts off in the token
		let (hyphenated_token, index) = self.get_hyphen_str(token, token_width, max_line_width);
		// Apply the line to the spellbook
		self.apply_text(hyphenated_token.text(), y_min);
		// Set the x position back to the new-line reset point
		self.x = current_x_min;
		// Move the y position down a line
		self.y -= self.current_newline_amount();
		// If there's still some characters left in the token
		if index < token.len()
		{
			// Take off the part of the token that was hyphenated
			let token = &token[index..];
			// Calculate the width of the new shorter token
			let width = self.calc_text_width(token);
			// Return the new starting index and width
			(index, width)
		}
		// If the entire token has been handled already, return the index at the end of the token and a width of 0
		// (no need to worry about an unnecessary hyphen being added onto the pushed hyphen line since the
		// get_hyphen_str method handles that)
		else { (index, 0.0) }
	}

	/// For use in `write_textbox` functions. If the given font variant is different than the current one being used,
	/// it applies the current line of text being processed, empties it, switches the current font variant to the
	/// given one, and resets the line width to 0.
	fn switch_font_variant(&mut self, font_variant: FontVariant, line: &mut String, line_width: &mut f32, y_min: f32)
	{
		// If the current font variant different than the one to switch to
		if *self.current_font_variant() != font_variant
		{
			// Applies the current line of text
			self.apply_text(line.trim_start(), y_min);
			// Empties the line of text
			*line = String::new();
			// Move the cursor over by a space width of the current font type to prevent text of different font types
			// being too close together.
			let space_width = self.calc_text_width(SPACE);
			self.x += space_width;
			// Switches to the desired font variant
			self.set_current_font_variant(font_variant);
			// Resets the line width to 0 since the line is empty now
			*line_width = 0.0;
		}
	}

	/// Writes vertically and horizontally centered text into a fixed sized textbox.
	/// If the text is too big to fit in the textbox, it continues into the next page from the top of the page going
	/// to the bottom and staying within the same horizontal bounds.
	/// This method can also process font variant changes in the text.
	fn write_centered_textbox(&mut self, text: &str, x_min: f32, x_max: f32, y_min: f32, y_max: f32)
	{
		// If either dimensional bounds overlap with each other, do nothing
		if x_min >= x_max || y_min >= y_max { return; }
		// Calculates the actual sizes of the horizontal and vertical dimensions of the textbox
		let textbox_width = x_max - x_min;
		let textbox_height = y_max - y_min;
		// Split the text into lines that will fit horizontally within the textbox
		let lines = self.get_textbox_lines(text, textbox_width, textbox_width);
		// Apply the text lines to the spellbook
		self.apply_centered_text_lines(&lines, textbox_width, textbox_height, x_min, y_min, y_max);
	}

	/// Parses a table and applies it to the spellbook.
	fn parse_table(&mut self, table: &spells::Table, x_min: f32, x_max: f32, y_min: f32, y_max: f32)
	{
		// Set the text type to table body mode
		self.set_current_text_type(TextType::TableBody);
		// Get the width of the widest cell in each column
		let max_column_widths = self.get_max_table_column_widths(&table.column_labels, &table.cells);
		// Calculate and assign widths to each column (as well as whether each column is centered or not)
		let column_width_data = self.get_table_column_width_data(&max_column_widths, x_min, x_max, y_min, y_max);
		// Calculate the width of the entire table
		let table_width = self.get_table_width(&column_width_data);
		// Get a vec of all data about columns needed for writing the table to the spellbook (computes x_min and
		// x_max values for each column and stores whether each column is centered or not)
		let column_data = self.get_column_data(&column_width_data, table_width);
		// Split each column label into lines that will fit within the width of their columns
		let column_label_lines =
		self.get_table_row_lines(&table.column_labels, &column_width_data, FontVariant::Bold);
		// Split each cell in the table into lines that will fit within the column each cell is in
		let cell_lines = self.get_table_lines(&table.cells, &column_width_data);
		// Change the text type and font variant to be in table title mode
		self.set_current_text_type(TextType::TableTitle);
		self.set_current_font_variant(FontVariant::Bold);
		// Split the table title into lines that will fit on the page
		let total_width = x_max - x_min;
		let title_lines = self.get_textbox_lines(&table.title, total_width, total_width);
		// TODO
		// 1. Calculate title height (determines entire table height)
		// 1. Calculate height of each row (determines row color lines height and vertical placement of single line
		// cells)
		// 2. Calculate Entire table height (determine whether table gets place on current page or next page)
		// 3. Apply title
		// 4. Apply color lines
		// 5. Apply table
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
				// Calculate the width of the cell (font switches included)
				let column_width = self.get_textbox_lines
				(
					&cells[row_index][column_index],
					f32::INFINITY,
					f32::INFINITY
				)[0].width();
				// If a max width for this column already exists
				if column_index < column_widths.len()
				{
					// Replace the max width of this column with this cell's width if its bigger than the current max
					// width of this column
					column_widths[column_index].1 = column_widths[column_index].1.max(column_width);
				}
				// If this is a jagged table and a width hasn't been added for this column yet, push this width
				else { column_widths.push((column_index, column_width)); }
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
		x_max: f32,
		y_min: f32,
		y_max: f32
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
			"Failed to compare 2 `f32`s in `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::get_column_widths`: {} and {}",
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
	fn get_table_lines(&mut self, cells: &Vec<Vec<String>>, column_width_data: &Vec<(f32, bool)>)
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

	/// Takes a string along with a maximum width for lines to fit into, separates the string into lines of tokens
	/// that fit within the max width, and returns a vec of those lines.
	fn get_textbox_lines(&mut self, text: &str, first_line_width: f32, textbox_width: f32) -> Vec<TextLine>
	{
		// Get all tokens separated by whitespace
		// Collects it into a vec so the `is_empty` method can be used without having to clone a new iterator.
		let mut tokens: Vec<_> = text.split_whitespace().collect();
		// If there is no text, do nothing
		if tokens.is_empty() { return Vec::new(); }
		// Store the font variant at the start so the current font variant can be reset to it after constructing the
		// lines of text since the current font variant will change while calculating line widths
		let start_font_variant = *self.current_font_variant();
		// Keeps track of the current max textbox width
		// Uses `first_line_width` for the first line and `textbox_width` for all lines after that
		let mut max_width = first_line_width;
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
			match tokens[i]
			{
				// If It's a font tag, add the tag to the line and switch the current font variant so width can be
				// calculated correctly for the following tokens
				REGULAR_FONT_TAG =>
				{
					line.add_font_tag(FontVariant::Regular, self.space_widths());
					self.set_current_font_variant(FontVariant::Regular);
				},
				BOLD_FONT_TAG =>
				{
					line.add_font_tag(FontVariant::Bold, self.space_widths());
					self.set_current_font_variant(FontVariant::Bold);
				},
				ITALIC_FONT_TAG =>
				{
					line.add_font_tag(FontVariant::Italic, self.space_widths());
					self.set_current_font_variant(FontVariant::Italic);
				},
				BOLD_ITALIC_FONT_TAG | ITALIC_BOLD_FONT_TAG =>
				{
					line.add_font_tag(FontVariant::BoldItalic, self.space_widths());
					self.set_current_font_variant(FontVariant::BoldItalic);
				},
				// If it's not a special token, calculate its width and determine what to do from there
				_ =>
				{
					// If the token is an escaped font tag, remove the first backslash at the start
					if self.is_escaped_font_tag(tokens[i]) { tokens[i] = &tokens[i][1..]; }
					let mut width = 0.0;
					// Hyphenate the token if it's too long to fit on a line and compute its width
					(tokens[i], width) = self.hyphenate_token
					(
						tokens[i],
						&mut max_width,
						textbox_width,
						&mut line,
						&mut lines
					);
					// If the line is currently empty
					if line.width() == 0.0
					{
						// Put the token into the line
						let text_token = TextToken::with_width(tokens[i], width);
						line.add_text(text_token, self.space_widths());
					}
					// If the line is not empty
					else if line.width() > 0.0
					{
						// Calculate the width of the current token with a space in front of it
						let padded_width = self.get_current_space_width() + width;
						// If adding this token to the line would make it go outside the textbox,
						// apply the current line and set it to just the current token
						if line.width() + padded_width > max_width
						{
							// Make sure the line doesn't have any excess capacity in its vec
							line.shrink_to_fit();
							// Add the current line to the vec of lines
							lines.push(line);
							// Create a new line with the capacity of the number of remaining tokens
							line = TextLine::with_capacity
							(
								tokens.len() - i,
								*self.current_text_type(),
								*self.current_font_variant()
							);
							// Add the token to the start of the new line
							let text_token = TextToken::with_width(tokens[i], width);
							line.add_text(text_token, self.space_widths());
							// Set the max width width to the textbox width in case the previous line was the first
							// line
							max_width = textbox_width;
						}
						// If this token can fit on the line, add it to the line
						else
						{
							// Add this token to the line
							let text_token = TextToken::with_width(tokens[i], width);
							line.add_text(text_token, self.space_widths());
							// Set the max width width to the textbox width in case the previous line was the first
							// line
							max_width = textbox_width;
						}
					}
					// If the line has a negative width
					else
					{
						panic!("Line width is less than 0.0 in `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::get_textbox_lines`");
					}
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
	fn get_text_token(&self, token: &str, font_variant: FontVariant) -> TextToken
	{
		let font_size_data = self.font_data.get_size_data_for(font_variant);
		let scalar = self.font_data.get_scalar_for(font_variant);
		TextToken::new(token, font_size_data, self.current_font_scale(), scalar)
	}

	/// Returns whether or not a token / string is an escaped font tag (font tag with any amount of backslashes
	/// before it).
	fn is_escaped_font_tag(&self, token: &str) -> bool
	{
		// Determine whether or not there is an escaped font tag in the token
		match self.escaped_font_tag_regex.find(token)
		{
			// If there is an escaped font tag in the token
			Some(pat_match) =>
			{
				// If the escaped font tag is the entire token
				if pat_match.range() == (Range { start: 0, end: token.len() }) { true }
				else { false }
			},
			None => false
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
		max_width: &mut f32,
		textbox_width: f32,
		current_line: &mut TextLine,
		lines: &mut Vec<TextLine>
	)
	-> (&'t str, f32)
	{
		// Get the original token's length to be able to tell if it was hyphenated or not
		let og_token_len = token.len();
		let mut width = 0.0;
		// If the token was too wide to fit on a the first line, hyphenate it and return its width
		// Otherwise, just return the token the way it is and its width
		(token, width) = self.hyphenate_once(token, *max_width - current_line.width(), current_line, lines);
		// If the token was hyphenated (different length), set the current max width to the actual textbox width
		// since its definitely not the first line anymore.
		if token.len() != og_token_len { *max_width = textbox_width; }
		// Otherwise, return the original token and its width
		else { return (token, width); }
		// Hyphenate the token until just the end of it remains and it can fit on a single line
		while width > textbox_width
		{
			(token, width) = self.hyphenate_once(token, textbox_width, current_line, lines);
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
		textbox_width: f32,
		current_line: &mut TextLine,
		lines: &mut Vec<TextLine>
	)
	-> (&'t str, f32)
	{
		// Calculate the width of the token
		let mut width = self.calc_text_width(token);
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
				// Add the hyphenated part of the token to the current line
				current_line.add_text(hyphenated_token, self.space_widths());
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
		let mut hyphenated_string = String::new();
		// Keeps track of the width of the hyphenated part of the text
		let mut hyphen_str_width = token_width;
		// If the string can fit in the textbox, return itself and its length
		if hyphen_str_width <= textbox_width { return (TextToken::empty(), text.len()); }
		// Lower and upper possible bounds for what the index could be
		let mut lower_bound = 0;
		let mut upper_bound = text.len();
		// The current index being tested
		let mut index = upper_bound / 2;
		let mut last_index = index;
		// Whether or not the index had just gone down or up
		let mut went_up = false;
		// Do - While loop until index and last_index are equal
		// Binary search for the index where the text plus a hyphen at the end is as long as possible without going
		// outside the textbox
		while
		{
			// Store index in last index
			last_index = index;
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
				// Zero the went up flag
				went_up = true;
			}
			// If the width is greater than the width of the textbox
			else
			{
				// Decrease the upper bound to the index
				upper_bound = index;
				// Decrease the index to be between the lower and upper bound
				index -= (upper_bound - lower_bound) / 2;
				// Set the went up flag
				went_up = false;
			}
			// Do - While condition
			index != last_index
		}{}
		// If the index is 0, return that irregardless of whether it went up or down to signal that it can't fit
		// within the width of this textbox at all
		if index == 0
		{
			let new_token = TextToken::empty();
			(new_token, index)
		}
		else
		{
			// If the index tried to go up on the last iteration, increase the cutoff / delimiter index by 1 along
			// and change the hyphenated string and its width to match the index change
			if went_up
			{
				index += 1;
				hyphenated_string = format!("{}-", &text[0..index]);
				hyphen_str_width = self.calc_text_width(&hyphenated_string);
			}
			let new_token = TextToken::with_width(&hyphenated_string, hyphen_str_width);
			(new_token, index)
		}
	}

	/// Applies lines of text to the spellbook so that each line is centered horizontally and all of the lines are
	/// centered horizontally if possible.
	fn apply_centered_text_lines
	(
		&mut self,
		text_lines: &Vec<TextLine>,
		textbox_width: f32,
		textbox_height: f32,
		x_min: f32,
		y_min: f32,
		y_max: f32
	)
	{
		// Calculate how many lines this text is going to be
		let max_lines = (textbox_height / self.current_newline_amount()).floor() as usize;
		// If There are more lines than can fit on the page, set the y value to the top of the textbox
		// (text on following pages will start at the top of the entire page but stay within the horizontal
		// boundries of the textbox)
		if text_lines.len() > max_lines { self.y = y_max; }
		// If all the lines can fit on one page, calculate what y value to start the text at so it is vertically
		// centered in the textbox and set the y value to that
		else { self.y = (y_max / 2.0) + (text_lines.len() - 1) as f32 / 2.0 * self.current_newline_amount(); }
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
			self.apply_text_line(line, y_min);
		}
	}

	/// Applies a single line of text to the current page in the spellbook.
	fn apply_text_line(&mut self, line: &TextLine, y_min: f32)
	{
		// Keeps track of what index in the line to start at when applying tokens to the page
		let mut last_index = 0;
		let tokens = line.tokens();
		// Loop through all of the tokens to find font tags
		for index in 0..tokens.len()
		{
			match &tokens[index]
			{
				// If the current token is a font tag
				Token::FontTag(font_variant) =>
				{
					// If the font tag is different than the current font
					if *font_variant != *self.current_font_variant()
					{
						// Get a vec of strings of all the previous tokens
						let next_line: &Vec<_> =
						&tokens[last_index..index].iter().map(|token| token.as_spellbook_string()).collect();
						// Join those tokens together with spaces and apply them to the page
						self.apply_text(next_line.join(SPACE).as_str(), y_min);
						// If this isn't the last token in the line, apply another space to the page
						if index < tokens.len() - 1
						{
							self.apply_text(SPACE, y_min);
						}
						// Set the current font variant so the following tokens will be applied correctly
						self.set_current_font_variant(*font_variant);
						// Increase the index to start applying tokens at to be after this font tag token
						last_index = index + 1;
					}
				},
				Token::Text(text) => ()
			}
		}
		// Get a vec of strings of all the previous tokens
		let next_line: &Vec<_> =
		&tokens[last_index..].iter().map(|token| token.as_spellbook_string()).collect();
		// Join those tokens together withs spaces and apply them to the page
		self.apply_text(next_line.join(SPACE).as_str(), y_min);
	}

	/// Writes a line of text to a page.
	/// Moves to a new page / creates a new page if the text is below a certain y value.
	fn apply_text(&mut self, text: &str, y_min: f32)
	{
		// If there is no text to apply, do nothing
		if text.is_empty() { return; }
		// Checks to see if the text should be applied to the next page or if a new page should be created.
		self.check_for_new_page(y_min);
		// Create a new text section on the page
		self.layers[self.current_page_index].begin_text_section();
		// Set the text cursor to the current x and y position of the text
		self.layers[self.current_page_index].set_text_cursor(Mm(self.x), Mm(self.y));
		// Set the font and font size of the text
		self.layers[self.current_page_index].set_font(self.current_font_ref(), self.current_font_size());
		// Set the text color
		self.layers[self.current_page_index].set_fill_color(self.current_text_color().clone());
		// Write the text to the page
		self.layers[self.current_page_index].write_text(text, self.current_font_ref());
		// End the text section on the page
		self.layers[self.current_page_index].end_text_section();
		// Move the x position to be at the end of the newly applied line
		self.x += self.calc_text_width(&text);
	}

	/// Checks if the current layer should move to the next page if the text y position is below given `y_min` value.
	/// Sets the y position to the top of the page if the function moves the text to a new page.
	/// Creates a new page if the page index goes beyond the number of layers that exist.
	fn check_for_new_page(&mut self, y_min: f32)
	{
		// If the y level is below the bottom of where text is allowed on the page
		if self.y < y_min
		{
			// Increase the current page index to the layer for the next page
			self.current_page_index += 1;
			// If the index is beyond the number of layers in the document
			if self.current_page_index >= self.layers.len()
			{
				// Create a new page
				self.make_new_page();
			}
			// Move the y position of the text to the top of the page
			self.y = self.y_max();
		}
	}

	/// Adds a new page to the pdf document, including the background image and page number if options for those were
	/// given. Sets `current_page_index` to the new page.
	fn make_new_page(&mut self)
	{
		// Create a new page
		let (page, layer) = self.doc.add_page
		(
			Mm(self.page_width()),
			Mm(self.page_height()),
			format!("{} {}", LAYER_NAME_PREFIX, self.layers.len())
		);
		// Get the layer for the new page
		let layer_ref = self.doc.get_page(page).get_layer(layer);
		// Add the new layer and page to the vecs holding them
		self.layers.push(layer_ref);
		self.pages.push(page);
		// Update the current page index to point to the new page
		self.current_page_index = self.layers.len() - 1;
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
			// Construct a `printpdf::Image` from the `image::DynamicImage`
			// Note: Cannot store a `printpdf::Image` in the background struct because of ownership issues and
			// lacking implementations of the `printpdf::Image` struct from the `printpdf` crate.
			let image = Image::from_dynamic_image(&background.image().clone());
			// Add the image to the current layer with the given transform data
			image.add_to_layer(self.current_layer().clone(), *background.transform());
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
				// Set the page fill color to the color of the page numbers
				self.layers[self.current_page_index].set_fill_color(data.color().clone());
				// Apply the page number to the document
				self.layers[self.current_page_index].use_text
				(
					&text,
					data.font_size(),
					Mm(x),
					Mm(data.bottom_margin()),
					data.font_ref()
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

	/// Calculates the width of some text using the current state of this object's font data field.
	fn calc_text_width(&self, text: &str) -> f32
	{
		calc_text_width(text, self.current_size_data(), self.current_font_scale(), self.current_scalar())
	}

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

	fn document(&self) -> &PdfDocumentReference { &self.doc }
	fn layers(&self) -> &Vec<PdfLayerReference> { &self.layers }
	fn pages(&self) -> &Vec<PdfPageIndex> { &self.pages }
	fn current_page_index(&self) -> usize { self.current_page_index }
	fn current_page_num(&self) -> i64 { self.current_page_num }
	fn font_data(&self) -> &FontData { &self.font_data }
	fn page_size_data(&self) -> &PageSizeData { &self.page_size_data }
	fn page_number_data(&self) -> &Option<PageNumberData> { &self.page_number_data }
	fn background(&self) -> &Option<BackgroundImage> { &self.background }
	fn table_options(&self) -> &TableOptions { &self.table_options }
	fn space_widths(&self) -> &SpaceWidths { &self.space_widths }
	/// Current x position of the text
	fn x(&self) -> &f32 { &self.x }
	/// Current y position of the text
	fn y(&self) -> &f32 { &self.y }

	// Layer Getters

	fn current_layer(&self) -> &PdfLayerReference
	{
		&self.layers[self.current_page_index]
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

	// The text color of the page number.
	fn page_number_color(&self) -> Option<&Color>
	{
		match &self.page_number_data
		{
			Some(data) => Some(data.color()),
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

	// Space Width Getters

	fn get_current_space_width(&self) -> f32
	{ self.space_widths.get_width_for(*self.current_text_type(), *self.current_font_variant()) }

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
