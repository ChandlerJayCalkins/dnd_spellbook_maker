# dnd_spellbook_maker
Library for making pdf documents of spells that a 5th edition D&D character has.

Documentation at <https://docs.rs/dnd_spellbook_maker>.

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

This program makes a spellbook. It assumes the directory it is in has the same "spells", "fonts", and "img" folders from this repository with all their contents.

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
This library requires font files in order to work so it can add text to the document. Adding a background image to each of the pages is optional, but needs to be manually supplied if that is desired.

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

# Spells
To create a spellbook using the `generate_spellbook()` function, it needs a vec of `Spell` objects. Spell objects should generally be created from spell files.
Details on the structure of spell files is below if you wish to create your own. This library comes with a spell file for every spell in the following source material books:
- The Player's Handbook
- Xanathar's Guide to Everything
- Tasha's Cauldron of Everything
- Strixhaven: A curriculum of Chaos.

# Spell Files
Spell files are plaintext files with fields separated mostly by newlines. Each field in a spell file corresponds to one of the fields in the `Spell` struct. The fields are as follows:
- `name:`
- `level:`
- `school:`
- `is_ritual:`
- `casting_time:`
- `range:`
- `has_v_component:`
- `has_s_component:`
- `m_components:`
- `duration:`
- `description:`
- `upcast_description:`

## Fields

### Name
`name:`

The name of the spell. Can be any string on a single line (doesn't need to be surrounded by quotes).

### Level
`level:`

The level of the spell. Ranges from the integers 0 to 9.

### School
`school:`

The magic school of the spell. Can be one of the following values:
- `abjuration`
- `conjuration`
- `divination`
- `enchantment`
- `evocation`
- `illusion`
- `necromancy`
- `transmutation`

### Ritual
`is_ritual:`

Whether or not the spell is a ritual. Must be either `true` or `false`.

### Casting Time
`casting_time:`

The amount of time it takes to cast this spell. Can one of the following values:
- `seconds` Must be followed by a nonnegative integer.
- `actions` Must be followed by a nonnegative integer.
- `bonusaction`
- `reaction` Must be followed by text inside of quotes that does not go to a new line.
- `minutes` Must be followed by a nonnegative integer.
- `hours` Must be followed by a nonnegative integer.
- `days` Must be followed by a nonnegative integer.
- `weeks` Must be followed by a nonnegative integer.
- `months` Must be followed by a nonnegative integer.
- `years` Must be followed by a nonnegative integer.
- `special`

### Range
`range:`

The distance / area that this spell can target things within. Can be one of the following values:
- `self` This value can optionally be followed by an AOE (area of effect) value. Valid AOE values:
	- `line` Must be followed by a distance value (details on distances below).
	- `cone` Must be followed by a distance value.
	- `cube` Must be followed by a distance value.
	- `sphere` Must be followed by a distance value.
	- `hemisphere` Must be followed by a distance value.
	- `cylinder` Must be followed by two distance values.
- `touch`
- A distance value (details on distances below).
- `sight`
- `unlimited`
- `special`

### V / S Components
`has_v_component:` / `has_s_component:`

Whether or not the spell has a verbal / somantic component. Must be either `true` or `false`.

### M Components
`m_components:`

The material components for the spell. If the spell has material components, its value should be text inside of quotes that does not go to a new line. 
If the spell does not have any material components, its value should be `none`.

### Duration
`duration:`
The length of time that the spell lasts. Can be one of the following values:
- `instant`
- `seconds` Must be followed by a nonnegative integer. The integer can also be followed by the value `concentration` if this spell requires concentration.
- `rounds` Must be followed by a nonnegative integer. The integer can also be followed by the value `concentration` if this spell requires concentration.
- `minutes` Must be followed by a nonnegative integer. The integer can also be followed by the value `concentration` if this spell requires concentration.
- `hours` Must be followed by a nonnegative integer. The integer can also be followed by the value `concentration` if this spell requires concentration.
- `days` Must be followed by a nonnegative integer. The integer can also be followed by the value `concentration` if this spell requires concentration.
- `weeks` Must be followed by a nonnegative integer. The integer can also be followed by the value `concentration` if this spell requires concentration.
- `months` Must be followed by a nonnegative integer. The integer can also be followed by the value `concentration` if this spell requires concentration.
- `years` Must be followed by a nonnegative integer. The integer can also be followed by the value `concentration` if this spell requires concentration.
- `dispelledortriggered` Can be followed by the value `concentration` if the spell requires concentration.
- `untildispelled` Can be followed by the value `concentration` if the spell requires concentration.
- `permanent`
- `special` Can be followed by the value `concentration` if the spell requires concentration.

### Description
`description:`

The text that describes what the spell does. This field's value must be text inside of quotes that can span multiple lines.

### Upcast Description
`upcast_description:`

The text that describes what the spell does when you cast it at a higher level. Follows the same rules as the `description:` field.
