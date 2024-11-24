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

pub fn generate_spellbook(spellbook_name: &str, spell_list: &Vec<spells::Spell>, font_paths: FontPaths,
font_sizes: FontSizes, font_scalars: FontScalars, spacing_options: SpacingOptions, text_colors: TextColors,
page_size_options: PageSizeOptions, page_number_options: Option<PageNumberOptions>,
background: Option<(&str, ImageTransform)>, table_options: TableOptions)
-> Result<(PdfDocumentReference, Vec<PdfLayerReference>), Box<dyn Error>>
{
	todo!()
}

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
impl Error for SpellFileNameReadError {}

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
