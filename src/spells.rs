// The level of a spell
// 0 is a cantrip, max level is 9
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
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
impl ToString for MagicSchool
{
	fn to_string(&self) -> String
	{
		match self
		{
			Self::Abjuration => String::from("Abjuration"),
			Self::Conjuration => String::from("Conjuration"),
			Self::Divination => String::from("Divination"),
			Self::Enchantment => String::from("Enchantment"),
			Self::Evocation => String::from("Evocation"),
			Self::Illusion => String::from("Illusion"),
			Self::Necromancy => String::from("Necromancy"),
			Self::Transmutation => String::from("Transmutation")
		}
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

// Converts a time and a unit of time into a string (Ex: '2' and 'minute' becomes '2 minutes')
fn get_time_string(time: u16, unit: &str) -> String
{
	if time == 1
	{
		return format!("1 {}", unit);
	}
	else
	{
		return format!("{} {}s", time, unit);
	}
}

// Converts casting times into strings
impl ToString for CastingTime<'_>
{
	fn to_string(&self) -> String
	{
		match self
		{
			Self::Seconds(t) => get_time_string(*t, "second"),
			Self::Actions(t) => get_time_string(*t, "action"),
			Self::BonusAction => String::from("1 bonus action"),
			Self::Reaction(e) => format!("1 reaction, which you take when {}", e),
			Self::Minutes(t) => get_time_string(*t, "minute"),
			Self::Hours(t) => get_time_string(*t, "hour"),
			Self::Days(t) => get_time_string(*t, "day"),
			Self::Weeks(t) => get_time_string(*t, "week"),
			Self::Months(t) => get_time_string(*t, "month"),
			Self::Years(t) => get_time_string(*t, "year")
		}
	}
}

// Area of Effect
// The shape of the area in which targets of a spell need to be in to be affected by the spell
#[derive(Clone, Copy, Debug)]
pub enum AOE
{
	// No AOE
	None,
	// u16 tuple defines length and width of line (respectively)
	Line(u16, u16),
	// u16 defines length / height and diameter of cone
	Cone(u16),
	// u16 defines the length of the edges of the cube
	Cube(u16),
	// u16 defines radius of sphere
	Sphere(u16),
	// u16 tuple defines radius and height of cylinder (respectively)
	Cylinder(u16, u16)
}

// The farthest distance a target can be from the caster of a spell
#[derive(Clone, Copy, Debug)]
pub enum Range
{
	Yourself(AOE),
	Touch,
	Feet(u16),
	Miles(u16)
}

// How long a spell's effect lasts
#[derive(Clone, Copy, Debug)]
pub enum Duration
{
	Instant,
	Seconds(u16),
	Rounds(u16),
	Minutes(u16),
	Hours(u16),
	Days(u16),
	Weeks(u16),
	Months(u16),
	Years(u16),
	UntilDispelled,
	Permanent
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
	pub has_verbal_component: bool,
	// Whether or not the spell requires a somantic component to be cast
	pub has_somantic_component: bool,
	// Optional text that lists all of the material components a spell might need to be cast
	pub material_components: Option<&'a str>,
	pub duration: Duration,
	// Whether or not the effect of this spell requires concentration
	pub requires_concentration: bool,
	// Text that describes the effects of the spell
	pub description: &'a str,
	// Optional text that describes the benefits a spell gains from being upcast
	// (being cast at a level higher than its base level)
	pub upcast_description: Option<&'a str>
}
