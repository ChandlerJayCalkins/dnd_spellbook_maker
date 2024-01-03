use crate::spells::*;

pub static fireball: Spell = Spell
{
	name: "Fireball",
	level: Level::Level3,
	school: MagicSchool::Evocation,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Feet(150),
	has_verbal_component: true,
	has_somantic_component: true,
	material_components: Some("a tiny ball of bat guano and sulfur"),
	duration: Duration::Instant,
	description: "A bright streak flashes from your pointing finger to a point you choose within range and then blossoms with a low roar into an explosion of flame. Each creature in a 20-foot-radius sphere centered on that point must make a Dexterity saving throw. A target takes 8d6 fire damage on a failed save, or half as much damage on a successful one.
	The fire spreads around forners. It ignites flammable objects in the area that aren't being worn or carried.",
	upcast_description: Some("When you cast this spell using a spell slot of 4th level or higher, the damage increases by 1d6 for eaeh slot level above 3rd.")
};

pub static fire_bolt: Spell = Spell
{
	name: "Fire Bolt",
	level: Level::Cantrip,
	school: MagicSchool::Evocation,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Feet(120),
	has_verbal_component: true,
	has_somantic_component: true,
	material_components: None,
	duration: Duration::Instant,
	description: "You hurl a mote of fire at a creature or object within range. Make a ranged spell attack against the target. On a hit, the target takes 1d10 fire damage. A flammable object hit by this spell ignites if it isn't being worn or carried.
	This spell's damage increases by 1d10 when you reach 5th level (2d10), 11th level (3d10), and 17th level (4d10).",
	upcast_description: None
};
