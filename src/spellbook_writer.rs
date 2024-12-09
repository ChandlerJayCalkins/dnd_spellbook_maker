//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//	Spellbook PDF document generation
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;
use std::cell::Ref;
use std::error::Error;
use std::ops::Range;
use std::cmp::min;

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
			table_options: table_options,
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
		let escaped_font_tag_pattern = Regex::new(&escaped_font_tag_pattern)
		.expect(format!
		(
			"Failed to build regex pattern \"{}\" in `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::write_textbox`",
			escaped_font_tag_pattern
		).as_str());
		// Create a regex pattern to find table tags
		let table_tag_pattern = "\\[table\\]\\[[0-9]+\\]";
		let table_tag_pattern = Regex::new(table_tag_pattern)
		.expect(format!
		(
			"Failed to build regex pattern \"{}\" in `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::write_textbox`",
			table_tag_pattern
		).as_str());
		// Create a regex pattern to find repeating backslashes
		let backslashes_pattern = "\\\\+";
		let backslashes_pattern = Regex::new(backslashes_pattern)
		.expect(format!
		(
			"Failed to build regex pattern \"{}\" in `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::write_textbox`",
			backslashes_pattern
		).as_str());
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
				// If there is a table tag (ex: "[table][5]", "[table][0]", etc.) in the first token
				if let Some(pat_match) = table_tag_pattern.find(tokens[0])
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
								// Move the y position down an extra newline amount to separate it more from normal
								// paragraphs (to match the Player's Handbook formatting)
								// Moves the y position down 0 newlines on the first paragraph, 0 on all others.
								self.y -= paragraph_newline_scalar * self.current_newline_amount();
								// Set the table flag to signal that a table is being processed
								in_table = true;
							}
							// If this table is right after a bullet list (bullet flag still set)
							if in_bullet_list
							{
								// Set the value that the x position resets to so that it lines up with the left side
								// of the text box again
								current_x_min = x_min;
								// Zero the bullet flag to signal that a bullet list isn't being currently processed
								// anymore
								in_bullet_list = false;
							}
							// Zero the paragraph flag
							in_paragraph = false;
							// Make it so all paragraphs after the first get moved down a newline amount before being
							// processed
							paragraph_newline_scalar = 1.0;
							// TODO: Add code to put in a table
							self.x = x_min;
							self.apply_text_line(tokens[0], y_min);
							// Skip the token loop below and move to the next paragraph
							continue;
						}
						// Check to see if this is an escaped table tag (a backslash or multiple backslashes before
						// the table tag)
						// If there is at least one backslash in the first token
						else if let Some(backslashes_match) = backslashes_pattern.find(tokens[0])
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
						if let Some(pat_match) = escaped_font_tag_pattern.find(token)
						{
							if pat_match.range() == (Range { start: 0, end: token.len() })
							{
								token = &token[1..];
								// Also recalculate the width of the token
								width = self.calc_text_width(token);
							}
						}
						// If the next line to be written is currently empty
						if line_width == 0.0
						{
							// If the token is too wide to fit on a single line in the textbox
							if current_x_min + width > x_max
							{
								// Get a hyphenated part of the token and the index for where the hyphen cuts off in
								// the token
								let (hyphenated_token, _, index) = self.get_hyphen_str(token, x_max - self.x);
								// Apply the line to the spellbook
								self.apply_text_line(&hyphenated_token, y_min);
								// Set the x position back to the new-line reset point
								self.x = current_x_min;
								// Move the y position down a line
								self.y -= self.current_newline_amount();
								// Zero the first line flag since at least 1 line has been written
								first_line = false;
								// Take off the part of the token that was hyphenated
								token = &token[index..];
								// Calculate the width of the new shorter token
								width = self.calc_text_width(token);
								// Calculate the maximum line width based on where the text starts and ends
								let max_line_width = x_max - current_x_min;
								// Keep hyphenating the token until the only part that remains can fit on a single
								// line
								while width > max_line_width
								{
									// Get a hyphenated part of the token and the index for where the hyphen cuts off
									// in the token
									let (hyphenated_token, _, index) = self.get_hyphen_str(token, max_line_width);
									self.apply_text_line(&hyphenated_token, y_min);
									// Set the x position back to the new-line reset point
									self.x = current_x_min;
									// Move the y position down a line
									self.y -= self.current_newline_amount();
									// If the entire token has been handled already, empty the token, set the width
									// to 0, and return (no need to worry about an unnecessary hyphen being added
									// onto the pushed hyphen line since the get_hyphen_str method handles that)
									if index >= token.len()
									{
										token = "";
										width = 0.0;
										break;
									}
									// Take off the part of the token that was hyphenated
									token = &token[index..];
									// Calculate the width of the new shorter token
									width = self.calc_text_width(token);
								}
							}
							// If it's the first token on the first line in a tabbed paragraph and it can't fit on
							// that first line
							else if self.x + width > x_max && self.x == x_min + current_tab_amount && first_line
							{
								// Get a hyphenated part of the token and the index for where the hyphen cuts off in
								// the token
								let (hyphenated_token, _, index) =
								self.get_hyphen_str(token, x_max - x_min - current_tab_amount);
								// Apply the line to the spellbook
								self.apply_text_line(&hyphenated_token, y_min);
								// Set the x position back to the new-line reset point
								self.x = current_x_min;
								// Move the y position down a line
								self.y -= self.current_newline_amount();
								// Zero the first line flag since at least 1 line has been written
								first_line = false;
								// Take off the part of the token that was hyphenated
								token = &token[index..];
								// Calculate the width of the new shorter token
								width = self.calc_text_width(token);
								// Calculate the maximum line width based on where the text starts and ends
								let max_line_width = x_max - current_x_min;
								// Keep hyphenating the token until the only part that remains can fit on a single
								// line
								while width > max_line_width
								{
									// Get a hyphenated part of the token and the index for where the hyphen cuts off
									// in the token
									let (hyphenated_token, _, index) = self.get_hyphen_str(token, max_line_width);
									// Apply the line to the spellbook
									self.apply_text_line(&hyphenated_token, y_min);
									// Set the x position back to the new-line reset point
									self.x = current_x_min;
									// Move the y position down a line
									self.y -= self.current_newline_amount();
									// If the entire token has been handled already, empty the token, set the width
									// to 0, and return (no need to worry about an unnecessary hyphen being added
									// onto the pushed hyphen line since the get_hyphen_str method handles that)
									if index >= token.len()
									{
										token = "";
										width = 0.0;
										break;
									}
									// Take off the part of the token that was hyphenated
									token = &token[index..];
									// Calculate the width of the new shorter token
									width = self.calc_text_width(token);
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
							// Calculate the width of the current token with a space in front of it
							// (which is how it would be added to the line)
							let padded_width = self.calc_text_width(" ") + width;
							// Calculate where the line would end if the token was added onto this line
							let new_line_end = self.x + line_width + padded_width;
							// If this token would make the line go past the right side boundry of the textbox,
							// Apply the current line and reset it to just the current token
							if new_line_end > x_max
							{
								// If the current token is too wide to fit on a single line in the textbox
								if current_x_min + width > x_max
								{
									// Calculate the maximum line width based on where the text starts and ends
									let max_line_width = x_max - current_x_min;
									// Get a hyphenated part of the token and the index for where the hyphen cuts off
									// in the token
									let (hyphenated_token, _, index) =
									self.get_hyphen_str(token, max_line_width - self.x - line_width);
									// Add a space and the hyphenated token to the end of the line
									line += " ";
									line += &hyphenated_token;
									// Apply the line to the spellbook
									self.apply_text_line(&line, y_min);
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
										// Get a hyphenated part of the token and the index for where the hyphen cuts
										// off in the token
										let (hyphenated_token, _, index) =
										self.get_hyphen_str(token, max_line_width);
										// Apply the line to the spellbook
										self.apply_text_line(&hyphenated_token, y_min);
										// Set the x position back to the new-line reset point
										self.x = current_x_min;
										// Move the y position down a line
										self.y -= self.current_newline_amount();
										// If the entire token has been handled already, empty the token, set the
										// width to 0, and return (no need to worry about an unnecessary hyphen being
										// added onto the pushed hyphen line since the get_hyphen_str method handles
										// that)
										if index >= token.len()
										{
											token = "";
											width = 0.0;
											break;
										}
										// Take off the part of the token that was hyphenated
										token = &token[index..];
										// Calculate the width of the new shorter token
										width = self.calc_text_width(token);
									}
									line = String::from(token);
									line_width = width;
								}
								else
								{
									// Apply the current line
									self.apply_text_line(&line, y_min);
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
			else { self.apply_text_line(&line, y_min); }
			// If this was a paragraph, set the current tab amount to be the normal tab amount so all paragraphs
			// after the first are tabbed in on the first line
			if in_paragraph { current_tab_amount = self.tab_amount(); }
		}
		// If a table was the last thing that was applied to the page, move down an extra newline amount to keep
		// whatever comes next more separated from the table (to match the Player's Handbook formatting)
		if in_table { self.y -= self.current_newline_amount(); }
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
		// The number of newlines to go down by at the start of each line
		// Is 0.0 for the first line
		// Is 1.0 for all other lines
		let mut newline_scalar = 0.0;
		// Get all tokens separated by whitespace
		// Collects it into a vec so the `is_empty` method can be used without having to clone a new iterator.
		let tokens: Vec<_> = text.split_whitespace().collect();
		// If there is no text, do nothing
		if tokens.is_empty() { return; }
		// Store the font variant at the start so the current font variant can be reset to it after constructing the
		// lines of text since the current font variant will change while calculating line widths
		let start_font_variant = *self.current_font_variant();
		// Vector containing each line of text to write to the textbox and the width of that textbox
		let mut lines: Vec<(String, f32)> = Vec::with_capacity(1);
		// String of the current line being measured
		let mut line = String::new();
		// Width of the current line being measured
		let mut line_width: f32 = 0.0;
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
		let escaped_font_tag_pattern = Regex::new(&escaped_font_tag_pattern)
		.expect(format!
		(
			"Failed to build regex pattern \"{}\" in `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::write_textbox`",
			escaped_font_tag_pattern
		).as_str());
		// Loop through each token to measure how many lines there will be and how long each line is
		for mut token in tokens
		{
			match token
			{
				// If It's a font tag, add the tag to the line without adding the width and switch the current font
				// variant so width can be calculated correctly for the following tokens
				REGULAR_FONT_TAG =>
				{
					line = String::from(format!("{} {}", line, token).trim());
					self.set_current_font_variant(FontVariant::Regular);
				},
				BOLD_FONT_TAG =>
				{
					line = String::from(format!("{} {}", line, token).trim());
					self.set_current_font_variant(FontVariant::Bold);
				},
				ITALIC_FONT_TAG =>
				{
					line = String::from(format!("{} {}", line, token).trim());
					self.set_current_font_variant(FontVariant::Italic);
				},
				BOLD_ITALIC_FONT_TAG | ITALIC_BOLD_FONT_TAG =>
				{
					line = String::from(format!("{} {}", line, token).trim());
					self.set_current_font_variant(FontVariant::BoldItalic);
				},
				// If it's not a special token, calculate its width and determine what to do from there
				_ =>
				{
					// If the token is an escaped font tag, remove the first backslash from it so font tags can
					// actually appear in spell text without affecting the font
					if let Some(pat_match) = escaped_font_tag_pattern.find(token)
					{
						if pat_match.range() == (Range { start: 0, end: token.len() })
						{ token = &token[1..]; }
					}
					// If the line is currently empty
					if line_width == 0.0
					{
						// If the token is too large to fit on a single line, hyphenate it until it fits and get the
						// width of the reamining token
						// Calculate the width of the token
						let mut width = self.calc_text_width(token);
						// If the current token is too big to fit in the textbox
						// Hyphenate the token, add it as a line, and remove the hyphenated part from the token until
						// the remaining token can fit on a single line in the textbox
						while width > textbox_width
						{
							// Get a hyphenated part of the token and the index for where the hyphen cuts off in the
							// token
							let (hyphenated_token, hyphen_token_width, index) =
							self.get_hyphen_str(token, textbox_width);
							// Add the hyphenated part of the token as a line
							lines.push((hyphenated_token, hyphen_token_width));
							// If the entire token has been handled already, empty the token, set the width to 0, and
							// return (no need to worry about an unnecessary hyphen being added onto the pushed
							// hyphen line since the get_hyphen_str method handles that)
							if index >= token.len()
							{
								token = "";
								width = 0.0;
								break;
							}
							// Take part of the token that was hyphenated off of it
							token = &token[index..];
							// Recalculate the width of the token
							width = self.calc_text_width(token);
						}
						// If there's any token remaining after being hyphenated
						if width > 0.0
						{
							// Adds the current token to the start of the line
							// (Adds a space before it in case there is a font tag / are font tags at the front and
							// trim it to remove the space at the front if there are no font tags
							line = String::from(format!("{} {}", line, token).trim());
							// Sets the current line width to the width of this token
							line_width += width;
						}
					}
					// If the line is not empty
					else if line_width > 0.0
					{
						// Calculate the width of the current token with a space in front of it
						// (which could be added to the line)
						let padded_token = format!(" {}", token);
						let padded_width = self.calc_text_width(&padded_token);
						// If adding this token to the line would make it go outside the textbox,
						// Apply the current line and reset it to just the current token
						if line_width + padded_width > textbox_width
						{
							// Calculate the width of the token
							let mut width = self.calc_text_width(token);
							// If the current token is too wide to fit in the textbox
							if width > textbox_width
							{
								// Calculate the width of a space
								let space_width = self.calc_text_width(" ");
								// Hyphenate the first line of the token within the remaining space on the current
								// line (calculate remaining line width by subtracting current line width and space
								// width from the entire textbox width)
								let (hyphenated_token, hyphen_token_width, index) =
								self.get_hyphen_str(token, textbox_width - line_width - space_width);
								// Add the hyphenated part of the token to the line with a space at the start
								line += " ";
								line += &hyphenated_token;
								// Add the hyphenated token width and the width of the space to the total line width
								line_width += space_width + hyphen_token_width;
								// Push the current line to the lines vec
								lines.push((line, line_width));
								// Take the part of the token that was hyphenated out of it
								token = &token[index..];
								// Hyphenate the rest of the token
								// Calculate the width of the token
								width = self.calc_text_width(token);
								// If the current token is too big to fit in the textbox
								// Hyphenate the token, add it as a line, and remove the hyphenated part from the
								// token until the remaining token can fit on a single line in the textbox
								while width > textbox_width
								{
									// Get a hyphenated part of the token and the index for where the hyphen cuts off
									// in the token
									let (hyphenated_token, hyphen_token_width, index) =
									self.get_hyphen_str(token, textbox_width);
									// Add the hyphenated part of the token as a line
									lines.push((hyphenated_token, hyphen_token_width));
									// If the entire token has been handled already, empty the token, set the width
									// to 0, and return (no need to worry about an unnecessary hyphen being added
									// onto the pushed hyphen line since the get_hyphen_str method handles that)
									if index >= token.len()
									{
										token = "";
										width = 0.0;
										break;
									}
									// Take part of the token that was hyphenated off of it
									token = &token[index..];
									// Recalculate the width of the token
									width = self.calc_text_width(token);
								}
								// Set what's left of the token to the current line
								line = String::from(token);
								// Set the width of the token to the line width
								line_width = width;
							}
							else
							{
								// Add the current line to the vec of lines
								lines.push((line, line_width));
								// Empties the current line and puts the current token at the start of the next one
								line = String::from(token);
								// Sets the new current line width to the width of the current token
								line_width = width;
							}
						}
						// If this token can fit on the line, add it to the line and add the width of a space and
						// this token to the width of the line
						else
						{
							// Add a space and this token to the end of this line
							line += &padded_token;
							// Add the width of a space and this token added to the width of the line
							line_width += padded_width;
						}
					}
					else { panic!(
					"Line width is less than 0.0 in `dnd_spellbook_maker::spellbook_writer::SpellbookWriter::write_centered_textbox`"); }
				}
			}
		}
		// Push the remaining text in the last line to the vec of lines
		lines.push((line, line_width));
		// Set the font variant back to what it's supposed to be at the start of the text
		self.set_current_font_variant(start_font_variant);
		// Calculate how many lines this text is going to be
		let max_lines = (textbox_height / self.current_newline_amount()).floor() as usize;
		// If There are more lines than can fit on the page, set the y value to the top of the textbox
		// (text on following pages will start at the top of the entire page but stay within the horizontal
		// boundries of the textbox)
		if lines.len() > max_lines { self.y = y_max; }
		// If all the lines can fit on one page, calculate what y value to start the text at so it is vertically
		// centered in the textbox and set the y value to that
		else { self.y = (y_max / 2.0) + (lines.len() - 1) as f32 / 2.0 * self.current_newline_amount(); }
		// The number of newlines to go down by before each line is printed
		// Is 0.0 for the first line (so the textbox doesn't get moved down by an extra newline)
		// Is 1.0 for all other lines
		let mut newline_scalar = 0.0;
		// Loop through each line to apply it to the document
		for (line, width) in lines
		{
			// Move the y position down by 0 or 1 newline amounts
			// 0 newlines for the first line (so the textbox doesn't get moved down by an extra newline)
			// 1 newline for all other lines
			self.y -= newline_scalar * self.current_newline_amount();
			// Make it so all lines after the first will move down 1 newline amount before being applied to the page
			newline_scalar = 1.0;
			// Calculate where to set the x position so that the line is horizontally centered in the textbox and set
			// the x value to that
			self.x = (textbox_width / 2.0) - (width / 2.0) + x_min;
			// Apply the line to the page
			self.apply_text_line(&line, y_min);
		}
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
			self.apply_text_line(&line, y_min);
			// Empties the line of text
			*line = String::new();
			// Move the cursor over by a space width of the current font type to prevent text of different font types
			// being too close together.
			let space_width = self.calc_text_width(" ");
			self.x += space_width;
			// Switches to the desired font variant
			self.set_current_font_variant(font_variant);
			// Resets the line width to 0 since the line is empty now
			*line_width = 0.0;
		}
	}

	/// Takes a string that is too wide to fit on a single line in a textbox and finds the cutoff / delimiter index
	/// so that `&text[0..index] + '-'` fits inside the textbox, along with that hyphenated string itself
	fn get_hyphen_str(&self, text: &str, textbox_width: f32) -> (String, f32, usize)
	{
		// Keeps track of the last hyphenated part of the text that was measured
		let mut hyphenated_string = String::new();
		// Keeps track of the width of the hyphenated part of the text
		let mut hyphen_str_width = self.calc_text_width(text);
		// If the string can fit in the textbox, return itself and its length
		if hyphen_str_width <= textbox_width { return (String::from(text), hyphen_str_width, text.len()) }
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
			if hyphen_str_width == textbox_width { return (hyphenated_string, hyphen_str_width, index); }
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
		if index == 0 { (String::new(), 0.0, index) }
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
			(hyphenated_string, hyphen_str_width, index)
		}
	}

	/// Writes a line of text to a page.
	/// Moves to a new page / creates a new page if the text is below a certain y value.
	fn apply_text_line(&mut self, text: &str, y_min: f32)
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
		let width = self.current_size_data().layout(text, *self.current_font_scale(), point(0.0, 0.0))
			.map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
			.last()
			.unwrap_or(0.0);
		width * self.current_scalar()
	}

	/// Calculates the text width of a page number.
	fn calc_page_number_width(&self, page_number_text: &str) -> f32
	{
		let width = self.page_number_font_size_data().unwrap().layout
		(
			page_number_text,
			*self.page_number_font_scale().unwrap(),
			point(0.0, 0.0)
		)
		.map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
		.last()
		.unwrap_or(0.0);
		width * self.page_number_font_scalar().expect
		(
			"Called `SpellbookWriter::calc_page_number_width()` with no page number data."
		)
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
