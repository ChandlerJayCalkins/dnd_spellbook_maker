# Spell Struct Definition
---

```rs
pub struct Spell
{
	pub name: String,
	pub level: SpellField<Level>,
	pub school: SpellField<MagicSchool>,
	pub is_ritual: bool,
	pub casting_time: SpellField<CastingTime>,
	pub range: SpellField<Range>,
	pub has_v_component: bool,
	pub has_s_component: bool,
	pub m_components: Option<String>,
	pub duration: SpellField<Duration>,
	pub description: String,
	pub upcast_description: Option<String>,
	pub tables: Vec<Table>
}
```

# `name` Field
---

Any string.

```json
"name": "Spell Name"
```

# `level` Field
---

Either `Custom` value with any string

```json
"level":
{
    "Custom": "Custom Level"
}
```

or `Controlled` value with a `Level`.

```json
"level":
{
    "Controlled": "Cantrip"
}
```

```json
"level":
{
    "Controlled": "Level4"
}
```

Here is the definition of all possible `Level` variants:

```rs
pub enum Level
{
	Cantrip,
	Level1,
	Level2,
	Level3,
	Level4,
	Level5,
	Level6,
	Level7,
	Level8,
	Level9
}
```

# `school` Field
---

Either `Custom` value with any string

```json
"school":
{
    "Custom": "Custom Magic School"
}
```

or `Controlled` value with a `MagicSchool`

```json
"school":
{
	"Constrolled": "Conjuration"
}
```

```json
"school":
{
	"Controlled": "Illusion"
}
```

Here is the definition of all possible 'MagicSchool` variants:

```rs
pub enum MagicSchool
{
	Abjuration,
	Conjuration,
	Divination,
	Enchantment,
	Evocation,
	Illusion,
	Necromancy,
	Transmutation
}
```

# `is_ritual` Field
---

Boolean value.

```json
"is_ritual": false
```

```json
"is_ritual": true
```

# `casting_time` Field
---

Either `Custom` value with any string

```json
"casting_time":
{
    "Custom": "Custom Casting Time"
}
```

or `Controlled` value with a `CastingTime`.

```json
"casting_time":
{
	"Constrolled":
	{
		"Actions": 1
	}
}
```

```json
"casting_time":
{
	"Constrolled":
	{
		"Actions": 5
	}
}
```

```json
"casting_time":
{
	"Controlled":
	{
		"Minutes": 10
	}
}
```

```json
"casting_time":
{
	"Controlled":
	{
		"Hours": 4
	}
}
```

```json
"casting_time":
{
	"Controlled":
	{
		"BonusAction": null
	}
}
```

```json
"casting_time":
{
	"Controlled":
	{
		"Reaction": "which you take when you sense something that triggers a reaction"
	}
}
```

```json
"casting_time":
{
	"Controlled": "Special"
}
```

Here is the definition of all possible 'CastingTime` variants:

```rs
pub enum CastingTime
{
	Seconds(u16),
	Actions(u16),
	BonusAction,
	Reaction(String),
	Minutes(u16),
	Hours(u16),
	Days(u16),
	Weeks(u16),
	Months(u16),
	Years(u16),
	Special
}
```

`BonusAction` and `Reaction` variants can have either a `None` / `null` value

```json
"casting_time":
{
	"Controlled":
	{
		"BonusAction": null
	}
}
```

```json
"casting_time":
{
	"Controlled":
	{
		"Reaction": null
	}
}
```

or a `Some` value with a string.

```json
"casting_time":
{
	"Controlled":
	{
		"BonusAction": "which you take when you make a melee attack"
	}
}
```

```json
"casting_time":
{
	"Controlled":
	{
		"Reaction": "which you take when a creature makes a ranged attack against you"
	}
}
```

`BonusAction` variant will appear as either "Bonus action" in the spellbook if it has a null value, or as something like "Bonus action, which you take when you make a melee attack" in the spellbook if it has a string value.

`Reaction` variant will appear as either "Reaction" in the spellbook if it has a null value, or as something like "Reaction, which you take when a creature makes a ranged attack against you" in the spellbook if it has a string value.

# `range` Field
---

Either `Custom` value with any string

```json
"range":
{
    "Custom": "Custom Range"
}
```

or `Controlled` value with a `Range`.

```json
"range":
{
    "Controlled":
	{
		"Dist":
		{
			"Feet": 60
		}
	}
}
```

```json
"range":
{
    "Controlled":
	{
		"Dist":
		{
			"Miles": 2
		}
	}
}
```

```json
"range":
{
    "Controlled":
	{
		"Yourself": null
	}
}
```

```json
"range":
{
    "Controlled": "Touch"
}
```

Here is the definition of all possible `Range` variants:

```rs
pub enum Range
{
	Yourself(Option<AOE>),
	Touch,
	Dist(Distance),
	Sight,
	Unlimited,
	Special
}
```

`Yourself` variant can have either a `None` / `null` value

```json
"range":
{
    "Controlled":
	{
		"Yourself": null
	}
}
```

or a `Some` value with an AOE

```json
"range":
{
    "Controlled":
	{
		"Yourself":
		{
			"Emanation":
			{
				"Feet": 10
			}
		}
	}
}
```

```json
"range":
{
    "Controlled":
	{
		"Yourself":
		{
			"Cylinder":
			[
				{
					"Feet": 50
				},
				{
					"Miles": 1
				}
			]
		}
	}
}
```

`Yourself` variant will appear as either "Self" in the spellbook if it has a null value, or as something like "Self (10-foot emanation)", "Self (50-foot radius, 1-mile height cylinder)", etc. in the spellbook if it has an AOE value.

Here is the defintion of all possible `Aoe` variants:

```rs
pub enum AOE
{
	Line(Distance),
	Cone(Distance),
	Cube(Distance),
	Sphere(Distance),
	Emanation(Distance),
	Hemisphere(Distance),
	Cylinder(Distance, Distance),
}
```

(Note: the `Sphere` and `Emanation` variants are functionally the same in the game rules. Both are included here however to support both modern and legacy formatting for this spell field.)

`Dist` variant will appear as "30 feet", "1 mile", etc. based on its `Distance` value.

Here is the definition of all possible `Distance` variants:

```rs
pub enum Distance
{
	Feet(u16),
	Miles(u16)
}
```

# `has_v_component` Field
---

Boolean value.

```json
"has_v_component": false
```

```json
"has_v_component": true
```

# `has_s_component` Field
---

Boolean value.

```json
"has_s_component": false
```

```json
"has_s_component": true
```

# `m_components` Field
---

Either `None` value

```json
"m_components": null
```

or `Some` value with any string.

```json
"m_components": "Material component"
```

`Some` variant with a string will have the text "M \(" and "\)" put around the text in the string when written to the spell book if it has a `Some` value. For example, the `Some` example above will become "M \(Material component\)" in the spell book.

# `duration` Field
---

Either `Custom` value with any string

```json
"duration":
{
    "Custom": "Custom Duration"
}
```

or `Controlled` value with a `Duration`.

```json
"duration":
{
    "Controlled": "Instant"
}
```

```json
"duration":
{
    "Controlled":
	{
		"Seconds": [30, false]
	}
}
```

```json
"duration":
{
    "Controlled":
	{
		"Minutes": [1, true]
	}
}
```

```json
"duration":
{
    "Controlled":
	{
		"DispelledOrTriggered": false
	}
}
```

Here is the definition of all possible `Duration` variants:

```rs
pub enum Duration
{
	Instant,
	Seconds(u16, bool),
	Rounds(u16, bool),
	Minutes(u16, bool),
	Hours(u16, bool),
	Days(u16, bool),
	Weeks(u16, bool),
	Months(u16, bool),
	Years(u16, bool),
	DispelledOrTriggered(bool),
	UntilDispelled(bool),
	Permanent,
	Special(bool)
}
```

All `u16` values in this enum are the numeric values for the unit of time that their spell can last (the 1 in "1 action" or the 5 in "5 minutes", etc.). All `bool` values in this enum determine whether the duration is dependent on concentration or not. A bool value of `true` will have the text "Concentration, up to " put before the rest of the duration text in the spell book. For example, the `Minutes` example from above will become "Concentration, up to 1 minute" in the spell book while the `Seconds` example from above will become just "30 seconds" in the spell book.

# `description` Field
---

Any string.

```json
"description": "Spell description text."
```

There are several special tokens you can put in the spell description text to alter the content / appearance of the text. These tokens allow you to move to a new paragraph, change the font variant of text (regular, bold, italic, bold-italic), put text in bullet points, and place tables from the `tables` field into the text.

All non-newline whitespace text will be condensed down into a single space character when written to the spell book.

## Moving to a New Paragraph

Moving to a new paragraph only requires a newline character.

This description

```json
"description": "This text will appear in the first paragraph.\\nThis text will appear in the second paragraph. It can go on for a while in order to showcase line wrapping of paragraph text in this example. Here are some fun facts about D&D from The Fact Site by Jennifer Anyabuine (January 9, 2024) (https://www.thefactsite.com/dungeons-and-dragons-facts/). As of 2024, the longest running D&D campaign has been going on since 1982. The largest Dunegons & Dragons game featured 1,227 players. It only cost $2,000 to make the first Dungeons & Dragons game set. Gary Gygax's children helped him perfect Dungeons & Dragons and chose its name. Marvel made a Dungeons & Dragons cartoon.\\nThis is the text of the third paragraph."
```

will look similar to this in the spell book:

--

This text will appear in the first paragraph.

&nbsp;&nbsp;&nbsp;&nbsp;This text will appear in the second paragraph. It can go on for a while in order to showcase line wrapping of paragraph text in this example. Here are some fun facts about D&D from The Fact Site by Jennifer Anyabuine (January 9, 2024) (https://www.thefactsite.com/dungeons-and-dragons-facts/). As of 2024, the longest running D&D campaign has been going on since 1982. The largest Dunegons & Dragons game featured 1,227 players. It only cost $2,000 to make the first Dungeons & Dragons game set. Gary Gygax's children helped him perfect Dungeons & Dragons and chose its name. Marvel made a Dungeons & Dragons cartoon.

&nbsp;&nbsp;&nbsp;&nbsp;This is the text of the third paragraph.

--

Text will automatically move to new lines when it doesn't fit on the page anymore based on the font files and scalar values that are used for the text in the spell book (see spell book option documentation for more info). The first normal paragraph in a spell description will always have the first line fully left aligned without an indent and all following paragraphs will have an indented first line. This is done to match the formatting of spell descriptions in the official D&D source material books.

## Changing the Font Variant

To change the font variant being used between regular text, **bold text**, *italic text*, and ***bold italic text***, use these font tags:

- \<r\> (for regular text)
- \<b\> (for **bold text**)
- \<i\> (for *italic text*)
- \<bi\> (for ***bold italic text***)
- \<ib\> (also for ***bold italic text***)

Font tags will make the text that comes after them the corresponding font variant until either the end of the text or the next font tag, whichever comes first.

Font tags can be escaped with backslashes. For example, `\<r>` will appear as just `<r>` in a spell description without changing the font. Multiple backslashes can also be used to escape font tags and only the last one will be removed. For example, `\\\<bi>` will appear in a spell description as `\\<bi>`.

This description

```json
"description": "This text will be regular. <b> This text will be bold. <i> This text will be italic. <bi> This text will be bold-italic. <r> This text will be regular again. <ib> This text will be bold-italic again. <r> Here is an escaped bold font tag: \\<b>. This will just appear as a plain font tag in the text without actually making the text bold."
```

will look similar to this in the spell book:

--

This text will be regular. **This text will be bold.** *This text will be italic.* ***This text will be bold-italic.*** This text will be regular again. ***This text will be bold-italic again.*** Here is an escaped bold font tag: \<b\>. This will just appear as a plain font tag in the text without actually making the text bold.

--

You can also do this with the strings in `Custom` values in spell fields that allow either `Custom` or `Controlled` variants, as well as in tables. Most text that allows font tag processing will start in the regular font variant by default. Some exceptions are spell field names and table column labels (both start in bold).

## Bullet Point Lists

To make text appear in a single bullet point, have the text be on a line that starts with either the ascii dash character followed by non-newline whitespace "- " or a unicode bullet character followed by non-newline whitespace "• " (unicode hex 0x2022). All neighboring bullet point lines with no other types of lines / paragraphs / text in between will be combined into a bullet point list. Bullet points cannot be nested.

This description

```json
"description": "Here is a bullet point list:\\n- List item 1\\nL- List item 2\\n- List item 3\\nThis text will separate the bullet point list above and the one below. Here is another bullet point list:\\n• List item A\\n• List item B\\n• List item C\\n• List item D"
```

will look similar to this in the spell book:

--

Here is a bullet point list:

- List item 1
- List item 2
- List item 3

&nbsp;&nbsp;&nbsp;&nbsp;This text will separate the bullet point list above and the one below. Here is another bullet point list:

- List item A
- List item B
- List item C
- List item D

--

## Tables

To insert a table into this text, use a table tag `[table][x]` at the start of a paragraph, replacing the `x` with a nonnegative integer. The number used for the `x` value will determine which table is used at that location in the spell description. It will pull from the list of tables in the `tables` field in the json file and use the `x` value as an index in that list. Table indexes start at 0, so `[table][0]` would use the first table from the `tables` field, `[table][1]` would use the second, `[table][2]` would use the third, and so on. Tables cannot be nested.

This description

```json
"description": "Here is a table:\\n[table][1]\\nHere is another table:\\n[table][0]"
```

with this `tables` value

```json
"tables":
[
	{
		"title": "Title for the Table That Appears Second",
		"column_labels":
		[
			"Left Column",
			"Right Column"
		],
		"cells":
		[
			[
				"Alpha",
				"d4"
			],
			[
				"Beta",
				"d6"
			],
			[
				"Gamma",
				"d8"
			],
			[
				"Delta",
				"d10"
			]
		]
	},
	{
		"title": "",
		"column_labels":
		[
			"Column 1",
			"Column 2",
			"Column 3"
		],
		"cells":
		[
			[
				"First row that comes at the start",
				"1",
				"A"
			],
			[
				"Second row that comes after the first row",
				"2",
				"B"
			],
			[
				"Third row that comes last",
				"3",
				"C"
			]
		]
	}
]
```

will look similar to this in the spell book (formatting may vary):

--

Here is a table:

| Left column   | Right column  |
|:-------------:|:-------------:|
| Alpha         | d4            |
| Beta          | d6            |
| Gamma         | d8            |
| Delta         | d10           |

&nbsp;&nbsp;&nbsp;&nbsp;Here is another table:

**Title for the Table That Appears Second**

| Column 1                                  | Column 2      | Column 3      |
|:----------------------------------------- |:-------------:|:-------------:|
| First row that comes at the start         | 1             | A             |
| Second row that comes after the first row | 2             | B             |
| Third row that comes last                 | 3             | C             |

--

Any text that comes between a table tag and a newline will not be processed. Table tags that are not at the start of a new line are invalid. Table tags can be escaped just like font tags ("\\[table][5]", "\\[table][0]", etc.). Tokens that are valid table tags with 1 or more backslashes at the start will have the first backslash removed. Table tokens with an index that is out of range of the `tables` list are invalid. Invalid table tokens are treated like non-special tokens.

See the info on the `tables` field below for more information on tables and how to format them.

# `upcast_description` Field
---

Either `None` value

```json
"upcast_description": null
```

or `Some` value with any string.

```json
"upcast_description": "Text that describes what happens when you cast a spell at a higher level."
```

The string in the `Some` variant of this field allows use of the same special tokens described above in the `description` field info.

If the `None` value is used for this field, this field will not add any text to the spell book. If the `Some` value is used for this field, an indent and the text "Using a Higher-Level Spell Slot. " in bold will be put before the text in this field's string (starting with regular font).

For example, the `Some` variant example above will look like this in a spell book:

--

&nbsp;&nbsp;&nbsp;&nbsp;**Using a Higher-Level Spell Slot.** Text that describes what happens when you cast a spell at a higher level.

--

# `tables` Field
---

Contains a list of tables.

Can either be empty

```json
"tables": []
```

have a single table

```json
"tables":
[
	{
		"title": "Table Title",
		"column_labels":
		[
			"Column 1",
			"Column 2"
		],
		"cells":
		[
			[
				"Row 1",
				"A"
			],
			[
				"Row 2",
				"B"
			],
			[
				"Row 3",
				"C"
			]
		]
	}
]
```

or have multiple tables

```json
"tables":
[
	{
		"title": "",
		"column_labels":
		[
			"Letter",
			"Animal"
		],
		"cells":
		[
			[
				"A",
				"Ant"
			],
			[
				"B",
				"Bat"
			],
			[
				"C",
				"Cat"
			],
			[
				"D",
				"Dog"
			]
		]
	},
	{
		"title": "Types of Dice",
		"column_labels":
		[
			"Die Name",
			"Value Range",
			"Shape"
		],
		"cells":
		[
			[
				"d4",
				"1-4",
				"Tetrahedron"
			],
			[
				"d6",
				"1-6",
				"Cube"
			],
			[
				"d8",
				"1-8",
				"Octahedron"
			],
			[
				"d10",
				"1-10",
				"Pentagonal Trapezohedron"
			],
			[
				"d12",
				"1-12",
				"Dodecahedron"
			],
			[
				"d20",
				"1-20",
				"Icosahedron"
			],
			[
				"d100",
				"1-100",
				"Two pentagonal Trapezohedra (or a zocchihedron if you have an actual d100)"
			]
		]
	}
]
```

These tables can be placed into a spell's description using table tags (see `description` field info above).

Strings in the arrays represent individual cells. Each array of strings represents a row of cells.

Tables and their titles will be horizontally center aligned and start in bold font by default.

Tables with empty titles will not have any title text placed above the table cells.

Strings in the `column_labels` field will be displayed on the first row of the table starting in bold font by default. Not having any strings in the `column_labels` field will cause the table to skip over that row.

Font tags work inside table cells, titles, and column labels. Each cell starts with in the regular font variant by default. Having a cell with an empty string will cause that cell to still appear in the table but be empty.

All cells in a column share horizontal alignment (including the label for that column), either left-aligned or center-aligned. Columns where all cells can fit on one line will be center-aligned. Columns where at least one cell needs to wrap to a second line will be left-aligned.

If a table is small enough to fit on a single page, it will try to make sure it all stays on one page. If it can fit on the current page the text is on, it will apply itself there. Otherwise it will move to a new page and begin there if there is not enough room left for it on the current page. If a table is too big to even fit on a single page, it will begin applying itself wherever the text currently is regardless of how much space is left on the page.

Tables do not have to be perfectly rectangular, they can be jagged (missing / having extra columns on some rows). Jagged tables will cause empty cells to appear at the ends of other rows that didn't define a value for those columns.

The "Types of Dice" example table above will look similar to this in a spell book (formatting may vary):

**Types of Dice**

| Die Name | Value Range   | Shape                                                                               |
|:--------:|:-------------:|:----------------------------------------------------------------------------------- |
| d4       | 1-4           | Tetrahedron                                                                         |
| d6       | 1-6           | Cube                                                                                |
| d8       | 1-8           | Octahedron                                                                          |
| d10      | 1-10          | Pentagonal Trapezohedron                                                            |
| d12      | 1-12          | Dodecahedron                                                                        |
| d20      | 1-20          | Icosahedron                                                                         |
| d100     | 1-100         | Two pentagonal Trapezohedra (or a zocchihedron if you have an actual 100 sided die) |
