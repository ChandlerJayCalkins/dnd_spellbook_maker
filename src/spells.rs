/*
* All object types needed for defining spells
*/

use std::fmt;
use std::fs;
use std::io::Write;
use std::error;

// For reading and writing to spell files
trait SpellFileString: Sized
{
	type SpellFileStringError;

	// Turns an object into a string that can be written into a spell file
	fn to_spell_file_string(&self) -> String;
	// Tries to turn a string from a spell file into an object
	fn from_spell_file_string(s: &str) -> Result<Self, Self::SpellFileStringError>;
}

// The level of a spell
// 0 is a cantrip, max level is 9
#[derive(Clone, Copy, Debug, PartialEq)]
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
	type SpellFileStringError = &'static str;

	fn to_spell_file_string(&self) -> String
	{
		u8::from(*self).to_string()
	}

	fn from_spell_file_string(s: &str) -> Result<Self, Self::SpellFileStringError>
	{
		let parse_result = s.parse::<u8>();
		match parse_result
		{
			Ok(n) => n.try_into(),
			Err(_) => Err("Invalid level string.")
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
			Self::Cantrip => String::from("Cantrip"),
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

// The school of magic a spell belongs to
#[derive(Clone, Copy, Debug, PartialEq)]
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
	type SpellFileStringError = &'static str;

	fn to_spell_file_string(&self) -> String
	{
		self.to_string().to_lowercase()
	}

	fn from_spell_file_string(s: &str) -> Result<Self, Self::SpellFileStringError>
	{
		s.try_into()
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

// The amount of time it takes to cast a spell
#[derive(Clone, Debug)]
pub enum CastingTime
{
	Seconds(u16),
	// u16 is number of actions a spell takes to cast
	Actions(u16),
	BonusAction,
	// &str is the circumstance in which the reaction can be triggered
	// Ex: "you or a creature within 60 feet of you falls" or "you see a creature within 60 feet of you casting a spell"
	// Note: whatever you put for this, it will come after the string "1 reaction, which you take when" on the spell page
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
	type SpellFileStringError = &'static str;

	fn to_spell_file_string(&self) -> String
	{
		match self
		{
			Self::Seconds(t) => format!("actions {}", *t),
			Self::Actions(t) => format!("actions {}", *t),
			Self::BonusAction => String::from("bonusaction"),
			Self::Reaction(e) => format!("reaction {}", e),
			Self::Minutes(t) => format!("minutes {}", *t),
			Self::Hours(t) => format!("hours {}", *t),
			Self::Days(t) => format!("days {}", *t),
			Self::Weeks(t) => format!("weeks {}", *t),
			Self::Months(t) => format!("months {}", *t),
			Self::Years(t) => format!("years {}", *t),
			Self::Special => format!("special")
		}
	}

	fn from_spell_file_string(s: &str) -> Result<Self, Self::SpellFileStringError>
	{
		// Gets a vec of all the tokens in the string
		let tokens: Vec<_> = s.split_whitespace().collect();
		// If there aren't any tokens in the string, return an error
		if tokens.len() < 1 { return Err("Invalid CastingTime string."); }
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
						Err(_) => return Err("Invalid CastingTime string.")
					}
				}
				// If there's no second token, return an error
				else { return Err("Invalid CastingTime string."); }
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
						Err(_) => return Err("Invalid CastingTime string.")
					}
				}
				// If there's no second token, return an error
				else { return Err("Invalid CastingTime string."); }
			},
			"bonusaction" =>
			{
				return Ok(Self::BonusAction);
			},
			"reaction" =>
			{
				return Ok(Self::Reaction(tokens[1..].join(" ")));
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
						Err(_) => return Err("Invalid CastingTime string.")
					}
				}
				// If there's no second token, return an error
				else { return Err("Invalid CastingTime string."); }
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
						Err(_) => return Err("Invalid CastingTime string.")
					}
				}
				// Try to parse that second token into a u16 and use it as the value of this object
				else { return Err("Invalid CastingTime string."); }
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
						Err(_) => return Err("Invalid CastingTime string.")
					}
				}
				// Try to parse that second token into a u16 and use it as the value of this object
				else { return Err("Invalid CastingTime string."); }
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
						Err(_) => return Err("Invalid CastingTime string.")
					}
				}
				// Try to parse that second token into a u16 and use it as the value of this object
				else { return Err("Invalid CastingTime string."); }
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
						Err(_) => return Err("Invalid CastingTime string.")
					}
				}
				// Try to parse that second token into a u16 and use it as the value of this object
				else { return Err("Invalid CastingTime string."); }
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
						Err(_) => return Err("Invalid CastingTime string.")
					}
				}
				// Try to parse that second token into a u16 and use it as the value of this object
				else { return Err("Invalid CastingTime string."); }
			},
			"special" =>
			{
				return Ok(Self::Special);
			},
			// If the first token wasn't recognized as a CastingTime type, return an error
			_ => { return Err("Invalid CastingTime string."); }
		}
	}
}

// Allows strings to be converted into casting times
impl TryFrom<&str> for CastingTime
{
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		CastingTime::from_spell_file_string(value)
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
			Self::Reaction(e) => format!("1 reaction, which you take when {}", *e),
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

// Area of Effect
// The shape of the area in which targets of a spell need to be in to be affected by the spell
#[derive(Clone, Copy, Debug)]
pub enum AOE
{
	// u16 defines length of line in feet (width should be in spell description)
	Line(u16),
	// u16 defines length / height and diameter of cone in feet
	Cone(u16),
	// u16 defines the length of the edges of the cube in feet
	Cube(u16),
	// u16 defines radius of sphere in feet
	Sphere(u16),
	// u16 tuple defines radius and height of cylinder in feet (respectively)
	Cylinder(u16, u16),
	// u16 defines radius of effect in miles
	Radius(u16)
}

// Allows AOEs to be written to and read from spell files
impl SpellFileString for AOE
{
	type SpellFileStringError = &'static str;

	fn to_spell_file_string(&self) -> String
	{
		match self
		{
			Self::Line(l) => format!("line {}", *l),
			Self::Cone(l) => format!("cone {}", *l),
			Self::Cube(l) => format!("cube {}", *l),
			Self::Sphere(r) => format!("sphere {}", *r),
			Self::Cylinder(r, h) => format!("cylinder {} {}", *r, *h),
			Self::Radius(r) => format!("radius {}", *r)
		}
	}

	fn from_spell_file_string(s: &str) -> Result<Self, Self::SpellFileStringError>
	{
		// Get a vec of all the tokens in the string
		let tokens: Vec<_> = s.split_whitespace().collect();
		// If there aren't at least 2 tokens in the string, return an error
		if tokens.len() < 2 { return Err("Invalid AOE string"); }
		// Determine what type of AOE this is based on the first token
		match tokens[0].to_lowercase().as_str()
		{
			"line" =>
			{
				// Try to parse the second token and use it to construct the aoe
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid AOE string.")
				};
				Ok(Self::Line(num))
			},
			"cone" =>
			{
				// Try to parse the second token and use it to construct the aoe
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid AOE string.")
				};
				Ok(Self::Cone(num))
			},
			"cube" =>
			{
				// Try to parse the second token and use it to construct the aoe
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid AOE string.")
				};
				Ok(Self::Cube(num))
			},
			"sphere" =>
			{
				// Try to parse the second token and use it to construct the aoe
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid AOE string.")
				};
				Ok(Self::Sphere(num))
			},
			"cylinder" =>
			{
				// If there aren't at least 3 tokens for this aoe type, return an error
				if tokens.len() < 3 { return Err("Invalid AOE string."); }
				// Try to parse the second and third tokens and use them to construct the aoe
				let num1 = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid AOE string.")
				};
				let num2 = match tokens[2].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid AOE string.")
				};
				Ok(Self::Cylinder(num1, num2))
			},
			"radius" =>
			{
				// Try to parse the second token and use it to construct the aoe
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid AOE string.")
				};
				Ok(Self::Radius(num))
			},
			// If the first token wasn't recognized as an AOE type, return an error
			_ => Err("Invalid AOE string.")
		}
	}
}

// Allows AOEs to be created from strings
impl TryFrom<&str> for AOE
{
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		Self::from_spell_file_string(value)
	}
}

// Converts AOEs into strings
impl fmt::Display for AOE
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let text = match self
		{
			Self::Line(l) => format!("{}-foot line", l),
			Self::Cone(l) => format!("{}-foot cone", l),
			Self::Cube(l) => format!("{}-foot cube", l),
			Self::Sphere(r) => format!("{}-foot sphere", r),
			Self::Cylinder(r, h) => format!("{}-foot radius, {}-foot tall cylinder", r, h),
			Self::Radius(r) => format!("{}-mile radius", r)
		};
		write!(f, "{}", text)
	}
}

// The farthest distance a target can be from the caster of a spell
#[derive(Clone, Copy, Debug)]
pub enum Range
{
	Yourself(Option<AOE>),
	Touch,
	Feet(u16),
	Miles(u16),
	Special
}

// Allows ranges to be written to and read from spell files
impl SpellFileString for Range
{
	type SpellFileStringError = &'static str;

	fn to_spell_file_string(&self) -> String
	{
		match self
		{
			Self::Yourself(o) =>
			{
				let mut text = String::from("self");
				match o
				{
					Some(aoe) => format!("{} {}", text, aoe.to_spell_file_string()),
					None => text
				}
			},
			Self::Touch => String::from("touch"),
			Self::Feet(n) => format!("feet {}", *n),
			Self::Miles(n) => format!("miles {}", *n),
			Self::Special => String::from("special")
		}
	}

	fn from_spell_file_string(s: &str) -> Result<Self, Self::SpellFileStringError>
	{
		// Get a vec of all the tokens in the string
		let tokens: Vec<_> = s.split_whitespace().collect();
		// If there aren't any tokens in the string, return an error
		if tokens.len() < 1 { return Err("Invalid Range string."); }
		// Determine what kind of Range to create based on the first token
		match tokens[0].to_lowercase().as_str()
		{
			"self" =>
			{
				// If there isn't at least a second token, assume this Range takes type None
				if tokens.len() < 2 { return Ok(Self::Yourself(None)); }
				// Try to constuct an AOE from the following tokens and use that to construct this Range object
				match tokens[1..].join(" ").as_str().try_into()
				{
					Ok(aoe) => Ok(Self::Yourself(Some(aoe))),
					Err(_) => Err("Invalid Range string.")
				}
			},
			"touch" =>
			{
				Ok(Self::Touch)
			},
			"feet" =>
			{
				// If there isn't at least a second token, return an error
				if tokens.len() < 2 { return Err("Invalid Range string."); }
				// Try to parse the second token and use that to construct this Range object
				match tokens[1].parse::<u16>()
				{
					Ok(n) => Ok(Self::Feet(n)),
					Err(_) => Err("Invalid Range string.")
				}
			},
			"miles" =>
			{
				// If there isn't at least a second token, return an error
				if tokens.len() < 2 { return Err("Invalid Range string."); }
				// Try to parse the second token and use that to construct this Range object
				match tokens[1].parse::<u16>()
				{
					Ok(n) => Ok(Self::Miles(n)),
					Err(_) => Err("Invalid Range string.")
				}
			},
			"special" =>
			{
				Ok(Self::Special)
			},
			// If the first token wasn't recognized as a type of Range, return an error
			_ => Err("Invalid Range string.")
		}
	}
}

// Allows Ranges to be created from strings
impl TryFrom<&str> for Range
{
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		Self::from_spell_file_string(value)
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
			Self::Feet(r) => if *r == 1 { String::from("1 foot") } else { format!("{} feet", r) },
			Self::Miles(r) => get_amount_string(*r, "mile"),
			Self::Special => String::from("Special")
		};
		write!(f, "{}", text)
	}
}

// How long a spell's effect lasts
// u16 values are the number of units the spell can last
// Bool values are whether or not the spell requires concentration
#[derive(Clone, Copy, Debug)]
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
	type SpellFileStringError = &'static str;

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

	fn from_spell_file_string(s: &str) -> Result<Self, Self::SpellFileStringError>
	{
		// Gets a vec of all the tokens in the string
		let tokens: Vec<_> = s.split_whitespace().collect();
		// If there aren't any tokens in this string, return an error
		if tokens.len() < 1 { return Err("Invalid Duration string."); }
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
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
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
					else { Err("Invalid Duration string.") }
				}
				// If there is no third token, construct the Duration with the u16
				else { Ok(Self::Seconds(num, false)) }
			},
			"rounds" =>
			{
				// If there isn't a second token, return an error
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
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
					else { Err("Invalid Duration string.") }
				}
				// If there is no third token, construct the Duration with the u16
				else { Ok(Self::Rounds(num, false)) }
			},
			"minutes" =>
			{
				// If there isn't a second token, return an error
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
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
					else { Err("Invalid Duration string.") }
				}
				// If there is no third token, construct the Duration with the u16
				else { Ok(Self::Minutes(num, false)) }
			},
			"hours" =>
			{
				// If there isn't a second token, return an error
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
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
					else { Err("Invalid Duration string.") }
				}
				// If there is no third token, construct the Duration with the u16
				else { Ok(Self::Hours(num, false)) }
			},
			"days" =>
			{
				// If there isn't a second token, return an error
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
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
					else { Err("Invalid Duration string.") }
				}
				// If there is no third token, construct the Duration with the u16
				else { Ok(Self::Days(num, false)) }
			},
			"weeks" =>
			{
				// If there isn't a second token, return an error
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
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
					else { Err("Invalid Duration string.") }
				}
				// If there is no third token, construct the Duration with the u16
				else { Ok(Self::Weeks(num, false)) }
			},
			"months" =>
			{
				// If there isn't a second token, return an error
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
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
					else { Err("Invalid Duration string.") }
				}
				// If there is no third token, construct the Duration with the u16
				else { Ok(Self::Months(num, false)) }
			},
			"years" =>
			{
				// If there isn't a second token, return an error
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				// Try to parse the second token into a u16
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
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
					else { Err("Invalid Duration string.") }
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
						Err("Invalid Duration string.")
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
						Err("Invalid Duration string.")
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
						Err("Invalid Duration string.")
					}
				}
				// If there is not second token, construct the Duration with the concentration bool set to false
				else { Ok(Self::Special(false)) }
			},
			// If the first token isn't recognized as a type of Duration, return an error
			_ => Err("Invalid Duration string.")
		}
	}
}

// Allows Durations to be constructed from strings
impl TryFrom<&str> for Duration
{
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		Self::from_spell_file_string(value)
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

// Data containing all of the information about a spell
#[derive(Clone, Debug)]
pub struct Spell
{
	pub name: String,
	pub level: Level,
	pub school: MagicSchool,
	// Whether or not the spell can be casted as a ritual
	pub is_ritual: bool,
	pub casting_time: CastingTime,
	pub range: Range,
	// Whether or not the spell requires a verbal component to be cast
	pub has_v_component: bool,
	// Whether or not the spell requires a somantic component to be cast
	pub has_s_component: bool,
	// Optional text that lists all of the material components a spell might need to be cast
	pub m_components: Option<String>,
	pub duration: Duration,
	// Text that describes the effects of the spell
	pub description: String,
	// Optional text that describes the benefits a spell gains from being upcast
	// (being cast at a level higher than its base level)
	pub upcast_description: Option<String>
}

impl Spell
{
	// Constructs a spell object from a file
	pub fn from_file(file_name: &str) -> Result<Self, Box<dyn error::Error>>
	{
		// Reads the file
		let file_contents = fs::read_to_string(file_name)?;
		// Separates the file into lines
		let lines: Vec<_> = file_contents.lines().collect();

		// All of the variables that will be used to construct the spell
		// Options because if required fields are None after the file is done being processed, throw an error
		// If a non-required field is None after the file is done being processed, set it to a default value
		let mut name: Option<String> = None;
		let mut level: Option<Level> = None;
		let mut school: Option<MagicSchool> = None;
		let mut is_ritual: Option<bool> = None;
		let mut casting_time: Option<CastingTime> = None;
		let mut range: Option<Range> = None;
		let mut has_v_component: Option<bool> = None;
		let mut has_s_component: Option<bool> = None;
		let mut m_components: Option<String> = None;
		let mut duration: Option<Duration> = None;
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
			match tokens[0]
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
						let result = Level::from_spell_file_string(tokens[1..].join(" ").as_str());
						// Assign the level value if parsing succeeded, return error if not
						level = match result
						{
							Ok(l) => Some(l),
							Err(_) => return Err(SpellFileError::get_box(false, file_name, lines[line_index]))
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
						let result = MagicSchool::from_spell_file_string(tokens[1..].join(" ").as_str());
						// Assign the school value if parsing succeeded, return error if not
						school = match result
						{
							Ok(s) => Some(s),
							Err(_) => return Err(SpellFileError::get_box(false, file_name, lines[line_index]))
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
							Err(_) => return Err(SpellFileError::get_box(false, file_name, lines[line_index]))
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
						// Try to convert the value for this field into a CastingTime object
						let result = CastingTime::from_spell_file_string(tokens[1..].join(" ").as_str());
						// Assign the casting_time value if parsing succeeded, return error if not
						casting_time = match result
						{
							Ok(t) => Some(t),
							Err(_) => return Err(SpellFileError::get_box(false, file_name, lines[line_index]))
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
						let result = Range::from_spell_file_string(tokens[1..].join(" ").as_str());
						// Assign the range value if parsing succeeded, return error if not
						range = match result
						{
							Ok(r) => Some(r),
							Err(_) => return Err(SpellFileError::get_box(false, file_name, lines[line_index]))
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
							Err(_) => return Err(SpellFileError::get_box(false, file_name, lines[line_index]))
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
							Err(_) => return Err(SpellFileError::get_box(false, file_name, lines[line_index]))
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
							let text_result = Self::get_text_field(&tokens, &lines, &mut line_index, file_name)?;
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
						let result = Duration::from_spell_file_string(tokens[1..].join(" ").as_str());
						// Assign the duration value if parsing succeeded, return error if not
						duration = match result
						{
							Ok(d) => Some(d),
							Err(_) => return Err(SpellFileError::get_box(false, file_name, lines[line_index]))
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
						let description_result = Self::get_text_field(&tokens, &lines, &mut line_index, file_name)?;
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
							let description_result = Self::get_text_field(&tokens, &lines, &mut line_index, file_name)?;
							upcast_description = Some(description_result);
						}
					}
				},
				_ => return Err(SpellFileError::get_box(false, file_name, tokens[0]))
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
			None => return Err(SpellFileError::get_box(true, file_name, "name"))
		};
		// Level field (required)
		let level = match level
		{
			Some(s) => s,
			None => return Err(SpellFileError::get_box(true, file_name, "level"))
		};
		// School field (required)
		let school = match school
		{
			Some(s) => s,
			None => return Err(SpellFileError::get_box(true, file_name, "school"))
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
			None => return Err(SpellFileError::get_box(true, file_name, "casting_time"))
		};
		// Range field (required)
		let range = match range
		{
			Some(s) => s,
			None => return Err(SpellFileError::get_box(true, file_name, "range"))
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
			None => return Err(SpellFileError::get_box(true, file_name, "duration"))
		};
		// Description field (required)
		let description = match description
		{
			Some(s) => s,
			None => return Err(SpellFileError::get_box(true, file_name, "description"))
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
			upcast_description: upcast_description
		})
	}

	// Saves this spell to a file
	// Compress parameter is whether or not to not write some of the optional fields to the file to save space
	// Not compressing the file may make it more readable though
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
				spell_data = format!("{}m_components: \"{}\"\n", spell_data, c);
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
				spell_data = format!("{}m_components: \"{}\"\n", spell_data, c);
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
		spell_data = format!("{} description: \"{}\"\n", spell_data, self.description);
		// If this is supposed to be a compressed file
		if compress
		{
			// If this spell has an upcast description
			if let Some(d) = &self.upcast_description
			{
				// Add the spell's upcast description
				spell_data = format!("{}upcast_description: \"{}\"\n", spell_data, d);
			}
		}
		// if this is not supposed to be a compressed file
		else
		{
			// If this spell has an upcast description
			if let Some(d) = &self.upcast_description
			{
				// Add the spell's upcast description
				spell_data = format!("{}upcast_description: \"{}\"\n", spell_data, d);
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
		// If the first line ends with a quote but not an escape quote
		if desc.ends_with('"') && !desc.ends_with("\\\"")
		{
			// Return that line
			return Ok(desc[1..desc.len()-1].to_string());
		}
		// Otherwise continue to the next line
		else { *line_index += 1; }

		// Loop until a line that ends with a quote is reached
		loop
		{
			// Get the next line
			let new_line = lines[*line_index].split_whitespace().collect::<Vec<_>>().join(" ");
			// Combine the new line with the rest of the text
			desc = format!("{}\n{}", desc, new_line);
			// If the new line ends with a quote but not an escape quote, end the loop
			if desc.ends_with('"') && !desc.ends_with("\\\"") { break; }
			// Go to next line
			*line_index += 1;
			// If there are no lines left and an end quote still hasn't been reached
			if *line_index >= lines.len()
			{
				// Return an error
				return Err(SpellFileError::get_box(false, file_name, &desc));
			}
		}
		// Return the text without the start and end quotes
		Ok(desc[1..desc.len()-1].to_string())
	}

	// Gets a string of the required components for a spell
	// Ex: "V, S, M (a bit of sulfur and some wood bark)", "V, S", "V, M (a piece of hair)"
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
