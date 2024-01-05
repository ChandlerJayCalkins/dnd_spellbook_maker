/*
* All object types needed for defining spells
*/

use std::fmt;

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
#[derive(Clone, Copy, Debug)]
pub enum CastingTime<'a>
{
	Seconds(u16),
	// u16 is number of actions a spell takes to cast
	Actions(u16),
	BonusAction,
	// &str is the circumstance in which the reaction can be triggered
	// Ex: "you or a creature within 60 feet of you falls" or "you see a creature within 60 feet of you casting a spell"
	// Note: whatever you put for this, it will come after the string "1 reaction, which you take when" on the spell page
	Reaction(&'a str),
	Minutes(u16),
	Hours(u16),
	Days(u16),
	Weeks(u16),
	Months(u16),
	Years(u16)
}

// Converts casting times into strings
impl<'a> fmt::Display for CastingTime<'a>
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
			Self::Years(t) => get_amount_string(*t, "year")
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

// Data containing all of the information about a spell
#[derive(Clone, Copy, Debug)]
pub struct Spell<'a>
{
	pub name: &'a str,
	pub level: Level,
	pub school: MagicSchool,
	// Whether or not the spell can be casted as a ritual
	pub is_ritual: bool,
	pub casting_time: CastingTime<'a>,
	pub range: Range,
	// Whether or not the spell requires a verbal component to be cast
	pub has_v_component: bool,
	// Whether or not the spell requires a somantic component to be cast
	pub has_s_component: bool,
	// Optional text that lists all of the material components a spell might need to be cast
	pub m_components: Option<&'a str>,
	pub duration: Duration,
	// Text that describes the effects of the spell
	pub description: &'a str,
	// Optional text that describes the benefits a spell gains from being upcast
	// (being cast at a level higher than its base level)
	pub upcast_description: Option<&'a str>
}

impl<'a> Spell<'a>
{
	// Gets a string of the required components for a spell
	// Ex: "V, S, M (a bit of sulfur and some wood bark)", "V, S", "V, M (a piece of hair)"
	pub fn get_component_string(&self) -> String
	{
		let mut component_string = String::from("");
		if self.has_v_component
		{
			component_string += "V";
		}
		if self.has_s_component
		{
			if component_string.len() > 0
			{
				component_string += ", ";
			}
			component_string += "S";
		}
		if let Some(m) = self.m_components
		{
			if component_string.len() > 0
			{
				component_string += ", ";
			}
			component_string += format!("M ({})", m).as_str();
		}
		component_string
	}
}
