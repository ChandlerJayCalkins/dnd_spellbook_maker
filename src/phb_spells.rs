use crate::spells::*;

static fireball: Spell = Spell
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
	requires_concentration: false,
	description: "A bright streak flashes from your pointing finger to a 
	point you choose within range and then blossoms with 
	a low roar into an explosion of flame. Each creature 
	in a 20-foot-radius sphere centered on that point must 
	make a Dexterity saving throw. A target takes 8d6 fire 
	damage on a failed save, or half as mueh damage on a 
	successful one.\n
	The fire spreads around forners. It ignites flammable 
	objects in the area that aren't being worn or carried.",
	upcast_description: Some("When you cast this spell using a 
	spell slot of 4th level or higher, the damage increases by 
	1d6 for eaeh slot level above 3rd.")
};
