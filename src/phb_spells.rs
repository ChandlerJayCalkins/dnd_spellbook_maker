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

pub static ANIMAL_MESSENGER: Spell = Spell
{
	name: "Animal Messenger",
	level: Level::Level2,
	school: MagicSchool::Enchantment,
	is_ritual: true,
	casting_time: CastingTime::Actions(1),
	range: Range::Feet(30),
	has_v_component: true,
	has_s_component: true,
	m_components: Some("a morsel of food"),
	duration: Duration::Hours(24, false),
	description: "By means of this spell, you use an animal to deliver a message. Choose a Tiny beast you can see within range, such as a squirrel, a blue jay, or a bat. You specify a location, which you must have visited, and a recipient who matches a general description, such as \"a man or woman dressed in the uniform of the town guard\" or \"a red-haired dwarf wearing a pointed hat.\" You also speak a message of up to twenty-five words. The target beast travels for the duration of the spell toward the specified location, covering about 50 miles per 24 hours for a flying messenger, or 25 miles for other animals.
	When the messenger arrives, it delivers your message to the creature that you described, replicating the sound of your voice. The messenger speaks only to a creature matching the description you gave. If the messenger doesn't reach its destination before the spell ends, the message is lost, and the beast makes its way back to where you cast this spell.",
	upcast_description: Some("If you cast this spell using a spell slot of 3rd level or higher, the duration of the spell increases by 48 hours for each slot level above 2nd.")
};

pub static ANIMAL_SHAPES: Spell = Spell
{
	name: "Animal Shapes",
	level: Level::Level8,
	school: MagicSchool::Transmutation,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Feet(30),
	has_v_component: true,
	has_s_component: true,
	m_components: None,
	duration: Duration::Hours(24, true),
	description: "Your magic turns others into beasts. Choose any number of willing creatures that you can see within range. You transform each target into the form of a Large or smaller beast with a challenge rating of 4 or lower. On subsequent turns, you can use your action to transform affected creatures into new forms.
	The transformation lasts for the duration for each target, or until the target drops to 0 hit points or dies. You can choose a different form for each target. A target's game statistics are replaced by the statistics of the chosen beast, though the target retains its alignment and Intelligence, Wisdom, and Charisma scores. The target assumes the hit points of its new form, and when it reverts to its normal form, it returns to the number of hit points it had before it transformed. If it reverts as a result of dropping to 0 hit points, any excess damage carries over to its normal form. As long as the excess damage doesn't reduce the creature's normal form to 0 hit points, it isn't knocked unconscious. The creature is limited in the actions it can perform by the nature of its new form, and it can't speak or cast spells.
	The target's gear melds into the new form. The target can't activate, wield, or otherwise benefit from any of its equipment.",
	upcast_description: None
};

pub static ANIMATE_DEAD: Spell = Spell
{
	name: "Animate Dead",
	level: Level::Level3,
	school: MagicSchool::Necromancy,
	is_ritual: false,
	casting_time: CastingTime::Minutes(1),
	range: Range::Feet(10),
	has_v_component: true,
	has_s_component: true,
	m_components: Some("a drop of blood, a piece of flesh, and a pinch of bone dust"),
	duration: Duration::Instant,
	description: "This spell creates an undead servant. Choose a pile of bones or a corpse of a Medium or Small humanoid within range. Your spell imbues the target with a foul mimicry of life, raising it as an undead creature. The target becomes a skeleton if you chose bones or a zombie if you chose a corpse (the GM has the creature's game statistics).
	On each of your turns, you can use a bonus action to mentally command any creature you made with this spell if the creature is within 60 feet of you (if you control multiple creatures, you can command any or all of them at the same time, issuing the same command to each one). You decide what action the creature will take and where it will move during its next turn, or you can issue a general command, such as to guard a particular chamber or corridor. If you issue no commands, the creature only defends itself against hostile creatures. Once given an order, the creature continues to follow it until its task is complete.
	The creature is under your control for 24 hours, after which it stops obeying any command you've given it. To maintain control of the creature for another 24 hours, you must cast this spell on the creature again before the current 24-hour period ends. This use of the spell reasserts your control over up to four creatures you have animated with this spell, rather than animating a new one.",
	upcast_description: Some("When you cast this spell using a spell slot of 4th level or higher, you animate or reassert control over two additional undead creatures for each slot level above 3rd. Each of the creatures must come from a different corpse or pile of bones.")
};

pub static ANIMATE_OBJECTS: Spell = Spell
{
	name: "Animate Objects",
	level: Level::Level5,
	school: MagicSchool::Transmutation,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Feet(120),
	has_v_component: true,
	has_s_component: true,
	m_components: None,
	duration: Duration::Minutes(1, true),
	description: "Objects come to life at your command. Choose up to ten nonmagical objects within range that are not being worn or carried. Medium targets count as two objects, Large targets count as four objects, Huge targets count as eight objects. You can't animate any object larger than Huge. Each target animates and becomes a creature under your control until the spell ends or until reduced to 0 hit points.
	As a bonus action, you can mentally command any creature you made with this spell if the creature is within 500 feet of you (if you control multiple creatures, you can command any or all of them at the same time, issuing the same command to each one). You decide what action the creature will take and where it will move during its next turn, or you can issue a general command, such as to guard a particular chamber or corridor. If you issue no commands, the creature only defends itself against hostile creatures. Once given an order, the creature continues to follow it until its task is complete.
	Size | HP | AC | Str | Dex | Attack
	Tiny | 20 | 18 | 4 | 18 | +8 to hit, 1d4 + 4 damage
	Small | 25 | 16 | 6 | 14 | +6 to hit, 1d8 + 2 damage
	Medium | 40 | 13 | 10 | 12 | +5 to hit, 2d6 + 1 damage
	Large | 50 | 10 | 14 | 10 | +6 to hit, 2d10 + 2 damage
	Huge | 80 | 10 | 18 | 6 | +8 to hit, 2d12 + 4 damage
	An animated object is a construct with AC, hit points, attacks, Strength, and Dexterity determined by its size. Its Constitution is 10 and its Intelligence and Wisdom are 3, and its Charisma is 1. Its speed is 30 feet; if the object lacks legs or other appendages it can use for locomotion, it instead has a flying speed of 30 feet and can hover. If the object is securely attached to a surface or a larger object, such as a chain bolted to a wall, its speed is 0. It has blindsight with a radius of 30 feet and is blind beyond that distance. When the animated object drops to 0 hit points, it reverts to its original object form, and any remaining damage carries over to its original object form.
	If you command an object to attack, it can make a single melee attack against a creature within 5 feet of it. It makes a slam attack with an attack bonus and bludgeoning damage determined by its size. The GM might rule that a specific object inflicts slashing or piercing damage based on its form.",
	upcast_description: Some("If you cast this spell using a spell slot of 6th level or higher, you can animate two additional objects for each slot level above 5th.")
};

pub static ANTILIFE_SHELL: Spell = Spell
{
	name: "Antilife Shell",
	level: Level::Level5,
	school: MagicSchool::Abjuration,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Yourself(AOE::Sphere(10)),
	has_v_component: true,
	has_s_component: true,
	m_components: None,
	duration: Duration::Hours(1, true),
	description: "A shimmering barrier extends out from you in a 10-foot radius and moves with you, remaining centered on you and hedging out creatures other than undead and constructs. The barrier lasts for the duration.
	The barrier prevents an affected creature from passing or reaching through. An affected creature can cast spells or make attacks with ranged or reach weapons through the barrier.
	If you move so that an affected creature is forced to pass through the barrier, the spell ends.",
	upcast_description: None
};

pub static ANTIMAGIC_FIELD: Spell = Spell
{
	name: "Antimagic Field",
	level: Level::Level8,
	school: MagicSchool::Abjuration,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Yourself(AOE::Sphere(10)),
	has_v_component: true,
	has_s_component: true,
	m_components: Some("a pinch of powered iron or iron filings"),
	duration: Duration::Hours(1, true),
	description: "A 10-foot-radius invisible sphere of antimagic surrounds you. This area is divorced from the magical energy that suffuses the multiverse. Within the sphere, spells can't be cast, summoned creatures disappear, and even magic items become mundane. Until the spell ends, the sphere moves with you, centered on you.
	Spells and other magical effects, except those created by an artifact or a deity, are suppressed in the sphere and can't protrude into it. A slot expended to cast a suppressed spell is consumed. While an effect is suppressed, it doesn't function, but the time it spends suppressed counts against its duration.
	Targeted Effects: Spells and other magical effects, such as magic missile and charm person, that target a creature or an object in the sphere have no effect on that target.
	Areas of Magic: The area of another spell or magical effect, such as fireball, can't extend into the sphere. If the sphere overlaps an area of magic, the part of the area that is covered by the sphere is suppressed. For example, the flames created by a wall of fire are suppressed within the sphere, creating a gap in the wall if the overlap is large enough.
	Spells: Any active spell or other magical effect on a creature or an object in the sphere is suppressed while the creature or object is in it.
	Magic Items: The properties and powers of magic items are suppressed in the sphere. For example, a longsword, +1 in the sphere functions as a nonmagical longsword.
	A magic weapon's properties and powers are suppressed if it is used against a target in the sphere or wielded by an attacker in the sphere. If a magic weapon or a piece of magic ammunition fully leaves the sphere (for example, if you fire a magic arrow or throw a magic spear at a target outside the sphere), the magic of the item ceases to be suppressed as soon as it exits.
	Magical Travel: Teleportation and planar travel fail to work in the sphere, whether the sphere is the destination or the departure point for such magical travel. A portal to another location, world, or plane of existence, as well as an opening to an extradimensional space such as that created by the rope trick spell, temporarily closes while in the sphere.
	Creatures and Objects: A creature or object summoned or created by magic temporarily winks out of existence in the sphere. Such a creature instantly reappears once the space the creature occupied is no longer within the sphere.
	Dispel Magic: Spells and magical effects such as dispel magic have no effect on the sphere. Likewise, the spheres created by different antimagic field spells don't nullify each other.",
	upcast_description: None
};

pub static ANTIPATHY_SYMPATHY: Spell = Spell
{
	name: "Antipathy / Sympathy",
	level: Level::Level8,
	school: MagicSchool::Enchantment,
	is_ritual: false,
	casting_time: CastingTime::Hours(1),
	range: Range::Feet(60),
	has_v_component: true,
	has_s_component: true,
	m_components: Some("either a lump of alum soaked in vinegar for the antipathy effect or a drop of honey for the sympathy effect"),
	duration: Duration::Days(10, false),
	description: "This spell attracts or repels creatures of your choice. You target something within range, either a Huge or smaller object or creature or an area that is no larger than a 200-foot cube. Then specify a kind of intelligent creature, such as red dragons, goblins, or vampires. You invest the target with an aura that either attracts or repels the specified creatures for the duration. Choose antipathy or sympathy as the aura's effect.
	Antipathy: The enchantment causes creatures of the kind you designated to feel an intense urge to leave the area and avoid the target. When such a creature can see the target or comes within 60 feet of it, the creature must succeed on a Wisdom saving throw or become frightened. The creature remains frightened while it can see the target or is within 60 feet of it. While frightened by the target, the creature must use its movement to move to the nearest safe spot from which it can't see the target. If the creature moves more than 60 feet from the target and can't see it, the creature is no longer frightened, but the creature becomes frightened again if it regains sight of the target or moves within 60 feet of it.
	Sympathy: The enchantment causes the specified creatures to feel an intense urge to approach the target while within 60 feet of it or able to see it. When such a creature can see the target or comes within 60 feet of it, the creature must succeed on a Wisdom saving throw or use its movement on each of its turns to enter the area or move within reach of the target. When the creature has done so, it can't willingly move away from the target.
	If the target damages or otherwise harms an affected creature, the affected creature can make a Wisdom saving throw to end the effect, as described below.
	Ending the Effect: If an affected creature ends its turn while not within 60 feet of the target or able to see it, the creature makes a Wisdom saving throw. On a successful save, the creature is no longer affected by the target and recognizes the feeling of repugnance or attraction as magical. In addition, a creature affected by the spell is allowed another Wisdom saving throw every 24 hours while the spell persists.
	A creature that successfully saves against this effect is immune to it for 1 minute, after which time it can be affected again.",
	upcast_description: None
};

pub static ARCANE_EYE: Spell = Spell
{
	name: "Arcane Eye",
	level: Level::Level4,
	school: MagicSchool::Divination,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Feet(30),
	has_v_component: true,
	has_s_component: true,
	m_components: Some("a bit of bat fur"),
	duration: Duration::Hours(1, true),
	description: "You create an invisible, magical eye within range that hovers in the air for the duration.
	You mentally receive visual information from the eye, which has normal vision and darkvision out to 30 feet. The eye can look in every direction.
	As an action, you can move the eye up to 30 feet in any direction. There is no limit to how far away from you the eye can move, but it can't enter another plane of existence. A solid barrier blocks the eye's movement, but the eye can pass through an opening as small as 1 inch in diameter.",
	upcast_description: None
};

pub static ARCANE_GATE: Spell = Spell
{
	name: "Arcane Gate",
	level: Level::Level6,
	school: MagicSchool::Conjuration,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Feet(500),
	has_v_component: true,
	has_s_component: true,
	m_components: None,
	duration: Duration::Minutes(10, true),
	description: "You create linked teleportation portals that remain open for the duration. Choose two points on the ground that you can see, one point within 500 feet of you. A circular portal, 10 feet in diameter, opens over each point. If the portal would open in the space occupied by a creature, the spell fails, and the casting is lost.
	The portals are two-dimensional glowing rings filled with mist, hovering inches from the ground and perpendicular to it at the points you choose. A ring is visible only from one side (your choice), which is the side that functions as a portal.
	Any creature or object entering the portal exits from the other portal as if the two were adjacent to each other; passing through a portal from the nonportal side has no effect. The mist that fills each portal is opaque and blocks vision through it. On your turn, you can rotate the rings as a bonus action so that the active side faces a different direction.",
	upcast_description: None
};

pub static ARCANE_LOCK: Spell = Spell
{
	name: "Arcane Lock",
	level: Level::Level2,
	school: MagicSchool::Abjuration,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Touch,
	has_v_component: true,
	has_s_component: true,
	m_components: Some("gold dust worth at least 25 gp, which the spell consumes"),
	duration: Duration::UntilDispelled(false),
	description: "You touch a closed door, window, gate, chest, or other entryway, and it becomes locked for the duration. You and the creatures you designate when you cast this spell can open the object normally. You can also set a password that, when spoken within 5 feet of the object, suppresses this spell for 1 minute. Otherwise, it is impassable until it is broken or the spell is dispelled or suppressed. Casting knock on the object suppresses arcane lock for 10 minutes.
	While affected by this spell, the object is more difficult to break or force open; the DC to break it or pick any locks on it increases by 10.",
	upcast_description: None
};

pub static ARMOR_OF_AGATHYS: Spell = Spell
{
	name: "Armor of Agathys",
	level: Level::Level1,
	school: MagicSchool::Abjuration,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Yourself(AOE::None),
	has_v_component: true,
	has_s_component: true,
	m_components: Some("a cup of water"),
	duration: Duration::Hours(1, false),
	description: "A protective magical force surrounds you, manifesting as a spectral frost that covers you and your gear. You gain 5 temporary hit points for the duration. If a creature hits you with a melee attack while you have these hit points, the creature takes 5 cold damage.",
	upcast_description: Some("When you cast this spell using a spell slot of 2nd level or higher, both temporary hit points and the cold damage increase by 5 for each slot level above 1st.")
};

pub static ARMS_OF_HADAR: Spell = Spell
{
	name: "Arms of Hadar",
	level: Level::Level1,
	school: MagicSchool::Conjuration,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Yourself(AOE::Sphere(10)),
	has_v_component: true,
	has_s_component: true,
	m_components: None,
	duration: Duration::Instant,
	description: "You invoke the power of Hadar, the Dark Hunger. Tendrils of dark energy erupt from you and batter all creatures within 10 feet of you. Each creature in that area must make a strength saving throw. On a failed save, a target takes 2d6 necrotic damage and can't take reactions until its next turn. On a successful save, the creature takes half damage, but suffers no other effect.",
	upcast_description: Some("When you cast this spell using a spell slot of 2nd level or higher, the damage increases by 1d6 for each slot level above 1st.")
};

pub static ASTRAL_PROJECTION: Spell = Spell
{
	name: "Astral Projection",
	level: Level::Level9,
	school: MagicSchool::Necromancy,
	is_ritual: false,
	casting_time: CastingTime::Hours(1),
	range: Range::Feet(10),
	has_v_component: true,
	has_s_component: true,
	m_components: Some("for each creature you affect with this spell, you must provide one jacinth worth at least 1,000 gp, all of which the spell consumes"),
	duration: Duration::Special(false),
	description: "You and up to eight willing creatures within range project your astral bodies into the Astral Plane (the spell fails and the casting is wasted if you are already on that plane). The material body you leave behind is unconscious and in a state of suspended animation; it doesn't need food or air and doesn't age.
	Your astral body resembles your mortal form in almost every way, replicating your game statistics and possessions. The principal difference is the addition of a silvery cord that extends from between your shoulder blades and trails behind you, fading to invisibility after 1 foot. This cord is your tether to your material body. As long as the tether remains intact, you can find your way home. If the cord is cut--something that can happen only when an effect specifically states that it does--your soul and body are separated, killing you instantly.
	Your astral form can freely travel through the Astral Plane and can pass through portals there leading to any other plane. If you enter a new plane or return to the plane you were on when casting this spell, your body and possessions are transported along the silver cord, allowing you to re-enter your body as you enter the new plane. Your astral form is a separate incarnation. Any damage or other effects that apply to it have no effect on your physical body, nor do they persist when you return to it.
	The spell ends for you and your companions when you use your action to dismiss it. When the spell ends, the affected creature returns to its physical body, and it awakens.
	The spell might also end early for you or one of your companions. A successful dispel magic spell used against an astral or physical body ends the spell for that creature. If a creature's original body or its astral form drops to 0 hit points, the spell ends for that creature. If the spell ends and the silver cord is intact, the cord pulls the creature's astral form back to its body, ending its state of suspended animation.
	If you are returned to your body prematurely, your companions remain in their astral forms and must find their own way back to their bodies, usually by dropping to 0 hit points.",
	upcast_description: None
};

pub static AUGURY: Spell = Spell
{
	name: "Augury",
	level: Level::Level2,
	school: MagicSchool::Divination,
	is_ritual: true,
	casting_time: CastingTime::Minutes(1),
	range: Range::Yourself(AOE::None),
	has_v_component: true,
	has_s_component: true,
	m_components: Some("specially marked sticks, bones, or similar tokens worth at least 25 gp"),
	duration: Duration::Instant,
	description: "By casting gem-inlaid sticks, rolling dragon bones, laying out ornate cards, or employing some other divining tool, you receive an omen from an otherworldly entity about the results of a specific course of action that you plan to take within the next 30 minutes. The DM chooses from the following possible omens:
	Weal: for good results
	Woe: for bad results
	Weal and Woe: for both good and bad results
	Nothing: for results that aren't especially good or bad
	The spell doesn't take into account any possible circumstances that might change the outcome, such as the casting of additional spells or the loss or gain of a companion.
	If you cast the spell two or more times before completing your next long rest, there is a cumulative 25 percent chance for each casting after the first that you get a random reading. The DM makes this roll in secret.",
	upcast_description: None
};

pub static AURA_OF_LIFE: Spell = Spell
{
	name: "Aura of Life",
	level: Level::Level4,
	school: MagicSchool::Abjuration,
	is_ritual: false,
	casting_time: CastingTime::Actions(1),
	range: Range::Yourself(AOE::Sphere(30)),
	has_v_component: true,
	has_s_component: false,
	m_components: None,
	duration: Duration::Minutes(10, true),
	description: "Life-preserving energy radiates from you in an aura with a 30-foot radius. until the spell ends, the aura moves with you, centered on you. Each nonhostile creature in the aura (including you) has resistance to necrotic damage, and its hit  point maximum can't be reduced. In addition, a nonhostile, living creature regains 1 hit point when it starts its turn in the aura with 0 hit points.",
	upcast_description: None
};

// -----------------------------------------------------------------------------------------------------
// DONT FORGET TO ADD TO THE SPELL LIST
// -----------------------------------------------------------------------------------------------------
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

pub static SPELL_LIST: [&Spell; 20] =
[
	&ACID_SPLASH, &AID, &ALARM, &ALTER_SELF, &ANIMAL_FRIENDSHIP, &ANIMAL_MESSENGER,
	&ANIMAL_SHAPES, &ANIMATE_DEAD, &ANIMATE_OBJECTS, &ANTILIFE_SHELL,
	&ANTIMAGIC_FIELD, &ANTIPATHY_SYMPATHY, &ARCANE_EYE, &ARCANE_GATE, &ARCANE_LOCK,
	&ARMOR_OF_AGATHYS, &ARMS_OF_HADAR, &ASTRAL_PROJECTION, &AUGURY, &AURA_OF_LIFE
];
