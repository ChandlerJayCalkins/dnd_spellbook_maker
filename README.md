# dnd-spellbook-maker-lib
Library for making pdf documents of spells that a D&D character has.

## Quickstart

### Cargo.toml Dependency

```toml
dnd-spellbook-maker-lib = "0.1.0"
```

or

```toml
dnd-spellbook-maker-lib = { git = "https://github.com/ChandlerJayCalkins/dnd-spellbook-maker-lib" }
```

### Example Program

```Rust
use dnd-spellbook-maker-lib;

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
		spell_list.push(dnd-spellbook-maker-lib::spells::Spell::from_file(path).unwrap());
	}
	// File paths to the fonts needed
	let font_paths = FontPaths
	{
		regular: String::from("fonts/Bookman/Bookman-Regular.otf"),
		bold: String::from("fonts/Bookman/Bookman-Bold.otf"),
		italic: String::from("fonts/Bookman/Bookman-Italic.otf"),
		bold_italic: String::from("fonts/Bookman/Bookman-BoldItalic.otf")
	};
	// Parameters for determining the size of the page and the text margins on the page
	let page_size_data = dnd-spellbook-maker-lib::PageSizeData::new
	(
		210.0, 297.0, 10.0, 10.0, 10.0, 10.0
	).unwrap();
	// Parameters for determining page number behavior
	let page_number_data = dnd-spellbook-maker-lib::PageNumberData::new
	(
		true, false, 1, 5.0, 4.0
	).unwrap();
	// Parameters for determining font sizes, the tab amount, and newline amounts
	let font_size_data = dnd-spellbook-maker-lib::FontSizeData::new
	(
		32.0, 24.0, 12.0, 7.5, 12.0, 8.0, 5.0
	).unwrap();
	// Colors for each type of text
	let text_colors = dnd-spellbook-maker-lib::TextColors
	{
		title_color: (0, 0, 0),
		header_color: (115, 26, 26),
		body_color: (0, 0, 0)
	};
	// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
	let font_scalars = dnd-spellbook-maker-lib::FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
	// Parameters for table margins / padding and off-row color / scaling
	let table_options = dnd-spellbook-maker-lib::TableOptions::new
	(
		16.0, 10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)
	).unwrap();
	// File path to the background image
	let background_path = "img/parchment.jpg";
	// Image transform data for the background image
	let background_transform = dnd-spellbook-maker-lib::ImageTransform
	{
		translate_x: Some(Mm(0.0)),
		translate_y: Some(Mm(0.0)),
		scale_x: Some(1.95),
		scale_y: Some(2.125),
		..Default::default()
	};
	// Creates the spellbook
	let doc = dnd-spellbook-maker-lib::generate_spellbook
	(
		spellbook_name, &spell_list, &font_paths, &page_size_data, &Some(page_number_data),
		&font_size_data, &text_colors, &font_scalars, &table_options,
		&Some((background_path, &background_transform))
	).unwrap();
	// Saves the spellbook to a pdf document
	let _ = dnd-spellbook-maker-lib::save_spellbook(doc, "Spellbook.pdf");
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
