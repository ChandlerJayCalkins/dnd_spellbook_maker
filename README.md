# dnd_spellbook_maker
---

Library for making pdf documents of spells that ressemble 5th edition D&D official source book spell descriptions.

Documentation at <https://docs.rs/dnd_spellbook_maker>.

# Quickstart
---

## Cargo.toml Dependency

```toml
dnd_spellbook_maker = "0.2.0"
```

or

```toml
dnd_spellbook_maker = { git = "https://github.com/ChandlerJayCalkins/dnd_spellbook_maker" }
```

## Example Program

This program makes a spellbook. It assumes the directory it is in has the same "spells", "fonts", and "img" folders from this repository with all their contents.

```Rust
use dnd_spellbook_maker;

fn main()
{
	// Spellbook's name
	let spellbook_name = "A Spellcaster's Spellbook";
	// Vec of paths to spell json files that will be read from
	let spell_paths = vec!
	[
		"spells/players_handbook_2024/prestidigitation.json",
		"spells/players_handbook_2024/mending.json",
		"spells/players_handbook_2024/mage_hand.json",
		"spells/players_handbook_2024/fire_bolt.json",
		"spells/strixhaven/silvery_barbs.json",
		"spells/players_handbook_2024/color_spray.json",
		"spells/players_handbook_2024/magic_missile.json",
		"spells/xanathars_guide_to_everything/ice_knife.json",
		"spells/players_handbook_2024/mage_armor.json",
		"spells/players_handbook_2024/unseen_servant.json",
		"spells/players_handbook_2024/detect_magic.json",
		"spells/players_handbook_2024/alarm.json",
		"spells/players_handbook_2024/cloud_of_daggers.json",
		"spells/players_handbook_2024/scorching_ray.json"
	];
	// Vec of spells that will be added to spellbook
	let mut spell_list = Vec::with_capacity(spell_paths.len());
	// Attempt to loop through each spell file and convert it into a spell struct
	for path in spell_paths
	{
		println!("{}", path);
		// Convert spell file into spell struct and add it to spell_list vec
		spell_list.push(dnd_spellbook_maker::spells::Spell::from_json_file(path).unwrap());
	}
	// File paths to the fonts needed
	let font_paths = FontPaths
	{
		regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
		bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
		italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
		bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
	};
	// Parameters for determining font sizes
	let font_sizes = FontSizes::new(32.0, 24.0, 12.0, 16.0, 12.0)
		.expect("Failed to create font sizes.");
	// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
	let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515)
		.expect("Failed to create font scalars.");
	// Parameters for determining tab and newline sizes
	let spacing_options = SpacingOptions::new(7.5, 12.0, 8.0, 5.0, 6.4, 5.0)
		.expect("Failed to create spacing options.");
	// Colors for each type of text
	let text_colors = TextColorOptions
	{
		title_color: (0, 0, 0),
		header_color: (115, 26, 26),
		body_color: (0, 0, 0),
		table_title_color: (0, 0, 0),
		table_body_color: (0, 0, 0)
	};
	// Parameters for determining the size of the page and the text margins on the page
	let page_size_options = PageSizeOptions::new(210.0, 297.0, 10.0, 10.0, 6.0, 10.0)
		.expect("Failed to create page size options.");
	// Parameters for determining page number behavior
	let page_number_options = PageNumberOptions::new
	(HSide::Left, false, 1, FontVariant::Regular, 12.0, 5.0, (0, 0, 0), 5.0, 4.0)
		.expect("Failed to create page number options.");
	// File path to the background image
	let background_path = String::from("img/parchment.jpg");
	// Image transform data for the background image
	let background_transform = ImageTransform
	{
		translate_x: Some(Mm(0.0)),
		translate_y: Some(Mm(0.0)),
		scale_x: Some(1.95),
		scale_y: Some(2.125),
		..Default::default()
	};
	// Parameters for table margins / padding and off-row color / scaling
	let table_options = TableOptions::new(10.0, 8.0, 4.0, 12.0, 0.12, 4.4, (213, 209, 224))
		.expect("Failed to create table options.");
	// Creates the spellbook
	let (doc, _, _) = create_spellbook
	(
		spellbook_name_1,
		&spell_list,
		font_paths.clone(),
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((&background_path, background_transform)),
		table_options
	).unwrap();
	// Saves the spellbook to a file
	let _ = save_spellbook(doc, "Spellbook.pdf").unwrap();
}
```

It is recommended to use the font files in the (fonts)[fonts] folder in this repository along with the font scalar values in this example. Different font scalar values will be needed if use different font files are used.

See documentation to better understand this code.

# Setup
---

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
---

To create a spellbook using the `generate_spellbook()` function, it needs a vec of `Spell` objects. Spell objects should generally be created from spell json files. Details on the structure of spell json files are below if you wish to create your own. This library comes with a spell file for every spell in the following source material books:
- The Player's Handbook (2014)
- The Player's Handbook (2024)
- Xanathar's Guide to Everything
- Tasha's Cauldron of Everything
- Strixhaven: A curriculum of Chaos.

# Spell JSON Files
---

Here are some example spell json files. See the [Spell JSON Formatting Documentation](spell_json_formatting.md) for more info on each field.

```json
{
	"name": "Chaos Bolt",
	"level":
	{
		"Controlled": "Level1"
	},
	"school":
	{
		"Controlled": "Evocation"
	},
	"is_ritual": false,
	"casting_time":
	{
		"Controlled":
		{
			"Actions": 1
		}
	},
	"range":
	{
		"Controlled":
		{
			"Dist":
			{
				"Feet": 120
			}
		}
	},
	"has_v_component": true,
	"has_s_component": true,
	"m_components": null,
	"duration":
	{
		"Controlled": "Instant"
	},
	"description": "You hurl an undulating, warbling mass of chaotic energy at one creature in range. Make a ranged spell attack against the target. On a hit, the target takes 2d8 + 1d6 damage. Choose one of the d8s. The number rolled on that die determines the attack's damage type, as shown below.\n[table][0]\nIf you roll the same number on both d8s, the chaotic energy leaps from the target to a different creature of your choice within 30 feet of it. Make a new attack roll against the new target, and make a new damage roll, which could cause the chaotic energy to leap again.\nA creature can be targeted only once by each casting of this spell.",
	"upcast_description": "When you cast this spell using a spell slot of 2nd level or higher, each target takes 1d6 extra damage of the type rolled for each slot level above 1st.",
	"tables":
	[
		{
			"title": "",
			"column_labels":
			[
				"d8",
				"Damage Type"
			],
			"cells":
			[
				[
					"1",
					"Acid"
				],
				[
					"2",
					"Cold"
				],
				[
					"3",
					"Fire"
				],
				[
					"4",
					"Force"
				],
				[
					"5",
					"Lightning"
				],
				[
					"6",
					"Poison"
				],
				[
					"7",
					"Psychic"
				],
				[
					"8",
					"Thunder"
				]
			]
		}
	]
}
```

```json
{
	"name": "Acid Splash",
	"level":
	{
		"Controlled": "Cantrip"
	},
	"school":
	{
		"Controlled": "Conjuration"
	},
	"is_ritual": false,
	"casting_time":
	{
		"Controlled":
		{
			"Actions": 1
		}
	},
	"range":
	{
		"Controlled":
		{
			"Dist":
			{
				"Feet": 60
			}
		}
	},
	"has_v_component": true,
	"has_s_component": true,
	"m_components": null,
	"duration":
	{
		"Controlled": "Instant"
	},
	"description": "You hurl a bubble of acid. Choose one creature within range, or choose two creatures within range that are within 5 feet of each other. A target must succeed on a Dexterity saving throw or take 1d6 acid damage.\nThis spell's damage increases by 1d6 when you reach 5th level (2d6), 11th level (3d6), and 17th level (4d6).",
	"upcast_description": null,
	"tables": []
}
```

```json
{
	"name": "Antimagic Field",
	"level":
	{
		"Controlled": "Level8"
	},
		"school":
		{
			"Controlled": "Abjuration"
		},
	"is_ritual": false,
	"casting_time":
	{
		"Controlled":
		{
			"Actions": 1
		}
	},
	"range":
	{
		"Controlled":
		{
			"Yourself":
			{
				"Sphere":
				{
					"Feet": 10
				}
			}
		}
	},
	"has_v_component": true,
	"has_s_component": true,
	"m_components": "a pinch of powdered iron or iron filings",
	"duration":
	{
		"Controlled":
		{
			"Hours":
			[
				1,
				true
			]
		}
	},
	"description": "A 10-foot-radius invisible sphere of antimagic surrounds you. This area is divorced from the magical energy that suffuses the multiverse. Within the sphere, spells can't be cast, summoned creatures disappear, and even magic items become mundane. Until the spell ends, the sphere moves with you, centered on you.\nSpells and other magical effects, except those created by an artifact or a deity, are suppressed in the sphere and can't protrude into it. A slot expended to cast a suppressed spell is consumed. While an effect is suppressed, it doesn't function, but the time it spends suppressed counts against its duration.\n<bi> Targeted Effects. <r> Spells and other magical effects, such as magic missile and charm person, that target a creature or an object in the sphere have no effect on that target.\n<bi> Areas of Magic. <r> The area of another spell or magical effect, such as fireball, can't extend into the sphere. If the sphere overlaps an area of magic, the part of the area that is covered by the sphere is suppressed. For example, the flames created by a wall of fire are suppressed within the sphere, creating a gap in the wall if the overlap is large enough.\n<bi> Spells. <r> Any active spell or other magical effect on a creature or an object in the sphere is suppressed while the creature or object is in it.\n<bi> Magic Items. <r> The properties and powers of magic items are suppressed in the sphere. For example, a +1 longsword in the sphere functions as a nonmagical longsword.\nA magic weapon's properties and powers are suppressed if it is used against a target in the sphere or wielded by an attacker in the sphere. If a magic weapon or a piece of magic ammunition fully leaves the sphere (for example, if you fire a magic arrow or throw a magic spear at a target outside the sphere), the magic of the item ceases to be suppressed as soon as it exits.\n<bi> Magical Travel. <r> Teleportation and planar travel fail to work in the sphere, whether the sphere is the destination or the departure point for such magical travel. A portal to another location, world, or plane of existence, as well as an opening to an extradimensional space such as that created by the rope trick spell, temporarily closes while in the sphere.\n<bi> Creatures and Objects. <r> A creature or object summoned or created by magic temporarily winks out of existence in the sphere. Such a creature instantly reappears once the space the creature occupied is no longer within the sphere.\n<bi> Dispel Magic. <r> Spells and magical effects such as dispel magic have no effect on the sphere. Likewise, the spheres created by different antimagic field spells don't nullify each other.",
	"upcast_description": null,
	"tables": []
}
```

```json
{
	"name": "Control Weather",
	"level":
	{
		"Controlled": "Level8"
	},
	"school":
	{
		"Controlled": "Transmutation"
	},
	"is_ritual": false,
	"casting_time":
	{
		"Controlled":
		{
			"Minutes": 10
		}
	},
	"range":
	{
		"Controlled":
		{
			"Yourself":
			{
				"Sphere":
				{
					"Miles": 5
				}
			}
		}
	},
	"has_v_component": true,
	"has_s_component": true,
	"m_components": "burning incense and bits of earth and wood mixed in water",
	"duration":
	{
		"Controlled":
		{
			"Hours":
			[
				8,
				true
			]
		}
	},
	"description": "You take control of the weather within 5 miles of you for the duration. You must be outdoors to cast this spell. Moving to a place where you don't have a clear path to the sky ends the spell early.\nWhen you cast the spell, you change the current weather conditions, which are determined by the DM based on the climate and season. You can change precipitation, temperature, and wind. It takes 1d4 × 10 minutes for the new conditions to take effect. Once they do so, you can change the conditions again. When the spell ends, the weather gradually returns to normal. When you change the weather conditions, find a current condition on the following tables and change its stage by one, up or down. When changing the wind, you can change its direction.\n[table][0]\n[table][1]\n[table][2]",
	"upcast_description": null,
	"tables":
	[
		{
			"title": "Precipitation",
			"column_labels":
			[
				"Stage",
				"Condition"
			],
			"cells":
			[
				[
					"1",
					"Clear"
				],
				[
					"2",
					"Light Clouds"
				],
				[
					"3",
					"Overcast or ground fog"
				],
				[
					"4",
					"Rain, hail, or snow"
				],
				[
					"5",
					"Torrential rain, driving hail, or blizzard"
				]
			]
		},
		{
			"title": "Temperature",
			"column_labels":
			[
				"Stage",
				"Condition"
			],
			"cells":
			[
				[
					"1",
					"Unbearable heat"
				],
				[
					"2",
					"Hot"
				],
				[
					"3",
					"Warm"
				],
				[
					"4",
					"Cool"
				],
				[
					"5",
					"Cold"
				],
				[
					"6",
					"Arctic cold"
				]
			]
		},
		{
			"title": "Wind",
			"column_labels":
			[
				"Stage",
				"Condition"
			],
			"cells":
			[
				[
					"1",
					"Calm"
				],
				[
					"2",
					"Moderate"
				],
				[
					"3",
					"Strong wind"
				],
				[
					"4",
					"Gale"
				],
				[
					"5",
					"Storm"
				]
			]
		}
	]
}
```
