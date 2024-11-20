/*
* All object types needed for defining spells
*/

use std::fmt;
use std::fs;
use std::io::{Write, BufReader};
use std::error;
use serde::{Serialize, Deserialize};
use serde_json::{from_reader, to_writer, to_writer_pretty};

/// For reading and writing to spell files.
trait SpellFileString: Sized
{
	/// Turns an object into a string that can be written into a spell file.
	fn to_spell_file_string(&self) -> String;
	/// Tries to turn a string from a spell file into an object.
	///
	/// # Parameters
	/// - `s` Str of a field value from a spell file to convert into an object.
	/// - `file_name` The name of the file that `s` is from (for producing errors).
	/// - `field` The field that s was a value of (for producing errors).
	///
	/// # Output
	/// - `Ok` An instance of the object this trait was implemented for.
	/// - `Err` Any errors that occurred while processing.
	fn from_spell_file_string(s: &str, file_name: &str, field: &str) -> Result<Self, Box<SpellFileError>>;
}

/// Holds spell fields with either a controlled value or a custom value represented by a string.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[allow(private_bounds)]
pub enum SpellField<T: fmt::Display + SpellFileString>
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
impl<T: fmt::Display + SpellFileString> fmt::Display for SpellField<T>
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

// Allows spell fields to be written to and read from spell files
impl<T: fmt::Display + SpellFileString> SpellFileString for SpellField<T>
{
	fn to_spell_file_string(&self) -> String
	{
		match self
		{
			// If this is a controlled value, use this value's implementation of SpellFileString
			Self::Controlled(controlled_value) => controlled_value.to_spell_file_string(),
			// If this is a custom value, just return this string surrounded by quotes
			Self::Custom(custom_value) => format!("\"{}\"", (*custom_value).clone())
		}
	}

	fn from_spell_file_string(s: &str, file_name: &str, field: &str) -> Result<Self, Box<SpellFileError>>
	{
		// Custom values are denoted with quotes around it
		// If there are quotes around this value
		if s.starts_with('"') && s.ends_with('"')
		{
			// Construct a Custom SpellField from the text inside the quotes
			Ok(Self::Custom(String::from(&s[1..s.len()-1])))
		}
		// If the string does not both start and end with quotes, assume it's a controlled value
		else
		{
			// Attempt to parse the string with the controlled value's type's SpellFileString implementation
			match T::from_spell_file_string(s, file_name, field)
			{
				Ok(v) => Ok(Self::Controlled(v)),
				Err(e) => Err(e)
			}
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

// Allows levels to be written to and read from spell files
impl SpellFileString for Level
{
	fn to_spell_file_string(&self) -> String
	{
		u8::from(*self).to_string()
	}

	fn from_spell_file_string(s: &str, file_name: &str, field: &str) -> Result<Self, Box<SpellFileError>>
	{
		let parse_result = s.parse::<u8>();
		match parse_result
		{
			Ok(n) => match Level::try_from(n)
			{
				Ok(level) => Ok(level),
				Err(_) => Err(SpellFileError::get_box(false, file_name, format!("{} {}", field, s).as_str()))
			},
			Err(_) => Err(SpellFileError::get_box(false, file_name, format!("{} {}", field, s).as_str()))
		}
	}
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

// Allows magic schools to be written to and read from spell files
impl SpellFileString for MagicSchool
{
	fn to_spell_file_string(&self) -> String
	{
		self.to_string().to_lowercase()
	}

	fn from_spell_file_string(s: &str, file_name: &str, field: &str) -> Result<Self, Box<SpellFileError>>
	{
		match MagicSchool::try_from(s)
		{
			Ok(school) => Ok(school),
			Err(_) => Err(SpellFileError::get_box(false, file_name, format!("{} {}", field, s).as_str()))
		}
	}
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

// Allows casting times to be written to and read from spell files
impl SpellFileString for CastingTime
{
	fn to_spell_file_string(&self) -> String
	{
		match self
		{
			Self::Seconds(t) => format!("actions {}", *t),
			Self::Actions(t) => format!("actions {}", *t),
			Self::BonusAction => String::from("bonusaction"),
			Self::Reaction(e) => format!("reaction \"{}\"", e),
			Self::Minutes(t) => format!("minutes {}", *t),
			Self::Hours(t) => format!("hours {}", *t),
			Self::Days(t) => format!("days {}", *t),
			Self::Weeks(t) => format!("weeks {}", *t),
			Self::Months(t) => format!("months {}", *t),
			Self::Years(t) => format!("years {}", *t),
			Self::Special => format!("special")
		}
	}

	fn from_spell_file_string(s: &str, file_name: &str, field: &str) -> Result<Self, Box<SpellFileError>>
	{
		// Gets a vec of all the tokens in the string
		let tokens: Vec<_> = s.split_whitespace().collect();
		// Error object to be returned for when the string can't be parsed
		let error = SpellFileError::get_box(false, file_name, format!("{} {}", field, s).as_str());
		// If there aren't any tokens in the string, return an error
		if tokens.len() < 1 { return Err(error); }
		// Determine what type of casting time it is based on the first token
		match tokens[0].to_lowercase().as_str()
		{
			"seconds" =>
			{
				// If there's a second token
				if tokens.len() > 1
				{
					// Try to parse that second token into a u16 and use it as the value of this object
					match tokens[1].parse::<u16>()
					{
						Ok(n) => return Ok(Self::Seconds(n)),
						Err(_) => return Err(error)
					}
				}
				// If there's no second token, return an error
				else { return Err(error); }
			},
			"actions" =>
			{
				// If there's a second token
				if tokens.len() > 1
				{
					// Try to parse that second token into a u16 and use it as the value of this object
					match tokens[1].parse::<u16>()
					{
						Ok(n) => return Ok(Self::Actions(n)),
						Err(_) => return Err(error)
					}
				}
				// If there's no second token, return an error
				else { return Err(error); }
			},
			"bonusaction" =>
			{
				return Ok(Self::BonusAction);
			},
			"reaction" =>
			{
				// Try to get all of the text inside of quotes that comes after the first token
				match get_text_field(&tokens[1..].join(" "), file_name, "casting_time: reaction")
				{
					// If it succeeded, return a reaction with that text
					Ok(desc) => return Ok(Self::Reaction(desc)),
					Err(e) => return Err(e)
				}
			},
			"minutes" =>
			{
				// If there's a second token
				if tokens.len() > 1
				{
					// Try to parse that second token into a u16 and use it as the value of this object
					match tokens[1].parse::<u16>()
					{
						Ok(n) => return Ok(Self::Minutes(n)),
						Err(_) => return Err(error)
					}
				}
				// If there's no second token, return an error
				else { return Err(error); }
			},
			"hours" =>
			{
				// If there's a second token
				if tokens.len() > 1
				{
					// Try to parse that second token into a u16 and use it as the value of this object
					match tokens[1].parse::<u16>()
					{
						Ok(n) => return Ok(Self::Hours(n)),
						Err(_) => return Err(error)
					}
				}
				// Try to parse that second token into a u16 and use it as the value of this object
				else { return Err(error); }
			},
			"days" =>
			{
				// If there's a second token
				if tokens.len() > 1
				{
					// Try to parse that second token into a u16 and use it as the value of this object
					match tokens[1].parse::<u16>()
					{
						Ok(n) => return Ok(Self::Days(n)),
						Err(_) => return Err(error)
					}
				}
				// Try to parse that second token into a u16 and use it as the value of this object
				else { return Err(error); }
			},
			"weeks" =>
			{
				// If there's a second token
				if tokens.len() > 1
				{
					// Try to parse that second token into a u16 and use it as the value of this object
					match tokens[1].parse::<u16>()
					{
						Ok(n) => return Ok(Self::Weeks(n)),
						Err(_) => return Err(error)
					}
				}
				// Try to parse that second token into a u16 and use it as the value of this object
				else { return Err(error); }
			},
			"months" =>
			{
				// If there's a second token
				if tokens.len() > 1
				{
					// Try to parse that second token into a u16 and use it as the value of this object
					match tokens[1].parse::<u16>()
					{
						Ok(n) => return Ok(Self::Months(n)),
						Err(_) => return Err(error)
					}
				}
				// Try to parse that second token into a u16 and use it as the value of this object
				else { return Err(error); }
			},
			"years" =>
			{
				// If there's a second token
				if tokens.len() > 1
				{
					// Try to parse that second token into a u16 and use it as the value of this object
					match tokens[1].parse::<u16>()
					{
						Ok(n) => return Ok(Self::Years(n)),
						Err(_) => return Err(error)
					}
				}
				// Try to parse that second token into a u16 and use it as the value of this object
				else { return Err(error); }
			},
			"special" =>
			{
				return Ok(Self::Special);
			},
			// If the first token wasn't recognized as a CastingTime type, return an error
			_ => { return Err(error); }
		}
	}
}

// Allows strings to be converted into casting times
impl TryFrom<&str> for CastingTime
{
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		match Self::from_spell_file_string(value, "NOT A FILE", "NO FIELD")
		{
			Ok(time) => Ok(time),
			Err(_) => Err("Invalid CastingTime String.")
		}
	}
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
	/// Used in displaying distances for AOEs.
	pub fn get_aoe_string(&self) -> String
	{
		match self
		{
			Self::Feet(d) => format!("{}-foot", d),
			Self::Miles(d) => format!("{}-mile", d)
		}
	}
}

impl SpellFileString for Distance
{
	fn to_spell_file_string(&self) -> String
	{
		match self
		{
			Self::Feet(d) => format!("feet {}", d),
			Self::Miles(d) => format!("miles {}", d)
		}
	}

	fn from_spell_file_string(s: &str, file_name: &str, field: &str) -> Result<Self, Box<SpellFileError>>
	{
		// Get a vec of all the tokens in the string
		let tokens: Vec<_> = s.split_whitespace().collect();
		// Error object to be returned for when the string can't be parsed
		let error = SpellFileError::get_box(false, file_name, format!("{} {}", field, s).as_str());
		// If there aren't at least 2 tokens in the string (1 for unit, 1 for value), return an error
		if tokens.len() < 2 { return Err(error); }
		// Determine what the unit of measurement for this distance is based on the first token
		match tokens[0].to_lowercase().as_str()
		{
			"feet" =>
			{
				// Try to parse the second token as a u16 for the value of the distance
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err(error)
				};
				Ok(Self::Feet(num))
			},
			"miles" =>
			{
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err(error)
				};
				Ok(Self::Miles(num))
			},
			_ => Err(error)
		}
	}
}

// Allows Distances to be created from strings
impl TryFrom<&str> for Distance
{
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		match Self::from_spell_file_string(value, "NOT A FILE", "NO FIELD")
		{
			Ok(d) => Ok(d),
			Err(_) => Err("Invalid Distance String.")
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
pub enum AOE
{
	/// Distance defines length of line (width should be in spell description).
	Line(Distance),
	/// Distance defines length / height and diameter of cone.
	Cone(Distance),
	/// Distance defines the length of the edges of the cube.
	Cube(Distance),
	/// Distance defines radius of sphere.
	Sphere(Distance),
	/// Distance defines radius of hemisphere.
	Hemisphere(Distance),
	/// Distances define radius and height of cylinder (respectively).
	Cylinder(Distance, Distance),
}

// Allows AOEs to be written to and read from spell files
impl SpellFileString for AOE
{
	fn to_spell_file_string(&self) -> String
	{
		match self
		{
			Self::Line(l) => format!("line {}", (*l).to_spell_file_string()),
			Self::Cone(l) => format!("cone {}", (*l).to_spell_file_string()),
			Self::Cube(l) => format!("cube {}", (*l).to_spell_file_string()),
			Self::Sphere(r) => format!("sphere {}", (*r).to_spell_file_string()),
			Self::Hemisphere(r) => format!("hemisphere {}", (*r).to_spell_file_string()),
			Self::Cylinder(r, h) => format!("cylinder {} {}", (*r).to_spell_file_string(), (*h).to_spell_file_string()),
		}
	}

	fn from_spell_file_string(s: &str, file_name: &str, field: &str) -> Result<Self, Box<SpellFileError>>
	{
		// Get a vec of all the tokens in the string
		let tokens: Vec<_> = s.split_whitespace().collect();
		// Error object to be returned for when the string can't be parsed
		let error = SpellFileError::get_box(false, file_name, format!("{} {}", field, s).as_str());
		// If there aren't at least 3 tokens in the string (1 for shape, 1 for dimension, 1 for unit), return an error
		if tokens.len() < 3 { return Err(error); }
		// Determine what type of AOE this is based on the first token
		match tokens[0].to_lowercase().as_str()
		{
			"line" =>
			{
				// Try to parse the following tokens as a Distance and use it to construct the aoe
				let dist = match Distance::from_spell_file_string(tokens[1..].join(" ").as_str(), file_name, field)
				{
					Ok(d) => d,
					Err(_) => return Err(error)
				};
				Ok(Self::Line(dist))
			},
			"cone" =>
			{
				let dist = match Distance::from_spell_file_string(tokens[1..].join(" ").as_str(), file_name, field)
				{
					Ok(d) => d,
					Err(_) => return Err(error)
				};
				Ok(Self::Cone(dist))
			},
			"cube" =>
			{
				let dist = match Distance::from_spell_file_string(tokens[1..].join(" ").as_str(), file_name, field)
				{
					Ok(d) => d,
					Err(_) => return Err(error)
				};
				Ok(Self::Cube(dist))
			},
			"sphere" =>
			{
				let dist = match Distance::from_spell_file_string(tokens[1..].join(" ").as_str(), file_name, field)
				{
					Ok(d) => d,
					Err(_) => return Err(error)
				};
				Ok(Self::Sphere(dist))
			},
			"hemisphere" =>
			{
				let dist = match Distance::from_spell_file_string(tokens[1..].join(" ").as_str(), file_name, field)
				{
					Ok(d) => d,
					Err(_) => return Err(error)
				};
				Ok(Self::Hemisphere(dist))
			},
			"cylinder" =>
			{
				// If there aren't at least 5 tokens for this aoe type, return an error
				// 1. Shape name
				// 2. Radius unit
				// 3. Radius
				// 4. Height unit
				// 5. Height
				if tokens.len() < 5 { return Err(error); }
				// Try to parse the second and third tokens as a Distance for the radius
				let dist1 = match Distance::from_spell_file_string(tokens[1..3].join(" ").as_str(), file_name, field)
				{
					Ok(d) => d,
					Err(_) => return Err(error)
				};
				// Try to parse the fourth and fifth tokens as a Distance for the height
				let dist2 = match Distance::from_spell_file_string(tokens[3..].join(" ").as_str(), file_name, field)
				{
					Ok(d) => d,
					Err(_) => return Err(error)
				};
				Ok(Self::Cylinder(dist1, dist2))
			},
			// If the first token wasn't recognized as an AOE type, return an error
			_ => Err(error)
		}
	}
}

// Allows AOEs to be created from strings
impl TryFrom<&str> for AOE
{
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		match Self::from_spell_file_string(value, "NOT A FILE", "NO FIELD")
		{
			Ok(aoe) => Ok(aoe),
			Err(_) => Err("Invalid AOE String.")
		}
	}
}

// Converts AOEs into strings
impl fmt::Display for AOE
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let text = match self
		{
			Self::Line(l) => format!("{} line", l.get_aoe_string()),
			Self::Cone(l) => format!("{} cone", l.get_aoe_string()),
			Self::Cube(l) => format!("{} cube", l.get_aoe_string()),
			Self::Sphere(r) => format!("{} radius", r.get_aoe_string()),
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
	/// The AOE option in this variant is for spells that have areas of effect that come from the spellcaster.
	/// Ex: "Burning Hands" has a range of "Self (15-foot cone)".
	Yourself(Option<AOE>),
	Touch,
	/// This variant is for plain distance ranges like "60 feet" or "5 miles".
	Dist(Distance),
	Sight,
	Unlimited,
	Special
}

// Allows ranges to be written to and read from spell files
impl SpellFileString for Range
{
	fn to_spell_file_string(&self) -> String
	{
		match self
		{
			Self::Yourself(o) =>
			{
				let text = String::from("self");
				match o
				{
					Some(aoe) => format!("{} {}", text, aoe.to_spell_file_string()),
					None => text
				}
			},
			Self::Touch => String::from("touch"),
			Self::Dist(d) => d.to_spell_file_string(),
			Self::Sight => String::from("sight"),
			Self::Unlimited => String::from("unlimited"),
			Self::Special => String::from("special")
		}
	}

	fn from_spell_file_string(s: &str, file_name: &str, field: &str) -> Result<Self, Box<SpellFileError>>
	{
		// Get a vec of all the tokens in the string
		let tokens: Vec<_> = s.split_whitespace().collect();
		// Error object to be returned for when the string can't be parsed
		let error = SpellFileError::get_box(false, file_name, format!("{} {}", field, s).as_str());
		// If there aren't any tokens in the string, return an error
		if tokens.len() < 1 { return Err(error); }
		// Determine what kind of Range to create based on the first token
		match tokens[0].to_lowercase().as_str()
		{
			"self" =>
			{
				// If there isn't at least a second token, assume this Range takes type None
				if tokens.len() < 2 { return Ok(Self::Yourself(None)); }
				// Try to constuct an AOE from the following tokens and use that to construct this Range object
				match AOE::from_spell_file_string(tokens[1..].join(" ").as_str(), file_name, field)
				{
					Ok(aoe) => Ok(Self::Yourself(Some(aoe))),
					Err(_) => Err(error)
				}
			},
			"touch" =>
			{
				Ok(Self::Touch)
			},
			"sight" =>
			{
				Ok(Self::Sight)
			},
			"unlimited" =>
			{
				Ok(Self::Unlimited)
			},
			"special" =>
			{
				Ok(Self::Special)
			},
			// If the first token is anything else, assume it's a Dist
			_ =>
			{
				// If there isn't at least a second token, return an error since this type needs a Distance
				if tokens.len() < 2 { return Err(error); }
				// Attempt to parse the following tokens as a Distance
				match Distance::from_spell_file_string(s, file_name, field)
				{
					Ok(dist) => Ok(Self::Dist(dist)),
					Err(_) => Err(error)
				}
			}
		}
	}
}

// Allows Ranges to be created from strings
impl TryFrom<&str> for Range
{
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		match Self::from_spell_file_string(value, "NOT A FILE", "NO FIELD")
		{
			Ok(range) => Ok(range),
			Err(_) => Err("Invalid Range String.")
		}
	}
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

// Allows durations to be written to and read from spell files
impl SpellFileString for Duration
{
	fn to_spell_file_string(&self) -> String
	{
		match self
		{
			Self::Instant => String::from("instant"),
			Self::Seconds(t, c) =>
			{
				let mut text = format!("seconds {}", *t);
				if *c { text += " concentration"; }
				text
			},
			Self::Rounds(t, c) =>
			{
				let mut text = format!("rounds {}", *t);
				if *c { text += " concentration"; }
				text
			},
			Self::Minutes(t, c) =>
			{
				let mut text = format!("minutes {}", *t);
				if *c { text += " concentration"; }
				text
			},
			Self::Hours(t, c) =>
			{
				let mut text = format!("hours {}", *t);
				if *c { text += " concentration"; }
				text
			},
			Self::Days(t, c) =>
			{
				let mut text = format!("days {}", *t);
				if *c { text += " concentration"; }
				text
			},
			Self::Weeks(t, c) =>
			{
				let mut text = format!("weeks {}", *t);
				if *c { text += " concentration"; }
				text
			},
			Self::Months(t, c) =>
			{
				let mut text = format!("months {}", *t);
				if *c { text += " concentration"; }
				text
			},
			Self::Years(t, c) =>
			{
				let mut text = format!("years {}", *t);
				if *c { text += " concentration"; }
				text
			},
			Self::DispelledOrTriggered(c) =>
			{
				let mut text = String::from("dispelledortriggered");
				if *c { text += " concentration"; }
				text
			},
			Self::UntilDispelled(c) =>
			{
				let mut text = String::from("untildispelled");
				if *c { text += " concentration"; }
				text
			},
			Self::Permanent => String::from("permanent"),
			Self::Special(c) =>
			{
				let mut text = String::from("special");
				if *c { text += " concentration"; }
				text
			}
		}
	}

	fn from_spell_file_string(s: &str, file_name: &str, field: &str) -> Result<Self, Box<SpellFileError>>
	{
		// Gets a vec of all the tokens in the string
		let tokens: Vec<_> = s.split_whitespace().collect();
		// Error object to be returned for when the string can't be parsed
		let error = SpellFileError::get_box(false, file_name, format!("{} {}", field, s).as_str());
		// If there aren't any tokens in this string, return an error
		if tokens.len() < 1 { return Err(error); }
		// Determine what type of Duration this is based on the first token
		match tokens[0].to_lowercase().as_str()
		{
			"instant" =>
			{
				Ok(Self::Instant)
			},
			"seconds" =>
			{
				// If there isn't a second token, return an error
				if tokens.len() < 2 { return Err(error); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err(error)
				};
				// If there's a third token
				if tokens.len() > 2
				{
					// If the third token signifies concentration
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						// Construct the Duration with the u16 and the concentration bool set to true
						Ok(Self::Seconds(num, true))
					}
					// If the third token is anything else, return an error
					else { Err(error) }
				}
				// If there is no third token, construct the Duration with the u16
				else { Ok(Self::Seconds(num, false)) }
			},
			"rounds" =>
			{
				// If there isn't a second token, return an error
				if tokens.len() < 2 { return Err(error); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err(error)
				};
				// If there's a third token
				if tokens.len() > 2
				{
					// If the third token signifies concentration
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						// Construct the Duration with the u16 and the concentration bool set to true
						Ok(Self::Rounds(num, true))
					}
					// If the third token is anything else, return an error
					else { Err(error) }
				}
				// If there is no third token, construct the Duration with the u16
				else { Ok(Self::Rounds(num, false)) }
			},
			"minutes" =>
			{
				// If there isn't a second token, return an error
				if tokens.len() < 2 { return Err(error); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err(error)
				};
				// If there's a third token
				if tokens.len() > 2
				{
					// If the third token signifies concentration
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						// Construct the Duration with the u16 and the concentration bool set to true
						Ok(Self::Minutes(num, true))
					}
					// If the third token is anything else, return an error
					else { Err(error) }
				}
				// If there is no third token, construct the Duration with the u16
				else { Ok(Self::Minutes(num, false)) }
			},
			"hours" =>
			{
				// If there isn't a second token, return an error
				if tokens.len() < 2 { return Err(error); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err(error)
				};
				// If there's a third token
				if tokens.len() > 2
				{
					// If the third token signifies concentration
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						// Construct the Duration with the u16 and the concentration bool set to true
						Ok(Self::Hours(num, true))
					}
					// If the third token is anything else, return an error
					else { Err(error) }
				}
				// If there is no third token, construct the Duration with the u16
				else { Ok(Self::Hours(num, false)) }
			},
			"days" =>
			{
				// If there isn't a second token, return an error
				if tokens.len() < 2 { return Err(error); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err(error)
				};
				// If there's a third token
				if tokens.len() > 2
				{
					// If the third token signifies concentration
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						// Construct the Duration with the u16 and the concentration bool set to true
						Ok(Self::Days(num, true))
					}
					// If the third token is anything else, return an error
					else { Err(error) }
				}
				// If there is no third token, construct the Duration with the u16
				else { Ok(Self::Days(num, false)) }
			},
			"weeks" =>
			{
				// If there isn't a second token, return an error
				if tokens.len() < 2 { return Err(error); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err(error)
				};
				// If there's a third token
				if tokens.len() > 2
				{
					// If the third token signifies concentration
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						// Construct the Duration with the u16 and the concentration bool set to true
						Ok(Self::Weeks(num, true))
					}
					// If the third token is anything else, return an error
					else { Err(error) }
				}
				// If there is no third token, construct the Duration with the u16
				else { Ok(Self::Weeks(num, false)) }
			},
			"months" =>
			{
				// If there isn't a second token, return an error
				if tokens.len() < 2 { return Err(error); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err(error)
				};
				// If there's a third token
				if tokens.len() > 2
				{
					// If the third token signifies concentration
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						// Construct the Duration with the u16 and the concentration bool set to true
						Ok(Self::Months(num, true))
					}
					// If the third token is anything else, return an error
					else { Err(error) }
				}
				// If there is no third token, construct the Duration with the u16
				else { Ok(Self::Months(num, false)) }
			},
			"years" =>
			{
				// If there isn't a second token, return an error
				if tokens.len() < 2 { return Err(error); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err(error)
				};
				// If there's a third token
				if tokens.len() > 2
				{
					// If the third token signifies concentration
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						// Construct the Duration with the u16 and the concentration bool set to true
						Ok(Self::Years(num, true))
					}
					// If the third token is anything else, return an error
					else { Err(error) }
				}
				// If there is no third token, construct the Duration with the u16
				else { Ok(Self::Years(num, false)) }
			},
			"dispelledortriggered" =>
			{
				// If there's a second token
				if tokens.len() > 1
				{
					// If the second token signifies concentration
					if tokens[1].to_lowercase().as_str() == "concentration"
					{
						// Construct the Duration with the concentration bool set to true
						Ok(Self::DispelledOrTriggered(true))
					}
					// If the second token is anything else
					else
					{
						// Return an error
						Err(error)
					}
				}
				// If there is not second token, construct the Duration with the concentration bool set to false
				else { Ok(Self::DispelledOrTriggered(false)) }
			},
			"untildispelled" =>
			{
				// If there's a second token
				if tokens.len() > 1
				{
					// If the second token signifies concentration
					if tokens[1].to_lowercase().as_str() == "concentration"
					{
						// Construct the Duration with the concentration bool set to true
						Ok(Self::UntilDispelled(true))
					}
					// If the second token is anything else
					else
					{
						// Return an error
						Err(error)
					}
				}
				// If there is not second token, construct the Duration with the concentration bool set to false
				else { Ok(Self::UntilDispelled(false)) }
			},
			"permanent" =>
			{
				Ok(Self::Permanent)
			},
			"special" =>
			{
				// If there's a second token
				if tokens.len() > 1
				{
					// If the second token signifies concentration
					if tokens[1].to_lowercase().as_str() == "concentration"
					{
						// Construct the Duration with the concentration bool set to true
						Ok(Self::Special(true))
					}
					// If the second token is anything else
					else
					{
						// Return an error
						Err(error)
					}
				}
				// If there is not second token, construct the Duration with the concentration bool set to false
				else { Ok(Self::Special(false)) }
			},
			// If the first token isn't recognized as a type of Duration, return an error
			_ => Err(error)
		}
	}
}

// Allows Durations to be constructed from strings
impl TryFrom<&str> for Duration
{
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		match Self::from_spell_file_string(value, "NOT A FILE", "NO FIELD")
		{
			Ok(duration) => Ok(duration),
			Err(_) => Err("Invalid Duration String.")
		}
	}
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

// Gets text within quotes as a field from a spell file
fn get_text_field(text: &str, file_name: &str, field_prefix: &str) -> Result<String, Box<SpellFileError>>
{
	// Error to return in case the text can't be parsed
	let error = SpellFileError::get_box(false, file_name, format!("{} {}", field_prefix, text).as_str());
	// If the field value doesn't start with a quote
	if !text.starts_with('"')
	{
		// Return an error
		return Err(error);
	}

	// Get the first line of the text field if it does start with a quote
	let desc = &text[1..];
	// If the first line ends with a quote
	if desc.ends_with('"')
	{
		// Return the text inside that quote
		Ok(desc[..desc.len()-1].to_string())
	}
	else
	{
		Err(error)
	}
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

// Converts a string to a boolean value, return an empty error if it fails
fn str_to_bool(s: &str) -> Result<bool, ()>
{
	match s.to_lowercase().as_str()
	{
		"true" => Ok(true),
		"false" => Ok(false),
		_ => Err(())
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
	/// Optional text that describes the benefits a spell gains from being upcast
	/// (being cast at a level higher than its base level).
	pub upcast_description: Option<String>,
	/// Any tables that the spell might have in its description
	pub tables: Vec<Vec<Vec<String>>>
}

impl Spell
{
	/// Constructs a spell object from a json file.
	///
	/// # Parameters
	/// - `file_path` The path to the json file to create the spell from.
	///
	/// # Output
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
	/// - `file_path` The file path to save the spell to.
	/// - `compress` True to put all the data onto one line, false to make the file more human readable.
	///
	/// # Output
	/// - `Ok` Nothing if there were no errors.
	/// - `Err` Any errors that occurred.
	pub fn to_json_file(&self, file_path: &str, compress: bool) -> Result<(), Box<dyn error::Error>>
	{
		let file = fs::File::create(file_path)?;
		if compress { to_writer(file, self)?; }
		else { to_writer_pretty(file, self)?; }
		Ok(())
	}

	/// Constructs a spell object from a spell file.
	///
	/// # Parameters
	/// - `file_path` The path to the spell file to create a spell from.
	///
	/// # Output
	/// - `Ok` A Spell object.
	/// - `Err` Any errors that occurred.
	pub fn from_file(file_path: &str) -> Result<Self, Box<dyn error::Error>>
	{
		// Reads the file
		let file_contents = fs::read_to_string(file_path)?;
		// Separates the file into lines
		let lines: Vec<_> = file_contents.lines().collect();

		// All of the variables that will be used to construct the spell
		// Options because if required fields are None after the file is done being processed, throw an error
		// If a non-required field is None after the file is done being processed, set it to a default value
		let mut name: Option<String> = None;
		let mut level: Option<SpellField<Level>> = None;
		let mut school: Option<SpellField<MagicSchool>> = None;
		let mut is_ritual: Option<bool> = None;
		let mut casting_time: Option<SpellField<CastingTime>> = None;
		let mut range: Option<SpellField<Range>> = None;
		let mut has_v_component: Option<bool> = None;
		let mut has_s_component: Option<bool> = None;
		let mut m_components: Option<String> = None;
		let mut duration: Option<SpellField<Duration>> = None;
		let mut description: Option<String> = None;
		let mut upcast_description: Option<String> = None;

		// Index that keeps track of which line is currently being processed
		let mut line_index = 0;
		// Loop through each line in the file
		while line_index < lines.len()
		{
			// Split the line into tokens
			let tokens: Vec<_> = lines[line_index].split_whitespace().collect();
			// If there are no tokens in the line, skip it
			if tokens.len() < 1 { continue; }
			// Figure out what kind of field this line is based on the first token
			match tokens[0].to_lowercase().as_str()
			{
				// Name field
				"name:" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// Just use up the rest of the line as the name
						name = Some(tokens[1..].join(" "));
					}
				},
				// Level field
				"level:" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// Try to convert the value for this field into a level object
						let result = SpellField::<Level>::from_spell_file_string(tokens[1..].join(" ").as_str(),
						file_path, tokens[0]);
						// Assign the level value if parsing succeeded, return error if not
						level = match result
						{
							Ok(l) => Some(l),
							Err(e) => return Err(e)
						};
					}
				},
				// School field
				"school:" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// Try to convert the value for this field into a MagicSchool object
						let result = SpellField::<MagicSchool>::from_spell_file_string(tokens[1..].join(" ").as_str(), file_path,
							tokens[0]);
						// Assign the school value if parsing succeeded, return error if not
						school = match result
						{
							Ok(s) => Some(s),
							Err(e) => return Err(e)
						};
					}
				},
				// Ritual field
				"is_ritual:" | "is_ritual" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// Try to convert that value into a boolean
						is_ritual = match str_to_bool(tokens[1..].join(" ").as_str())
						{
							Ok(b) => Some(b),
							Err(_) => return Err(SpellFileError::get_box(false, file_path, lines[line_index]))
						}
					}
					// If no value was given, assume the field name alone means true
					else { is_ritual = Some(true); }
				},
				// Casting time field
				"casting_time:" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// Try to convert the value for this field into a SpellField<CastingTime> object
						let result = SpellField::<CastingTime>::from_spell_file_string(tokens[1..].join(" ").as_str(),
							file_path, tokens[0]);
						// Assign the casting_time value if parsing succeeded, return error if not
						casting_time = match result
						{
							Ok(t) => Some(t),
							Err(e) => return Err(e)
						}
					}
				},
				// Range field
				"range:" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// Try to convert the value for this field into a Range object
						let result = SpellField::<Range>::from_spell_file_string(tokens[1..].join(" ").as_str(),
							file_path, tokens[0]);
						// Assign the range value if parsing succeeded, return error if not
						range = match result
						{
							Ok(r) => Some(r),
							Err(e) => return Err(e)
						}
					}
				},
				// V component field
				"has_v_component:" | "has_v_component" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// Try to convert that value into a boolean
						has_v_component = match str_to_bool(tokens[1..].join(" ").as_str())
						{
							Ok(b) => Some(b),
							Err(_) => return Err(SpellFileError::get_box(false, file_path, lines[line_index]))
						}
					}
					// If no value was given, assume the field name alone means true
					else { has_v_component = Some(true); }
				},
				// S component field
				"has_s_component:" | "has_s_component" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// Try to convert that value into a boolean
						has_s_component = match str_to_bool(tokens[1..].join(" ").as_str())
						{
							Ok(b) => Some(b),
							Err(_) => return Err(SpellFileError::get_box(false, file_path, lines[line_index]))
						}
					}
					// If no value was given, assume the field name alone means true
					else { has_s_component = Some(true); }
				},
				// M component field
				"m_components:" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// If the value of this field wasn't entered as "None"
						if tokens[1].to_lowercase().as_str() != "none"
						{
							// Try to collect value of this field as some text
							let text_result = Self::get_text_field(&tokens, &lines, &mut line_index, file_path)?;
							m_components = Some(text_result);
						}
					}
				},
				// Duration field
				"duration:" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// Try to convert the value for this field into a Duration object
						let result = SpellField::<Duration>::from_spell_file_string(tokens[1..].join(" ").as_str(),
							file_path, tokens[0]);
						// Assign the duration value if parsing succeeded, return error if not
						duration = match result
						{
							Ok(d) => Some(d),
							Err(e) => return Err(e)
						}
					}
				},
				// Description field
				"description:" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// Try to collect value of this field as some text
						let description_result = Self::get_text_field(&tokens, &lines, &mut line_index, file_path)?;
						description = Some(description_result);
					}
				},
				"upcast_description:" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// If the value of this field wasn't entered as "None"
						if tokens[1].to_lowercase().as_str() != "none"
						{
							// Try to collect value of this field as some text
							let description_result = Self::get_text_field(&tokens, &lines, &mut line_index, file_path)?;
							upcast_description = Some(description_result);
						}
					}
				},
				_ => return Err(SpellFileError::get_box(false, file_path, tokens[0]))
			}

			line_index += 1;
		}

		// Convert spell fields from options to their own values
		// Required fields return an error if None
		// Optional fields are set to a default value if None

		// Name field (required)
		let name = match name
		{
			Some(s) => s,
			None => return Err(SpellFileError::get_box(true, file_path, "name"))
		};
		// Level field (required)
		let level = match level
		{
			Some(s) => s,
			None => return Err(SpellFileError::get_box(true, file_path, "level"))
		};
		// School field (required)
		let school = match school
		{
			Some(s) => s,
			None => return Err(SpellFileError::get_box(true, file_path, "school"))
		};
		// Ritual field (optional)
		let is_ritual = match is_ritual
		{
			Some(s) => s,
			None => false
		};
		// Casting time field (required)
		let casting_time = match casting_time
		{
			Some(s) => s,
			None => return Err(SpellFileError::get_box(true, file_path, "casting_time"))
		};
		// Range field (required)
		let range = match range
		{
			Some(s) => s,
			None => return Err(SpellFileError::get_box(true, file_path, "range"))
		};
		// V component field (optional)
		let has_v_component = match has_v_component
		{
			Some(s) => s,
			None => false
		};
		// S component field (optional)
		let has_s_component = match has_s_component
		{
			Some(s) => s,
			None => false
		};
		// Duration field (required)
		let duration = match duration
		{
			Some(s) => s,
			None => return Err(SpellFileError::get_box(true, file_path, "duration"))
		};
		// Description field (required)
		let description = match description
		{
			Some(s) => s,
			None => return Err(SpellFileError::get_box(true, file_path, "description"))
		};

		// Create and return the spell object
		Ok(Spell
		{
			name: name,
			level: level,
			school: school,
			is_ritual: is_ritual,
			casting_time: casting_time,
			range: range,
			has_v_component: has_v_component,
			has_s_component: has_s_component,
			m_components: m_components,
			duration: duration,
			description: description,
			upcast_description: upcast_description,
			tables: Vec::new()
		})
	}

	/// Saves a spell to a file.
	///
	/// # Parameters
	/// - `file_path` The file path (including the file name) to save the spell to.
	/// - `compress` Whether or not to write some of the optional fields to the the spell file.
	/// Compressing can save file space, while not compressing can arguably make the spell file easier to read.
	///
	/// # Output
	/// - `Ok` Nothing if there were no errors.
	/// - `Err` Any errors that occurred.
	pub fn to_file(&self, file_path: &str, compress: bool) -> Result<(), Box<dyn error::Error>>
	{
		// Add the spell's name to the data to write
		let mut spell_data = format!("name: {}\n", self.name);
		// Add the spell's level
		spell_data = format!("{}level: {}\n", spell_data, self.level.to_spell_file_string());
		// Add the spell's school
		spell_data = format!("{}school: {}\n", spell_data, self.school.to_spell_file_string());
		// If this is supposed to be a compressed file
		if compress
		{
			// If this spell is a ritual
			if self.is_ritual
			{
				// Add the is_ritual field
				spell_data = format!("{}is_ritual\n", spell_data);
			}
		}
		// If this is not supposed to be a compressed file
		else
		{
			// Add the full form of the is_ritual field
			spell_data = format!("{}is_ritual: {}\n", spell_data, self.is_ritual);
		}
		// Add the spell's casting time
		spell_data = format!("{}casting_time: {}\n", spell_data, self.casting_time.to_spell_file_string());
		// Add the spell's range
		spell_data = format!("{}range: {}\n", spell_data, self.range.to_spell_file_string());
		// If this is supposed to be a compressed file
		if compress
		{
			// If the spell has a v component
			if self.has_v_component
			{
				// Add the has_v_component field
				spell_data = format!("{}has_v_component\n", spell_data);
			}
			// If the spell has an s component
			if self.has_s_component
			{
				// Add the has_s_component field
				spell_data = format!("{}has_s_component\n", spell_data);
			}
			// If the spell has any m components
			if let Some(c) = &self.m_components
			{
				// Add the spell's m components
				spell_data = format!("{}m_components: \"{}\"\n", spell_data, Self::treat_text_field(&c));
			}
		}
		// If this is not supposed to be a compressed file
		else
		{
			// Add the full form of the has_v_component and has_s_component fields
			spell_data = format!("{}has_v_component: {}\n", spell_data, self.has_v_component);
			spell_data = format!("{}has_s_component: {}\n", spell_data, self.has_s_component);
			// If the spell has any m components
			if let Some(c) = &self.m_components
			{
				// Add the spell's m components
				spell_data = format!("{}m_components: \"{}\"\n", spell_data, Self::treat_text_field(&c));
			}
			// If the spell has no m components
			else
			{
				// Add the m components field with the value "None"
				spell_data = format!("{}m_components: none\n", spell_data);
			}
		}
		// Add the spell's duration
		spell_data = format!("{}duration: {}\n", spell_data, self.duration.to_spell_file_string());
		// Add the spell's description
		spell_data = format!("{}description: \"{}\"\n", spell_data, Self::treat_text_field(&(self.description)));
		// If this is supposed to be a compressed file
		if compress
		{
			// If this spell has an upcast description
			if let Some(d) = &self.upcast_description
			{
				// Add the spell's upcast description
				spell_data = format!("{}upcast_description: \"{}\"\n", spell_data, Self::treat_text_field(&d));
			}
		}
		// if this is not supposed to be a compressed file
		else
		{
			// If this spell has an upcast description
			if let Some(d) = &self.upcast_description
			{
				// Add the spell's upcast description
				spell_data = format!("{}upcast_description: \"{}\"\n", spell_data, Self::treat_text_field(&d));
			}
			// If the spell has no upcast description
			else
			{
				// Add the upcast description field with the value "None"
				spell_data = format!("{}upcast_description: none\n", spell_data);
			}
		}

		// Create / Open / Truncate file
		let mut spell_file = fs::File::create(file_path)?;

		// Write all of the spell's data to the file
		spell_file.write(spell_data.as_bytes())?;

		// Return Ok if no errors occured
		Ok(())
	}

	// Gets text within quotes as a field from a spell file
	fn get_text_field(current_tokens: &Vec<&str>, lines: &Vec<&str>, line_index: &mut usize, file_name: &str)
	-> Result<String, Box<dyn error::Error>>
	{
		// If the field value doesn't start with a quote
		if !current_tokens[1].starts_with('"')
		{
			// Return an error
			return Err(SpellFileError::get_box(false, file_name, lines[*line_index]));
		}

		// Get the first line of the text field if it does start with a quote
		let mut desc = current_tokens[1..].join(" ");
		// If the first line ends with a quote
		if desc.ends_with('"')
		{
			// If the line ends with an escaped backslash and an escaped quote
			if desc.ends_with("\\\\\\\"")
			{
				// Remove two of the backslashes so the lines just ends with \"
				desc = format!("{}\"", &desc[..desc.len()-3])
			}
			// If the lines ends with an escaped backslash but an unescaped quote
			else if desc.ends_with("\\\\\"")
			{
				// Remove one of the backslashes and the quote and return the line
				return Ok(format!("{}\\", &desc[..desc.len()-2]));
			}
			// If the line ends with an escaped quote
			else if desc.ends_with("\\\"")
			{
				// Remove the escape backslash
				desc = format!("{}\"", &desc[..desc.len()-2]);
			}
			// If it's an unescaped quote, return the line
			else { return Ok(desc[1..desc.len()-1].to_string()); }
		}
		// Go to the next line
		*line_index += 1;

		// Loop until a line that ends with a quote is reached
		loop
		{
			// If there are no lines left and an end quote still hasn't been reached
			if *line_index >= lines.len()
			{
				// Return an error
				return Err(SpellFileError::get_box(false, file_name, &desc));
			}
			// Get the next line
			let new_line = lines[*line_index].split_whitespace().collect::<Vec<_>>().join(" ");
			// Combine the new line with the rest of the text
			desc = format!("{}\n{}", desc, new_line);
			// If the new line ends with a quote
			if desc.ends_with('"')
			{
				// If the line ends with an escaped backslash and an escaped quote
				if desc.ends_with("\\\\\\\"")
				{
					// Remove two of the backslashes so the lines just ends with \"
					desc = format!("{}\"", &desc[..desc.len()-3])
				}
				// If the lines ends with an escaped backslash but an unescaped quote
				else if desc.ends_with("\\\\\"")
				{
					// Remove one of the backslashes and the quote and return the line
					break;
				}
				// If the line ends with an escaped quote
				else if desc.ends_with("\\\"")
				{
					// Remove the escape backslash
					desc = format!("{}\"", &desc[..desc.len()-2]);
				}
				// If it's an unescaped quote, return the line
				else { break; }
			}
			// Go to next line
			*line_index += 1;
		}
		// Return the text without the start and end quotes
		Ok(desc[1..desc.len()-1].to_string())
	}

	// Treats a text field to prepare it for being stored to a spell file
	fn treat_text_field(text: &String) -> String
	{
		// The text that will be returned
		let mut treated_text = String::new();
		// Split the text field into lines
		let lines = text.split('\n');
		// Loop through each line in the text field
		for line in lines
		{
			// Add the line to the treated text

			// If the line ends with a quote
			treated_text += if line.ends_with('"')
			{
				// If the line ends with a backslash and a quote
				if line.ends_with("\\\"")
				{
					// Escape both the backslash and the quote with a backslash for each
					format!("\n{}\\\\\\\"", &line[..line.len()-2])
				}
				else
				{
					// Escape the quote with a backslash
					format!("\n{}\\\"", &line[..line.len()-1])
				}
			}
			else
			{
				// Just add the line to the text the way it is
				format!("\n{}", line)
			}.as_str();
		}
		// If there are any characters in the text
		if treated_text.len() > 0
		{
			// Remove the first character to get rid of the first newline character
			treated_text = treated_text[1..].to_string();
		}
		// Return the treated text
		treated_text
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
		let mut text = match &self.level
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
		// If the spell is a ritual
		if self.is_ritual
		{
			// Add that information to the end of the string
			text += " (ritual)";
		}
		// Return the string
		text
	}
}

// Used for when there is an error while reading a spell file
#[derive(Clone, Debug)]
enum SpellFileError
{
	// Type of error for when a required field is missing
	MissingField
	{
		// Name of the file that caused the error
		file_name: String,
		// The field that caused the error
		field: String
	},
	// Type of error for when a field has invalid syntax
	InvalidField
	{
		// Name of the file that caused the error
		file_name: String,
		// The field that caused the error
		field: String
	}
}

impl SpellFileError
{
	// Creates a SpellFileError and returns it in a box
	pub fn get_box(missing_type: bool, file_name: &str, field: &str) -> Box<SpellFileError>
	{
		match missing_type
		{
			// If this is a missing field error
			true => Box::new(SpellFileError::MissingField
			{
				file_name: file_name.to_string(),
				field: field.to_string()
			}),
			// If this is an invalid field error
			false => Box::new(SpellFileError::InvalidField
			{
				file_name: file_name.to_string(),
				field: field.to_string()
			})
		}
	}
}

// Makes it so SpellFileErrors can be displayed
impl fmt::Display for SpellFileError
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let s = match self
		{
			Self::MissingField {file_name, field} => format!("Missing field in {}: {}", file_name, field),
			Self::InvalidField {file_name, field} => format!("Invalid field in {}: {}", file_name, field)
		};
		write!(f, "{}", s)
	}
}

// Makes it so SpellFileErrors are considered errors
impl error::Error for SpellFileError {}
