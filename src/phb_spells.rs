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

pub static ALTER_SELF: Spell = Spell
{
	name: "Alter Self",
	level: Level::Level2,
	school: MagicSchool::Transmutation,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Yourself(AOE::None),
	has_v_component: true,
	has_s_component: true,
	m_components: None,
	duration: Duration::Hours(1, true),
	description: "You assume a different form. When you cast the spell, choose one of the following options, the effects of which last for the duration of the spell. While the spell lasts, you can end one option as an action to gain the benefits of a different one.
	Aquatic Adaptation: You adapt your body to an aquatic environment, sprouting gills and growing webbing between your fingers. You can breathe underwater and gain a swimming speed equal to your walking speed.
	Change Appearance: You transform your appearance. You decide what you look like, including your height, weight, facial features, sound of your voice, hair length, coloration, and distinguishing characteristics, if any. You can make yourself appear as a member of another race, though none of your statistics change. You also can't appear as a creature of a different size than you, and your basic shape stays the same; if you're bipedal, you can't use this spell to become quadrupedal, for instance. At any time for the duration of the spell, you can use your action to change your appearance in this way again.
	Natural Weapons: You grow claws, fangs, spines, horns, or a different natural weapon of your choice. Your unarmed strikes deal 1d6 bludgeoning, piercing, or slashing damage, as appropriate to the natural weapon you chose, and you are proficient with your unarmed strikes. Finally, the natural weapon is magic and you have a +1 bonus to the attack and damage rolls you make using it.",
	upcast_description: None
};

pub static ANIMAL_FRIENDSHIP: Spell = Spell
{
	name: "Animal Friendship",
	level: Level::Level1,
	school: MagicSchool::Enchantment,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Feet(30),
	has_v_component: true,
	has_s_component: true,
	m_components: Some("a morsel of food"),
	duration: Duration::Hours(24, false),
	description: "This spell lets you convince a beast that you mean it no harm. Choose a beast that you can see within range. It must see and hear you. If the beast's Intelligence is 4 or higher, the spell fails. Otherwise, the beast must succeed on a Wisdom saving throw or be charmed by you for the spell's duration. If you or one of your companions harms the target, the spell ends.",
	upcast_description: Some("When you cast this spell using a spell slot of 2nd level or higher, you can affect one additional beast for each slot level above 1st.")
};

//
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
