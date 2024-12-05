//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//	Utility Functions
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;
use std::error::Error;

pub use printpdf::{PdfDocumentReference, PdfLayerReference};

use crate::spellbook_writer::*;

pub use crate::spells;
pub use crate::spellbook_options::*;

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
	text_colors: TextColorOptions,
	page_size_options: PageSizeOptions,
	page_number_options: Option<PageNumberOptions>,
	background: Option<(&str, ImageTransform)>,
	table_options: TableOptions
)
-> Result<(PdfDocumentReference, Vec<PdfLayerReference>), Box<dyn Error>>
{
	SpellbookWriter::create_spellbook
	(
		title,
		spells,
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		page_number_options,
		background,
		table_options
	)
}

/// Saves spellbooks to a file as a pdf document.
///
/// # Parameters
///
/// - `doc` A spellbook that gets returned from `generate_spellbook()`.
/// - `file_name` The name to give to the file that the spellbook will be saved to.
/// 
/// # Output
///
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
impl Error for SpellFileNameReadError {}

/// Returns a vec of spells from every spell file in a folder.
///
/// It only uses files that end in the `.spell` extension.
/// 
/// # Parameters
///
/// - `folder_path` The file path to the folder to extract every spell from.
/// 
/// # Output
///
/// - `Ok` Returns a vec of spell objects that can be inputted into `generate_spellbook()`.
/// - `Err` Returns any errors that occurred.
pub fn get_all_spells_in_folder(folder_path: &str) -> Result<Vec<spells::Spell>, Box<dyn std::error::Error>>
{
	// Gets a list of every file in the folder
	let file_paths = fs::read_dir(folder_path)?;
	// Create a list of the spells that will be returned
	// Can't figure out how to count the number of files in file_paths to build vec with exact capacity
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
/// # Parameters
///
/// - `folder_path` The file path to the folder to extract every spell from.
/// 
/// # Output
///
/// - `Ok` Returns a vec of spell objects that can be inputted into `generate_spellbook()`.
/// - `Err` Returns any errors that occurred.
pub fn get_all_json_spells_in_folder(folder_path: &str) -> Result<Vec<spells::Spell>, Box<dyn std::error::Error>>
{
	// Gets a list of every file in the folder
	let file_paths = fs::read_dir(folder_path)?;
	// Create a list of the spells that will be returned
	// Can't figure out how to count the number of files in file_paths to build vec with exact capacity
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
