# dnd_spellbook_maker
Library for making pdf documents of spells that ressemble 5th edition D&D official source book spell descriptions.

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
Details on the structure of spell files are below if you wish to create your own. This library comes with a spell file for every spell in the following source material books:
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

Examples:

`name: Acid Splash`

`Name: Fireball`

`NAME: Weird`

### Level
`level:`

The level of the spell. Ranges from the integers 0 to 9. Level 0 represents cantrips.

Examples:

`level: 3`

`Level: 9`

`LEVEL: 0`

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

Examples:

`school: Evocation`

`School: illusion`

`SCHOOL: nEcRoMaNcY`

### Ritual
`is_ritual:`

Whether or not the spell is a ritual. Must be either `true` or `false`.
Alternatively, this field can just be placed in the spell file without any colons or value to represent `true`, or not be placed in the spell file at all to represent `false`.

Examples:

`is_ritual: true` or `is_ritual`

`is_ritual: false` or just not having the field in the file.

### Casting Time
`casting_time:`

The amount of time it takes to cast this spell. Can one of the following values:
- `seconds` Must be followed by a nonnegative integer.
- `actions` Must be followed by a nonnegative integer.
- `bonusaction`
- `reaction` Must be followed by text on one line inside of quotation marks.
- `minutes` Must be followed by a nonnegative integer.
- `hours` Must be followed by a nonnegative integer.
- `days` Must be followed by a nonnegative integer.
- `weeks` Must be followed by a nonnegative integer.
- `months` Must be followed by a nonnegative integer.
- `years` Must be followed by a nonnegative integer.
- `special`

Examples:

`casting_time: actions 1`

`Casting_Time: bonusaction`

`CASTING_TIME: Minutes 10`

`cAsTiNg_TiMe: REACTION "which you take when you see a creature within 60 feet succeed an attack roll"`

### Range
`range:`

The distance / area that this spell can target things within. Can be one of the following values:
- `self` This value can optionally be followed by an AOE (area of effect) value (details on AOEs below).
- `touch`
- A distance value (details on distances below).
- `sight`
- `unlimited`
- `special`

AOE (area of effect) values define a volumetric shape that a spell's effect takes place in. They can be one of the following values:
- `line` Must be followed by a distance value (details on distances below).
- `cone` Must be followed by a distance value.
- `cube` Must be followed by a distance value.
- `sphere` Must be followed by a distance value.
- `hemisphere` Must be followed by a distance value.
- `cylinder` Must be followed by two distance values, the first representing its radius, the second representing its height.

Distance values are self explanitory. They are defined by a positive integer and a string representing the unit of measurement of that distance. Valid distance values are:
- `feet` Must be followed by a nonnegative integer.
- `miles` Must be followed by a nonnegative integer.

Examples:

`range: feet 120`

`RANGE: Self`

`RaNgE: self cone feet 15`

`Range: self cylinder feet 15 feet 60`

`ranGe: special`

### V / S Components
`has_v_component:` / `has_s_component:`

These two fields determine whether or not the spell has verbal / somantic components. Must be either `true` or `false`.
Alternatively, these fields can just be placed in the spell file without any colons or value to represent `true`, or not be placed in the spell file at all to represent `false`.

Examples:

`has_v_component: true` or `has_v_component`

`has_v_component: false` or just not having the field in the file.

`has_s_component: true` or `has_s_component`

`has_s_component: false` or just not having the field in the file.

### M Components
`m_components:`

The material components for the spell. If the spell has material components, its value should be text on one line inside of quotation marks. 
If the spell does not have any material components, its value should be `none` or you can not include the field in the spell file for the same effect.

Examples:

`m_components: "A piece of string and a bit of wood"`

`m_components: none` or just not having the field in the file

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

Examples:

`duration: rounds 1`

`DURATION: minutes 10 concentration`

`dUrAtIoN: untildispelledortriggered`

`Duration: untildispelled Concentration`

`duration: permanent`

### Description
`description:`

The text that describes what the spell does. This field's value must be text inside of quotes that can span multiple lines.
If a line ends in a quote before you want the description text to end, you can place an escape backslash before the quote (like this: `\"`) to prevent the quote from ending the text early.
If you place another escape backslash before the first one (like this: `\\"`) then a backslash will appear in the text as the last character and the quote will end the line normally, and so on with more backslashes.

Spell descriptions can also have font changes, bullet points, and tables inside of them, just like in the Player's Handbook.

Text starts out using the regular font that is supplied to the `generate_spellbook()` function, but can be switched back and forth between any of the four supplied with these font tags:
- `<r>` for regular font
- `<b>` for bold font
- `<i>` for italic font
- `<bi>` or `<ib>` for bold-italic font

Once any of these tokens are detected, the following text will use that tag's font. The tags must be individual tokens in order to be detected (surrounded by whitespace).

Examples:

`description: "This text is regular font <bi> and this text is bold italic font <b> and this font is bold <r> and now it's back to regular font."`

`Description: "<i> This text starts out italic <r> and then it changes to regular."`

To do bullet points, just begin a line / paragraph inside of the text with either a dash `-` or a bullet point character `•`.

Examples:

```
description: "This is normal text on the first paragraph.
This is new text on a new second paragraph.
This is the third paragraph.
- This is the first bullet point in a series of them.
- This is the second bullet point.
- This is the third bullet point.
And now it's back to regular non-bullet-point text in the last paragraph."
```

```
DeScRiPtIoN: "This is some normal text in a paragraph.
• <bi> This is some bold-italic text in a bullet point.
• This is some more bold- italic text in another bullet point.
• <r> This text is back to regular font in a third bullet point.
This text is back to normal paragraph form."
```

To put tables in the description, you need to use several tags. First is the `<table>` tag which should be at the start and end of every table.
Second is the `<title>` tag which defines a title for the table.
This tag is optional, but if it is used it must appear on the same line as the opening `<table>` tag and must surround the title text of the table.
Next is the column delimiter `|` which separates individual cells inside of a row.
Lastly is the `<row>` tag which marks where new rows begin. This tag isn't required for the first row, aka the header row / column header row.

Examples:

```
description: "This is some text in normal paragraph form.
<table> <title> This is a table <title>
Column A | Column B
<row> 1 | Red
<row> 2 | Green
<row> 3 | Blue
<table>
And here's some more text because why not."
```

```
description: "Here is a table with 3 columns but no title.
<table>
1 | 2 | 3
<row> A | Black | Black
<row> B | Black | White
<row> C | White | Black
<row> D | White | White
<table>"
```

### Upcast Description
`upcast_description:`

The text that describes what the spell does when you cast it at a higher level.
Follows the same rules as the `description:` field, except if the spell doesn't have an upcast description, it can either have a value of `none` or just not be included in the spell file to have the same effect.

Examples:

```
upcast_description: "This is an upcast description <b> with some font changes <r>.
- Here are some bullet points too
- A second bullet point
Some more text.
<table> <title> Here's a table too <title>
A | B | C
<row> 1 | 2 | 3
<row> 4 | 5 | 6
<table>"
```

`upcast_description: none` or just not having the field in the file.

## Custom Fields

The fields `level:`, `school:`, `casting_time:`, `range:`, and `duration:` all have controlled values with only a few select valid types of values.
This control measure can be overridden for all of these fields, allowing users to inject whatever text they want into those fields on that spell's page(s) in the spellbook.
This can be done by making the value of each of these fields be text on one line inside of quotation marks, the same as nonempty `m_components:` values.

Examples:

`level: "10th-level"`

`school: "technomancy"`

`casting_time: "1 action or 8 hours"`
