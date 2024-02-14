# dnd_spellbook_maker
Library for making pdf documents of spells that a D&D character has.

## Quickstart

### Cargo.toml Dependency

```toml
dnd_spellbook_maker = "0.1.0"
```

or

```toml
dnd_spellbook_maker = { git = "https://github.com/ChandlerJayCalkins/dnd_spellbook_maker" }
```

### Example Program

This program makes a spellbook. It assumes the directory it is in has "spells", "fonts", and "img" folders populated with certain files, including subdirectories with spell files in the "spell" folder.

```Rust
use dnd_spellbook_maker;

fn main()
{
	// Spellbook's name
	let spellbook_name = "A Spellcaster's Spellbook";
	// Vec of spells that will be added to spellbook
	let mut spell_list = Vec::new();
	// Vec of paths to spell files that will be read from
	let spell_paths = vec!
	[
		"spells/players_handbook/prestidigitation.spell",
		"spells/players_handbook/mending.spell",
		"spells/players_handbook/mage_hand.spell",
		"spells/players_handbook/fire_bolt.spell",
		"spells/strixhaven/silvery_barbs.spell",
		"spells/players_handbook/color_spray.spell",
		"spells/players_handbook/magic_missile.spell",
		"spells/xanathars_guide_to_everything/ice_knife.spell",
		"spells/players_handbook/mage_armor.spell",
		"spells/players_handbook/unseen_servant.spell",
		"spells/players_handbook/detect_magic.spell",
		"spells/players_handbook/alarm.spell",
		"spells/players_handbook/cloud_of_daggers.spell",
		"spells/players_handbook/scorching_ray.spell"
	];
	// Attempt to loop through each spell file and convert it into a spell struct
	for path in spell_paths
	{
		println!("{}", path);
		// Convert spell file into spell struct and add it to spell_list vec
		spell_list.push(dnd_spellbook_maker::spells::Spell::from_file(path).unwrap());
	}
	// File paths to the fonts needed
	let font_paths = dnd_spellbook_maker::FontPaths
	{
		regular: String::from("fonts/Bookman/Bookman-Regular.otf"),
		bold: String::from("fonts/Bookman/Bookman-Bold.otf"),
		italic: String::from("fonts/Bookman/Bookman-Italic.otf"),
		bold_italic: String::from("fonts/Bookman/Bookman-BoldItalic.otf")
	};
	// Parameters for determining the size of the page and the text margins on the page
	let page_size_data = dnd_spellbook_maker::PageSizeData::new
	(
		210.0, 297.0, 10.0, 10.0, 10.0, 10.0
	).unwrap();
	// Parameters for determining page number behavior
	let page_number_data = dnd_spellbook_maker::PageNumberData::new
	(
		true, false, 1, 5.0, 4.0
	).unwrap();
	// Parameters for determining font sizes, the tab amount, and newline amounts
	let font_size_data = dnd_spellbook_maker::FontSizeData::new
	(
		32.0, 24.0, 12.0, 7.5, 12.0, 8.0, 5.0
	).unwrap();
	// Colors for each type of text
	let text_colors = dnd_spellbook_maker::TextColors
	{
		title_color: (0, 0, 0),
		header_color: (115, 26, 26),
		body_color: (0, 0, 0)
	};
	// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
	let font_scalars = dnd_spellbook_maker::FontScalars::new
	(
		0.475, 0.51, 0.48, 0.515
	).unwrap();
	// Parameters for table margins / padding and off-row color / scaling
	let table_options = dnd_spellbook_maker::TableOptions::new
	(
		16.0, 10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)
	).unwrap();
	// File path to the background image
	let background_path = "img/parchment.jpg";
	// Image transform data for the background image
	let background_transform = dnd_spellbook_maker::ImageTransform
	{
		translate_x: Some(dnd_spellbook_maker::Mm(0.0)),
		translate_y: Some(dnd_spellbook_maker::Mm(0.0)),
		scale_x: Some(1.95),
		scale_y: Some(2.125),
		..Default::default()
	};
	// Creates the spellbook
	let doc = dnd_spellbook_maker::generate_spellbook
	(
		spellbook_name, &spell_list, &font_paths, &page_size_data,
		&Some(page_number_data), &font_size_data, &text_colors, &font_scalars,
		&table_options, &Some((background_path, &background_transform))
	).unwrap();
	// Saves the spellbook to a pdf document
	let _ = dnd_spellbook_maker::save_spellbook(doc, "Spellbook.pdf");
}
```
 
# Setup
This library requires fonts files in order to work so it can add text to the document. Adding a background image to each of the pages is optional, but needs to be manually supplied if that is desired.

## Fonts
Four font files are required to be given to the generate_spellbook() function in one of the struct parameters. The required fonts files must be either **.otf** or **.ttf** files.
The four font files that are needed for a font are:

- Regular font
- Bold font
- Italic font
- Bolt Italic font

## Background Image
A background image can be added to every page of a spellbook, but it is not required. The image is added to each page of the spellbook via the printpdf crate which has bugs with adding images to pdf page layers.
If you encounter a bug where your image is not added to the page properly, or at all, try converting the image to a different type (**.jpg** to **.png** or vice versa, etc.).

# Creating a Spellbook
There are three main functions used to create spellbooks and several objects that are used as the parameters for those functions.

## Functions

### generate_spellbook()
```rust
pub fn generate_spellbook
(
	title: &str, spell_list: &Vec<spells::Spell>, font_paths: &FontPaths,
	page_size_data: &PageSizeData, page_number_options: &Option<PageNumberData>,
	font_size_data: &FontSizeData, text_colors: &TextColors, font_scalars: &FontScalars,
	table_options: &TableOptions, background_img_data: &Option<(&str, &ImageTransform)>
) -> Result<PdfDocumentReference, Box<dyn std::error::Error>>
```

This function is for creating spellbooks.

#### Parameters
- `title: &str` The name of the spellbook. It will determine what text appears on the cover page and what the pdf document will be named in the meta data.
- `spell_list: &Vec<spells::Spell>` The list of spells that the spellbook will contain. The spells do not have to be in any particular order.
- `font_paths: &FontPaths` Struct containing the file paths to the regular, bold, italic, and bold-italic fonts that the spellbook will use for the text.
- `page_size_data: &PageSizeData` Struct containing the data that determines the size of the page and the text margins (space between edge of page and text).
- `page_number_options: &Option<PageNumberData>` Option containing a struct of the page number behavior (starting number, positioning, flip sides or not, etc.). A value of `None` will make the spellbook have no page numbers.
- `font_size_data: &FontSizeData` Struct containing the font size for various types of text and spacing behavior like newline amounts and tabbing amounts.
- `text_colors: &TextColors` Struct containing the rgb values for each type of text in the spellbook.
- `font_scalars: &FontScalars` Numbers that determine how the size of each font is calculated. Numbers being slightly off may lead to text spilling off the page or going to new lines too early.
You may need to tinker with these values for the fonts you are using until the text in your spellbooks look good to get it right.
- `table_options: &TableOptions` Struct containing options that determine the appearance of tables.
- `background_img_data: &Option<(&str, &ImageTransform)>` Option containing the data needed to put a background image on every page in the spellbook.
The `&str` is the file path to the background image and the `&ImageTransform` is a struct containing options that determine the sizing and rotation of the image.

#### Output
Returns any errors that occur. Otherwise, it returns a struct containing the data of the spellbook that can be saved to a file if there were no errors.
The struct it returns on a success is a printpdf::PdfDocumentReference from the printpdf crate (https://docs.rs/printpdf/latest/printpdf/struct.PdfDocumentReference.html).

### save_spellbook()
```rust
pub fn save_spellbook(doc: PdfDocumentReference, file_name: &str)
-> Result<(), Box<dyn std::error::Error>>
```

This function is for saving spellbooks to a file as a pdf document.

#### Parameters
- `doc: PdfDocumentReference` The spellbook that gets returned from `generate_spellbook()`
- `file_name: &str` The name to give to the file that the spellbook will be saved to.

#### Output
Returns any errors that occur. Otherwise, it returns nothing if there were no errors.

### get_all_spells_in_folder()
```rust
pub fn get_all_spells_in_folder(folder_path: &str)
-> Result<Vec<spells::Spell>, Box<dyn std::error::Error>>
```

This function is for obtaining and entire folder of spells easily. It will assume every file in the folder is a spell.

#### Parameters
- `folder_path: &str` The file path to the folder to extract every spell from.

#### Output
Returns any errors that occur. Otherwise, it returns a vec of spell objects that can be inputted into `generate_spellbook()`.
