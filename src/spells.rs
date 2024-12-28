//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//	Data structures for defining, creating, and using spells
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fmt;
use std::fs;
use std::io::BufReader;
use std::error;

use serde::{Serialize, Deserialize};
use serde_json::{from_reader, to_writer, to_writer_pretty};

/// Holds spell fields with either a controlled value or a custom value represented by a string.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[allow(private_bounds)]
pub enum SpellField<T: fmt::Display>
{
	/// Controlled values are meant to be fields with limited values so invalid data cannot be displayed.
	///
	/// Controlled values must implement Display so the spell struct can display them without converting them manually.
	Controlled(T),
	/// Custom values allow for anything to be displayed in the spellbook.
	///
	/// Custom values appear in spell files as text surrounded by quotes in a field that otherwise has controlled values.
	Custom(String)
}

// Converts SpellFields into strings
impl<T: fmt::Display> fmt::Display for SpellField<T>
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		match self
		{
			Self::Controlled(controlled_value) => write!(f, "{}", controlled_value),
			Self::Custom(custom_value) => write!(f, "{}", custom_value)
		}
	}
}

/// The level of a spell.
// 0 is a cantrip, max level is 9
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

// Converts levels into strings
impl fmt::Display for Level
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let text = match self
		{
			Self::Cantrip => String::from("cantrip"),
			Self::Level1 => String::from("1st-Level"),
			Self::Level2 => String::from("2nd-Level"),
			Self::Level3 => String::from("3rd-Level"),
			_ => format!("{}th-Level", u8::from(*self))
		};
		write!(f, "{}", text)
	}
}

// Allows Levels to be created from integers (u8) for easier usage
impl TryFrom<u8> for Level
{
	type Error = &'static str;

	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		match value
		{
			0 => Ok(Self::Cantrip),
			1 => Ok(Self::Level1),
			2 => Ok(Self::Level2),
			3 => Ok(Self::Level3),
			4 => Ok(Self::Level4),
			5 => Ok(Self::Level5),
			6 => Ok(Self::Level6),
			7 => Ok(Self::Level7),
			8 => Ok(Self::Level8),
			9 => Ok(Self::Level9),
			_ => Err("Spell levels must be between 0 and 9 (inclusive).")
		}
	}
}

// Converts spell levels into integers (u8)
impl From<Level> for u8
{
	fn from(level: Level) -> Self
	{
		match level
		{
			Level::Cantrip => 0,
			Level::Level1 => 1,
			Level::Level2 => 2,
			Level::Level3 => 3,
			Level::Level4 => 4,
			Level::Level5 => 5,
			Level::Level6 => 6,
			Level::Level7 => 7,
			Level::Level8 => 8,
			Level::Level9 => 9
		}
	}
}

/// The school of magic a spell belongs to
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

// Allows strings of magic schools to be converted to the MagicSchool type
impl TryFrom<&str> for MagicSchool
{
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		match value.to_lowercase().as_str()
		{
			"abjuration" => Ok(Self::Abjuration),
			"conjuration" => Ok(Self::Conjuration),
			"divination" => Ok(Self::Divination),
			"enchantment" => Ok(Self::Enchantment),
			"evocation" => Ok(Self::Evocation),
			"illusion" => Ok(Self::Illusion),
			"necromancy" => Ok(Self::Necromancy),
			"transmutation" => Ok(Self::Transmutation),
			_ => Err("Invalid MagicSchool string.")
		}
	}
}

// Converts magic schools into strings
impl fmt::Display for MagicSchool
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let text = match self
		{
			Self::Abjuration => String::from("Abjuration"),
			Self::Conjuration => String::from("Conjuration"),
			Self::Divination => String::from("Divination"),
			Self::Enchantment => String::from("Enchantment"),
			Self::Evocation => String::from("Evocation"),
			Self::Illusion => String::from("Illusion"),
			Self::Necromancy => String::from("Necromancy"),
			Self::Transmutation => String::from("Transmutation")
		};
		write!(f, "{}", text)
	}
}

/// The amount of time it takes to cast a spell.
///
/// u16 values are the number of units of time it takes to cast the spell,
/// variants are the unit of time.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CastingTime
{
	Seconds(u16),
	Actions(u16),
	BonusAction,
	/// String is the circumstance in which the reaction can be triggered.
	/// Ex: "which you take when you or a creature within 60 feet of you falls" or
	/// "which you take when you see a creature within 60 feet of you casting a spell".
	///
	/// Note: whatever you put for this, it will come after the string "1 reaction, " on the spell page.
	Reaction(String),
	Minutes(u16),
	Hours(u16),
	Days(u16),
	Weeks(u16),
	Months(u16),
	Years(u16),
	Special
}

// Converts casting times into strings
impl fmt::Display for CastingTime
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let text = match self
		{
			Self::Seconds(t) => get_amount_string(*t, "second"),
			Self::Actions(t) => get_amount_string(*t, "action"),
			Self::BonusAction => String::from("1 bonus action"),
			Self::Reaction(e) => format!("1 reaction, {}", *e),
			Self::Minutes(t) => get_amount_string(*t, "minute"),
			Self::Hours(t) => get_amount_string(*t, "hour"),
			Self::Days(t) => get_amount_string(*t, "day"),
			Self::Weeks(t) => get_amount_string(*t, "week"),
			Self::Months(t) => get_amount_string(*t, "month"),
			Self::Years(t) => get_amount_string(*t, "year"),
			Self::Special => String::from("Special")
		};
		write!(f, "{}", text)
	}
}

/// Holds a distance value. The enum variant determine its unit of measurement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Distance
{
	Feet(u16),
	Miles(u16)
}

impl Distance
{
	/// Returns a string of the distance in the format of "d-u" where
	/// d is the distance value and u is the unit of measurement.
	///
	/// Used in displaying distances for Aoe.
	pub fn get_aoe_string(&self) -> String
	{
		match self
		{
			Self::Feet(d) => format!("{}-foot", d),
			Self::Miles(d) => format!("{}-mile", d)
		}
	}
}

// Converts Distances into strings
impl fmt::Display for Distance
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let text = match self
		{
			Self::Feet(d) => format!("{} feet", d),
			Self::Miles(d) => format!("{} miles", d)
		};
		write!(f, "{}", text)
	}
}

/// Area of Effect.
/// The volumnetric shape in which a spell's effect(s) take place.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Aoe
{
	/// Distance defines length of line (width should be in spell description).
	Line(Distance),
	/// Distance defines length / height and diameter of cone.
	Cone(Distance),
	/// Distance defines the length of the edges of the cube.
	Cube(Distance),
	/// Distance defines radius of sphere (same thing functionally as emanation in game rules).
	Sphere(Distance),
	/// Distance defines radius of emanation (same thing functionally as emanation in game rules).
	Emanation(Distance),
	/// Distance defines radius of hemisphere.
	Hemisphere(Distance),
	/// Distances define radius and height of cylinder (respectively).
	Cylinder(Distance, Distance),
}

// Converts Aoes into strings
impl fmt::Display for Aoe
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let text = match self
		{
			Self::Line(l) => format!("{} line", l.get_aoe_string()),
			Self::Cone(l) => format!("{} cone", l.get_aoe_string()),
			Self::Cube(l) => format!("{} cube", l.get_aoe_string()),
			Self::Sphere(r) => format!("{} radius", r.get_aoe_string()),
			Self::Emanation(r) => format!("{} emanation", r.get_aoe_string()),
			Self::Hemisphere(r) => format!("{} radius hemisphere", r.get_aoe_string()),
			Self::Cylinder(r, h) => format!("{} radius, {} height cylinder", r.get_aoe_string(), h.get_aoe_string())
		};
		write!(f, "{}", text)
	}
}

/// The farthest distance away a spell can target things.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Range
{
	/// The Aoe option in this variant is for spells that have areas of effect that come from the spellcaster.
	/// Ex: "Burning Hands" has a range of "Self (15-foot cone)".
	Yourself(Option<Aoe>),
	Touch,
	/// This variant is for plain distance ranges like "60 feet" or "5 miles".
	Dist(Distance),
	Sight,
	Unlimited,
	Special
}

// Converts spell ranges into strings
impl fmt::Display for Range
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let text = match self
		{
			Self::Yourself(o) =>
			{
				match o
				{
					None => String::from("Self"),
					Some(a) => format!("Self ({})", a)
				}
			}
			Self::Touch => String::from("Touch"),
			Self::Dist(d) => d.to_string(),
			Self::Sight => String::from("Sight"),
			Self::Unlimited => String::from("Unlimited"),
			Self::Special => String::from("Special")
		};
		write!(f, "{}", text)
	}
}

/// The length of time a spell's effect(s) lasts.
///
/// u16 values are the number of units the spell can last.
/// Bool values are whether or not the spell requires concentration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

// Converts spell durations into strings
impl fmt::Display for Duration
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let text = match self
		{
			Self::Instant => String::from("Instantaneous"),
			Self::Seconds(t, c) =>
			{
				let s = get_amount_string(*t, "second");
				if *c { format!("Concentration, up to {}", s) }
				else { s }
			},
			Self::Rounds(t, c) =>
			{
				let s = get_amount_string(*t, "round");
				if *c { format!("Concentration, up to {}", s) }
				else { s }
			},
			Self::Minutes(t, c) =>
			{
				let s = get_amount_string(*t, "minute");
				if *c { format!("Concentration, up to {}", s) }
				else { s }
			},
			Self::Hours(t, c) =>
			{
				let s = get_amount_string(*t, "hour");
				if *c { format!("Concentration, up to {}", s) }
				else { s }
			},
			Self::Days(t, c) =>
			{
				let s = get_amount_string(*t, "day");
				if *c { format!("Concentration, up to {}", s) }
				else { s }
			},
			Self::Weeks(t, c) =>
			{
				let s = get_amount_string(*t, "week");
				if *c { format!("Concentration, up to {}", s) }
				else { s }
			},
			Self::Months(t, c) =>
			{
				let s = get_amount_string(*t, "month");
				if *c { format!("Concentration, up to {}", s) }
				else { s }
			},
			Self::Years(t, c) =>
			{
				let s = get_amount_string(*t, "year");
				if *c { format!("Concentration, up to {}", s) }
				else { s }
			},
			Self::DispelledOrTriggered(c) =>
			{
				let s = String::from("Until dispelled or triggered");
				if *c { format!("Concentration, up {}", s) }
				else { s }
			}
			Self::UntilDispelled(c) =>
			{
				let s = String::from("Until dispelled");
				if *c { format!("Concentration, up {}", s) }
				else { s }
			}
			Self::Permanent => String::from("Permanent"),
			Self::Special(c) =>
			{
				let s = String::from("Special");
				if *c {format!("Concentration, {}", s) }
				else { s }
			}
		};
		write!(f, "{}", text)
	}
}

/// Holds a table that goes in a spellbook description.
/// It does not need to be a perfect square, jagged tables are allowed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Table
{
	/// The title text that goes above the table. Leave as empty string for no title.
	pub title: String,
	/// The labels above each column on the first row of the table.
	/// Leave entire vec empty for no column labels and individual strings empty to skip over a column.
	pub column_labels: Vec<String>,
	/// Vec of the text that goes in each individual cell in the table. Outer vec is the row of the cell (up and
	/// down placement), inner vec is the column of the cell (left to right placement). Lower row indexes mean higher
	/// up vertically on the table, lower column indexes mean more to the left.
	pub cells: Vec<Vec<String>>
}

// Gets a string of an amount of something like "1 minute", "5 minutes", "1 hour", or "3 hours"
// Note: the unit should be singular, not plural because an 's' will be added to the end of it if num is anything but 1
fn get_amount_string(num: u16, unit: &str) -> String
{
	if num == 1
	{
		format!("1 {}", unit)
	}
	else
	{
		format!("{} {}s", num, unit)
	}
}

/// Data containing all of the information about a spell needed to display it in a spellbook.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Spell
{
	pub name: String,
	/// Can be custom value or Level.
	pub level: SpellField<Level>,
	/// Can be custom value or MagicSchool.
	pub school: SpellField<MagicSchool>,
	/// Whether or not the spell can be casted as a ritual.
	pub is_ritual: bool,
	/// Can be custom value or CastingTime.
	pub casting_time: SpellField<CastingTime>,
	/// Can be custom value or Range.
	pub range: SpellField<Range>,
	/// Whether or not the spell requires a verbal component to be cast.
	pub has_v_component: bool,
	/// Whether or not the spell requires a somantic component to be cast.
	pub has_s_component: bool,
	/// Text that lists the material components a spell might need to be cast.
	/// A value of `None` represents the spell not needing any material components.
	pub m_components: Option<String>,
	/// Can be custom value or Duration.
	pub duration: SpellField<Duration>,
	/// Text that describes the effects of the spell.
	/// 
	/// Can be formatted with font changing tags, bullet points, and tables.
	///
	/// See spell file documentation for more information (<https://github.com/ChandlerJayCalkins/dnd_spellbook_maker>).
	pub description: String,
	/// Optional text that describes the benefits a spell gains from being upcast (being cast at a level higher than
	/// its base level if it's a non-cantrip or being cast by a character higher than a certain level if its a
	/// cantrip).
	pub upcast_description: Option<String>,
	/// Any tables that the spell might have in its description
	pub tables: Vec<Table>
}

impl Spell
{
	/// Constructs a spell object from a json file.
	///
	/// # Parameters
	///
	/// - `file_path` The path to the json file to create the spell from.
	///
	/// # Output
	///
	/// - `Ok` A spell object.
	/// - `Err` Any errors that occured.
	pub fn from_json_file(file_path: &str) -> Result<Self, Box<dyn error::Error>>
	{
		let file = fs::File::open(file_path)?;
		let reader = BufReader::new(file);
		let spell = from_reader(reader)?;
		Ok(spell)
	}

	/// Saves a spell to a json file.
	///
	/// # Parameters
	///
	/// - `file_path` The file path to save the spell to.
	/// - `compress` True to put all the data onto one line, false to make the file more human readable.
	///
	/// # Output
	///
	/// - `Ok` Nothing if there were no errors.
	/// - `Err` Any errors that occurred.
	pub fn to_json_file(&self, file_path: &str, compress: bool) -> Result<(), Box<dyn error::Error>>
	{
		let file = fs::File::create(file_path)?;
		if compress { to_writer(file, self)?; }
		else { to_writer_pretty(file, self)?; }
		Ok(())
	}

	/// Gets a string of the required components for a spell.
	///
	/// Ex: "V, S, M (a bit of sulfur and some wood bark)", "V, S", "V, M (a piece of hair)".
	pub fn get_component_string(&self) -> String
	{
		let mut component_string = String::new();
		// If there is a v component
		if self.has_v_component
		{
			// Add a v to the string
			component_string += "V";
		}
		// If there is an s component
		if self.has_s_component
		{
			// If there is at least 1 component already
			if component_string.len() > 0
			{
				// Add a comma to the string
				component_string += ", ";
			}
			// Add an s to the string
			component_string += "S";
		}
		// If there is an m component
		if let Some(m) = &self.m_components
		{
			// If there is at least 1 component already
			if component_string.len() > 0
			{
				// Add a comma to the string
				component_string += ", ";
			}
			// Add the m component(s) to the string
			component_string += format!("M ({})", m).as_str();
		}

		// If there are no components, set the string to "None"
		if component_string.is_empty() { component_string = "None".to_string(); }
		// Return the string
		component_string
	}

	/// Gets the school and level info from a spell and turns it into text that says something like "nth-Level School-Type".
	///
	/// Ex: "1st-Level abjuration", "8th-Level transmutation", "evocation cantrip".
	pub fn get_level_school_text(&self) -> String
	{
		// Gets a string of the level and the school from the spell
		let text = match &self.level
		{
			// If the spell is a cantrip, make the school come first and then the level
			SpellField::Controlled(Level::Cantrip) => format!("{} {}", &self.school, &self.level),
			// If the spell is any other level or a custom value
			_ =>
			{
				let school_text = match &self.school
				{
					// If the spell's school is a controlled value, get a lowercase string of it
					SpellField::Controlled(school) => school.to_string().to_lowercase(),
					// If the spell's school has a custom value, just use that string untouched
					SpellField::Custom(s) => s.clone()
				};
				// Make the level come before the school
				format!("{} {}", &self.level, school_text)
			}
		};
		// Return the string
		text
	}

	/// Gets the casting time and ritual info from a spell and turns it into text that says something like
	/// "1 action or Ritual", "1 bonus action", or "2 hours"
	pub fn get_casting_time_text(&self) -> String
	{
		// If the spell is a ritual, return the casting time with "or Ritual" at the end of it
		if self.is_ritual { format!("{} or Ritual", self.casting_time) }
		// If the spell is not a ritual, just return the casting time
		else { self.casting_time.to_string() }
	}
}
