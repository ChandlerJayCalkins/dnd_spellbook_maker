/*
* All object types needed for defining spells
*/

use std::fmt;
use std::fs;
use std::error;

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

// Spell Level methods
impl Level
{
	// Converts spell levels into integers (u8)
	pub fn as_num(&self) -> u8
	{
		match self
		{
			Self::Cantrip => 0,
			Self::Level1 => 1,
			Self::Level2 => 2,
			Self::Level3 => 3,
			Self::Level4 => 4,
			Self::Level5 => 5,
			Self::Level6 => 6,
			Self::Level7 => 7,
			Self::Level8 => 8,
			Self::Level9 => 9
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
		level.as_num()
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

// Allows strings to be converted into casting times
impl TryFrom<&str> for CastingTime
{
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		let tokens: Vec<_> = value.split_whitespace().collect();
		if tokens.len() > 0
		{
			match tokens[0].to_lowercase().as_str()
			{
				"seconds" | "second" =>
				{
					if tokens.len() > 1
					{
						match tokens[1].parse::<u16>()
						{
							Ok(n) => return Ok(Self::Seconds(n)),
							Err(_) => return Err("Invalid CastingTime string.")
						}
					}
					else { return Err("Invalid CastingTime string."); }
				},
				"actions" | "action" =>
				{
					if tokens.len() > 1
					{
						match tokens[1].parse::<u16>()
						{
							Ok(n) => return Ok(Self::Actions(n)),
							Err(_) => return Err("Invalid CastingTime string.")
						}
					}
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
				"minutes" | "minute" =>
				{
					if tokens.len() > 1
					{
						match tokens[1].parse::<u16>()
						{
							Ok(n) => return Ok(Self::Minutes(n)),
							Err(_) => return Err("Invalid CastingTime string.")
						}
					}
					else { return Err("Invalid CastingTime string."); }
				},
				"hours" | "hour" =>
				{
					if tokens.len() > 1
					{
						match tokens[1].parse::<u16>()
						{
							Ok(n) => return Ok(Self::Hours(n)),
							Err(_) => return Err("Invalid CastingTime string.")
						}
					}
					else { return Err("Invalid CastingTime string."); }
				},
				"days" | "day" =>
				{
					if tokens.len() > 1
					{
						match tokens[1].parse::<u16>()
						{
							Ok(n) => return Ok(Self::Days(n)),
							Err(_) => return Err("Invalid CastingTime string.")
						}
					}
					else { return Err("Invalid CastingTime string."); }
				},
				"weeks" | "week" =>
				{
					if tokens.len() > 1
					{
						match tokens[1].parse::<u16>()
						{
							Ok(n) => return Ok(Self::Weeks(n)),
							Err(_) => return Err("Invalid CastingTime string.")
						}
					}
					else { return Err("Invalid CastingTime string."); }
				},
				"months" | "month" =>
				{
					if tokens.len() > 1
					{
						match tokens[1].parse::<u16>()
						{
							Ok(n) => return Ok(Self::Months(n)),
							Err(_) => return Err("Invalid CastingTime string.")
						}
					}
					else { return Err("Invalid CastingTime string."); }
				},
				"years" | "year" =>
				{
					if tokens.len() > 1
					{
						match tokens[1].parse::<u16>()
						{
							Ok(n) => return Ok(Self::Years(n)),
							Err(_) => return Err("Invalid CastingTime string.")
						}
					}
					else { return Err("Invalid CastingTime string."); }
				},
				"special" =>
				{
					return Ok(Self::Special);
				},
				_ => { return Err("Invalid CastingTime string."); }
			}
		}
		else { return Err("Invalid CastingTime string."); }
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

// Allows AOEs to be created from strings
impl TryFrom<&str> for AOE
{
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		let tokens: Vec<_> = value.split_whitespace().collect();
		if tokens.len() < 2 { return Err("Invalid AOE string"); }
		match tokens[0].to_lowercase().as_str()
		{
			"line" =>
			{
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid AOE string.")
				};
				Ok(Self::Line(num))
			},
			"cone" =>
			{
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid AOE string.")
				};
				Ok(Self::Cone(num))
			},
			"cube" =>
			{
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid AOE string.")
				};
				Ok(Self::Cube(num))
			},
			"sphere" =>
			{
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid AOE string.")
				};
				Ok(Self::Sphere(num))
			},
			"cylinder" =>
			{
				if tokens.len() < 3 { return Err("Invalid AOE string."); }
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
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid AOE string.")
				};
				Ok(Self::Radius(num))
			},
			_ => Err("Invalid AOE string.")
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

impl TryFrom<&str> for Range
{
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		let tokens: Vec<_> = value.split_whitespace().collect();
		if tokens.len() < 1 { return Err("Invalid Range string."); }
		match tokens[0].to_lowercase().as_str()
		{
			"self" =>
			{
				if tokens.len() < 2 { return Ok(Self::Yourself(None)); }
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
				if tokens.len() < 2 { return Err("Invalid Range string."); }
				match tokens[1].parse::<u16>()
				{
					Ok(n) => Ok(Self::Feet(n)),
					Err(_) => Err("Invalid Range string.")
				}
			},
			"miles" =>
			{
				if tokens.len() < 2 { return Err("Invalid Range string."); }
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
			_ => Err("Invalid Range string.")
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

impl TryFrom<&str> for Duration
{
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		let tokens: Vec<_> = value.split_whitespace().collect();
		if tokens.len() < 1 { return Err("Invalid Duration string."); }
		match tokens[0].to_lowercase().as_str()
		{
			"instant" =>
			{
				Ok(Self::Instant)
			},
			"seconds" | "second" =>
			{
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
				};
				if tokens.len() > 2
				{
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						Ok(Self::Seconds(num, true))
					}
					else { Err("Invalid Duration string.") }
				}
				else { Ok(Self::Seconds(num, false)) }
			},
			"rounds" | "round" =>
			{
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
				};
				if tokens.len() > 2
				{
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						Ok(Self::Rounds(num, true))
					}
					else { Err("Invalid Duration string.") }
				}
				else { Ok(Self::Rounds(num, false)) }
			},
			"minutes" | "minute" =>
			{
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
				};
				if tokens.len() > 2
				{
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						Ok(Self::Minutes(num, true))
					}
					else { Err("Invalid Duration string.") }
				}
				else { Ok(Self::Minutes(num, false)) }
			},
			"hours" | "hour" =>
			{
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
				};
				if tokens.len() > 2
				{
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						Ok(Self::Hours(num, true))
					}
					else { Err("Invalid Duration string.") }
				}
				else { Ok(Self::Hours(num, false)) }
			},
			"days" | "day" =>
			{
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
				};
				if tokens.len() > 2
				{
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						Ok(Self::Days(num, true))
					}
					else { Err("Invalid Duration string.") }
				}
				else { Ok(Self::Days(num, false)) }
			},
			"weeks" | "week" =>
			{
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
				};
				if tokens.len() > 2
				{
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						Ok(Self::Weeks(num, true))
					}
					else { Err("Invalid Duration string.") }
				}
				else { Ok(Self::Weeks(num, false)) }
			},
			"months" | "month" =>
			{
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
				};
				if tokens.len() > 2
				{
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						Ok(Self::Months(num, true))
					}
					else { Err("Invalid Duration string.") }
				}
				else { Ok(Self::Months(num, false)) }
			},
			"years" | "year" =>
			{
				if tokens.len() < 2 { return Err("Invalid Duration string."); }
				let num = match tokens[1].parse::<u16>()
				{
					Ok(n) => n,
					Err(_) => return Err("Invalid Duration string.")
				};
				if tokens.len() > 2
				{
					if tokens[2].to_lowercase().as_str() == "concentration"
					{
						Ok(Self::Years(num, true))
					}
					else { Err("Invalid Duration string.") }
				}
				else { Ok(Self::Years(num, false)) }
			},
			"dispelledortriggered" =>
			{
				if tokens.len() > 1
				{
					if tokens[1].to_lowercase().as_str() == "concentration"
					{
						Ok(Self::DispelledOrTriggered(true))
					}
					else
					{
						Err("Invalid Duration string.")
					}
				}
				else { Ok(Self::DispelledOrTriggered(false)) }
			},
			"untildispelled" =>
			{
				if tokens.len() > 1
				{
					if tokens[1].to_lowercase().as_str() == "concentration"
					{
						Ok(Self::UntilDispelled(true))
					}
					else
					{
						Err("Invalid Duration string.")
					}
				}
				else { Ok(Self::UntilDispelled(false)) }
			},
			"permanent" =>
			{
				Ok(Self::Permanent)
			},
			"special" =>
			{
				if tokens.len() > 1
				{
					if tokens[1].to_lowercase().as_str() == "concentration"
					{
						Ok(Self::Special(true))
					}
					else
					{
						Err("Invalid Duration string.")
					}
				}
				else { Ok(Self::Special(false)) }
			},
			_ => Err("Invalid Duration string.")
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

// Gets a string of an amount of something like "1 minute", "5 minutes", "1 hour", or "3 hours"
// Note: the unit should be singular, not plural because an 's' will be added to the end of it if num is anything besides 1
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
						// Try to parse the value into a u8
						let level_num = tokens[1].parse::<u8>()?;
						// Try to convert that u8 into a level
						level = Some(level_num.try_into()?);
					}
				},
				// School field
				"school:" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// Try to convert the value for this field into a MagicSchool object
						school = Some(tokens[1].try_into()?);
					}
				},
				// Ritual field
				"is_ritual:" | "is_ritual" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// Try to convert that value into a boolean
						is_ritual = match str_to_bool(tokens[1])
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
						casting_time = match tokens[1..].join(" ").as_str().try_into()
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
						range = match tokens[1..].join(" ").as_str().try_into()
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
						has_v_component = match str_to_bool(tokens[1])
						{
							Ok(b) => Some(b),
							Err(_) => return Err(SpellFileError::get_box(false, file_name, lines[line_index]))
						}
					}
					// If no value was given, assume the field name alone means true
					else { is_ritual = Some(true); }
				},
				// S component field
				"has_s_component:" | "has_s_component" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// Try to convert that value into a boolean
						has_s_component = match str_to_bool(tokens[1])
						{
							Ok(b) => Some(b),
							Err(_) => return Err(SpellFileError::get_box(false, file_name, lines[line_index]))
						}
					}
					// If no value was given, assume the field name alone means true
					else { is_ritual = Some(true); }
				},
				// M component field
				"m_components:" =>
				{
					// If a value was given for this field
					if tokens.len() > 1
					{
						// If the value of this field wasn't entered as "None"
						if tokens[1] != "None"
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
						duration = match tokens[1..].join(" ").as_str().try_into()
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
						if tokens[1] != "None"
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
		while true
		{
			// Get the next line
			let new_line: Vec<_> = lines[*line_index].split_whitespace().collect().join(" ");
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
		let mut component_string = String::from("");
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
