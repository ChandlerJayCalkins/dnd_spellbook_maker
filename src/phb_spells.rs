use crate::spells::*;

pub static ACID_SPLASH: Spell = Spell
{
	name: "Acid Splash",
	level: Level::Cantrip,
	school: MagicSchool::Conjuration,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Feet(60),
	has_v_component: true,
	has_s_component: true,
	m_components: None,
	duration: Duration::Instant,
	description: "Vou hurl a bubble of acid. Choose one creature within range, or choose two creatures within range that are within 5 feet of each other. A target must succeed on a Dexterity saving throw or take 1d6 acid damage.
	This spell's damage increases by 1d6 when you reach 5th level (2d6), 11th level (3d6), and 17th level (4d6).",
	upcast_description: None
};

pub static AID: Spell = Spell
{
	name: "Aid",
	level: Level::Level2,
	school: MagicSchool::Abjuration,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Feet(30),
	has_v_component: true,
	has_s_component: true,
	m_components: Some("a tiny strip of white cloth"),
	duration: Duration::Hours(8, false),
	description: "Your spell bolsters your allies with toughness and resolve. Choose up to three creatures within range. Each target's hit point maximum and current hit points increase by 5 for the duration.",
	upcast_description: Some("When you cast this spell using a spell slot of 3rd level or higher, a target's hit points increase by an additional 5 for each slot level above 2nd.")
};

pub static ALARM: Spell = Spell
{
	name: "Alarm",
	level: Level::Level1,
	school: MagicSchool::Abjuration,
	is_ritual: true,
	casting_time: CastingTime::Minutes(1),
	range: Range::Feet(30),
	has_v_component: true,
	has_s_component: true,
	m_components: Some("a tiny bell and a piece of fine silver wire"),
	duration: Duration::Hours(8, false),
	description: "You set an alarm against unwanted intrusion. Choose a door, a window, or an area within range that is no larger than a 20-foot cube. Until the spell ends, an alarm alerts you whenever a Tiny or larger creature touches or enters the warded area. When you cast the spell, you can designate creatures that won't set off the alarm. You also choose whether the alarm is mental or audible.
	A mental alarm alerts you with a ping in your mind if you are within 1 mile of the warded area. This ping awakens you if you are sleeping.
	An audible alarm produces the sound of a hand bell for 10 seconds within 60 feet.",
	upcast_description: None
};

pub static FIREBALL: Spell = Spell
{
	name: "Fireball",
	level: Level::Level3,
	school: MagicSchool::Evocation,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Feet(150),
	has_v_component: true,
	has_s_component: true,
	m_components: Some("a tiny ball of bat guano and sulfur"),
	duration: Duration::Instant,
	description: "A bright streak flashes from your pointing finger to a point you choose within range and then blossoms with a low roar into an explosion of flame. Each creature in a 20-foot-radius sphere centered on that point must make a Dexterity saving throw. A target takes 8d6 fire damage on a failed save, or half as much damage on a successful one.
	The fire spreads around forners. It ignites flammable objects in the area that aren't being worn or carried.",
	upcast_description: Some("When you cast this spell using a spell slot of 4th level or higher, the damage increases by 1d6 for eaeh slot level above 3rd.")
};

pub static FIRE_BOLT: Spell = Spell
{
	name: "Fire Bolt",
	level: Level::Cantrip,
	school: MagicSchool::Evocation,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Feet(120),
	has_v_component: true,
	has_s_component: true,
	m_components: None,
	duration: Duration::Instant,
	description: "You hurl a mote of fire at a creature or object within range. Make a ranged spell attack against the target. On a hit, the target takes 1d10 fire damage. A flammable object hit by this spell ignites if it isn't being worn or carried.
	This spell's damage increases by 1d10 when you reach 5th level (2d10), 11th level (3d10), and 17th level (4d10).",
	upcast_description: None
};
