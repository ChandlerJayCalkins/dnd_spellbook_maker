//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//	Tests
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;
use std::path::Path;

use crate::utils::*;

#[test]
fn idk() {}

// Makes sure that creating valid spell files works
#[test]
fn create_spell_files()
{
	// Path to hand-made spell files that are compared to generated spells
	let comparison_folder = String::from("spells/necronomicon/");

	// Create the spells (necronomicon spell duplicates)
	let hell_spell = spells::Spell
	{
		name: String::from("HELL SPELL AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A"),
		level: spells::SpellField::Custom(String::from("100TH-LEVEL")),
		school: spells::SpellField::Custom(String::from("SUPER NECROMANCY")),
		is_ritual: true,
		casting_time: spells::SpellField::Controlled(spells::CastingTime::Reaction(String::from("THAT YOU TAKE WHEN YOU FEEL LIKE CASTING SPELLS AND DOING MAGIC AND AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A"))),
		range: spells::SpellField::Controlled(spells::Range::Yourself(Some(spells::AOE::Cylinder(spells::Distance::Miles(63489), spells::Distance::Miles(49729))))),
		has_v_component: true,
		has_s_component: true,
		m_components: Some(String::from("UNLIMITED POWAHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H")),
		duration: spells::SpellField::Controlled(spells::Duration::Years(57394, true)),
		description: String::from("<ib> CASTING SPELLS AND CONJURING ABOMINATIONS <b> AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA <r> THIS SPELL ISN'T FOR <i> weak underpowered feeble wizards -_-. <r> THIS SPELL IS FOR ONLY THE MOST POWERFUL OF ARCHMAGES AND NECROMANCERS WHO CAN WIELD THE MIGHTIEST OF <bi> ARCANE ENERGY <r> WITH THE FORTITUDE OF A <ib> MOUNTAIN. <b> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
<table> <title> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A \\A \\\\A \\\\\\A \\<title> \\\\<title> \\\\\\<title> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A <title>
COLUMN OF CHAOS | COLUMN OF NECROMANCY
<row> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A | A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
<row> B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B | B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B | B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B
<row> POWER | WIZARDRY
<row> C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C | C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C
<table>
MORE MAGIC SPELLS AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
<table> <title> THIS TABLE AGAIN A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A \\A \\\\A \\\\\\A \\<title> \\\\<title> \\\\\\<title> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A <title>
COLUMN OF CHAOS | COLUMN OF NECROMANCY
<row> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A | A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
<row> B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B | B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B | B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B
<row> POWER | WIZARDRY
<row> C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C | C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C
<table>
YOU CAN'T HANDLE THIS SPELL A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A
A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A"),
		upcast_description: Some(String::from("HELL ON EARTH")),
		tables: Vec::new()
	};
	let power_word_scrunch = spells::Spell
	{
		name: String::from("Power Word Scrunch"),
		level: spells::SpellField::Controlled(spells::Level::Level9),
		school: spells::SpellField::Controlled(spells::MagicSchool::Transmutation),
		is_ritual: false,
		casting_time: spells::SpellField::Controlled(spells::CastingTime::Actions(1)),
		range: spells::SpellField::Controlled(spells::Range::Dist(spells::Distance::Feet(60))),
		has_v_component: true,
		has_s_component: false,
		m_components: None,
		duration: spells::SpellField::Controlled(spells::Duration::Instant),
		description: String::from("Choose 1 target creature or object within range. That target gets scrunched.
- Scrunching has these effects <table> <title> Scrunching Effects <title>
Target | Effect
<row> Creature | Flesh Ball
<row> Object | Ball of that object's material
<row> Creature not made of flesh | Ball of that creature's material
<table>
- Scrunch balls (balls produced from scrunching) can be thrown and do 1d6 bludgeoning damage on hit.
Scrunch ball funny lol."),
		upcast_description: None,
		tables: Vec::new()
	};
	let the_ten_hells = spells::Spell
	{
		name: String::from("The Ten Hells"),
		level: spells::SpellField::Controlled(spells::Level::Level9),
		school: spells::SpellField::Controlled(spells::MagicSchool::Necromancy),
		is_ritual: true,
		casting_time: spells::SpellField::Controlled(spells::CastingTime::Actions(1)),
		range: spells::SpellField::Controlled(spells::Range::Yourself(Some(spells::AOE::Sphere(spells::Distance::Feet(90))))),
		has_v_component: true,
		has_s_component: false,
		m_components: Some(String::from("the nail or claw of a creature from an evil plane")),
		duration: spells::SpellField::Controlled(spells::Duration::Instant),
		description: String::from("Choose any number of creatures made of tangible matter within range. Those creatures must all make a constitution savint throw against your spell save DC. All creatures that fail this saving throw get turned inside out, immediately die, and have their souls eternally damned to all nine hells simultaneously.
Creatures that succeed the saving throw take 20d4 scrunching damage."),
		upcast_description: None,
		tables: Vec::new()
	};

	// Create vec of test spells and their file names (without extension or path)
	let spell_list = vec![(hell_spell, "hell_spell"), (power_word_scrunch, "power_word_scrunch"), (the_ten_hells, "the_ten_hells")];
	// Test to make sure spell files can be created properly
	spell_file_test(&spell_list, true, "spells/tests/spell/", &comparison_folder);
	json_file_test(&spell_list, false, "spells/tests/json/", &comparison_folder);
}

// Creates spell files from a list of spells into the output folder and compares them to the same hand-crafted spells in the comparison folder
fn spell_file_test(spell_list: &Vec<(spells::Spell, &str)>, compress: bool, output_folder: &str, comparison_folder: &str)
{
	const FILE_EXTENSION: &str = ".spell";

	// If the output folder doesn't exist yet
	if !Path::new(&output_folder).exists()
	{
		// Create it
		fs::create_dir(&output_folder).unwrap();
	}

	// Create vec of file paths to each spell that is going to be generated
	let mut spell_paths = Vec::with_capacity(spell_list.len());
	for spell in spell_list
	{
		// Get file path to the spell that's about to be generated and add it to the vec
		let spell_path = output_folder.to_owned() + spell.1 + FILE_EXTENSION;
		spell_paths.push(spell_path.clone());
		// Generate the spell file for this spell
		spell.0.to_file(&spell_path, compress).unwrap();
	}

	// Create a list of just the spells and the file names from the spell_list parameter
	let real_spell_list: Vec<spells::Spell> = spell_list.into_iter().map(|(s, _)| s.clone()).collect();
	// Read all of the generated spell files into spell objects and put them into a list
	let test_spell_list = get_all_spells_in_folder(&output_folder).unwrap();
	// Compare if the spell objects are the same
	assert_eq!(real_spell_list, test_spell_list);

	// Loop through spell file paths
	for (spell, spell_path) in spell_list.iter().zip(spell_paths.iter())
	{
		// Read all of the bytes from the original spell that the generated one was based on
		let real_spell_bytes = fs::read(&(comparison_folder.to_owned() + spell.1 + FILE_EXTENSION)).unwrap();
		// Read all of the bytes from the generated spell file
		let test_spell_bytes = fs::read(&spell_path).unwrap();
		// Compare the bytes from both files to make sure they are the same
		assert_eq!(real_spell_bytes, test_spell_bytes);
	}
}

// Creates json files from a list of spells into the output folder and compares them to the same hand-crafted spells in the comparison folder
fn json_file_test(spell_list: &Vec<(spells::Spell, &str)>, compress: bool, output_folder: &str, comparison_folder: &str)
{
	const FILE_EXTENSION: &str = ".json";

	// If the output folder doesn't exist yet
	if !Path::new(&output_folder).exists()
	{
		// Create it
		fs::create_dir(&output_folder).unwrap();
	}

	// Create vec of file paths to each spell that is going to be generated
	let mut spell_paths = Vec::with_capacity(spell_list.len());
	for spell in spell_list
	{
		// Get file path to the spell that's about to be generated and add it to the vec
		let spell_path = output_folder.to_owned() + spell.1 + FILE_EXTENSION;
		spell_paths.push(spell_path.clone());
		// Generate the json file for this spell
		spell.0.to_json_file(&spell_path, compress).unwrap();
	}

	// Create a list of just the spells and the file names from the spell_list parameter
	let real_spell_list: Vec<spells::Spell> = spell_list.into_iter().map(|(s, _)| s.clone()).collect();
	// Read all of the generated spell files into spell objects and put them into a list
	let test_spell_list = get_all_json_spells_in_folder(&output_folder).unwrap();
	// Compare if the spell objects are the same
	assert_eq!(real_spell_list, test_spell_list);

	// Loop through spell file paths
	for (spell, spell_path) in spell_list.iter().zip(spell_paths.iter())
	{
		// Read all of the bytes from the original spell that the generated one was based on
		let real_spell_bytes = fs::read(&(comparison_folder.to_owned() + spell.1 + FILE_EXTENSION)).unwrap();
		// Read all of the bytes from the generated spell file
		let test_spell_bytes = fs::read(&spell_path).unwrap();
		// Compare the bytes from both files to make sure they are the same
		assert_eq!(real_spell_bytes, test_spell_bytes);
	}
}

// Creates 2 spellbooks that combined contain every spell from the official d&d 5e player's handbook
#[test]
fn players_handbook()
{
	// Spellbook names
	let spellbook_name_1 = "Every Sepll in the 2014 Dungeons & Dragons 5th Edition Player's Handbook: Part 1";
	let spellbook_name_2 = "Every Sepll in the 2014 Dungeons & Dragons 5th Edition Player's Handbook: Part 2";
	// List of every spell in the player's handbook folder
	let spell_list = get_all_spells_in_folder("spells/players_handbook_2014").unwrap();
	// Split that vec into 2 vecs
	let spell_list_1 = spell_list[..spell_list.len()/2].to_vec();
	let spell_list_2 = spell_list[spell_list.len()/2..].to_vec();
	// File paths to the fonts needed
	let font_paths = FontPaths
	{
		regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
		bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
		italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
		bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
	};
	// Parameters for determining font sizes
	let font_sizes = FontSizes::new(32.0, 24.0, 12.0, 16.0, 12.0).unwrap();
	// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
	let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
	// Parameters for determining tab and newline sizes
	let spacing_options = SpacingOptions::new(7.5, 12.0, 8.0, 5.0, 6.4, 5.0).unwrap();
	// Colors for each type of text
	let text_colors = TextColors
	{
		title_color: (0, 0, 0),
		header_color: (115, 26, 26),
		body_color: (0, 0, 0),
		table_title_color: (0, 0, 0),
		table_body_color: (0, 0, 0)
	};
	// Parameters for determining the size of the page and the text margins on the page
	let page_size_options = PageSizeOptions::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
	// Parameters for determining page number behavior
	let page_number_options = PageNumberOptions::new
	(HSide::Left, false, 1, FontVariant::Regular, 12.0, 5.0, (0, 0, 0), 5.0, 4.0).unwrap();
	// File path to the background image
	let background_path = "img/parchment.jpg";
	// Image transform data for the background image
	let background_transform = ImageTransform
	{
		translate_x: Some(Mm(0.0)),
		translate_y: Some(Mm(0.0)),
		scale_x: Some(1.95),
		scale_y: Some(2.125),
		..Default::default()
	};
	// Parameters for table margins / padding and off-row color / scaling
	let table_options = TableOptions::new(10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
	// Create a spellbook with the first half of the spells
	let (doc_1, _) = create_spellbook
	(
		spellbook_name_1,
		spell_list_1,
		font_paths.clone(),
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((background_path, background_transform)),
		table_options
	).unwrap();
	// Save the first spellbook to a file
	let _ = save_spellbook(doc_1, "Player's Handbook 2014 Spells 1.pdf").unwrap();
	// Create a spellbook with the second half of the spells
	let (doc_2, _) = create_spellbook
	(
		spellbook_name_2,
		spell_list_2,
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((background_path, background_transform)),
		table_options
	).unwrap();
	// Save the second spellbook to a file
	let _ = save_spellbook(doc_2, "Player's Handbook 2014 Spells 2.pdf").unwrap();
}

// Create a spellbook with every spell from the xanathar's guide to everything source book
#[test]
fn xanathars_guide_to_everything()
{
	// Spellbook's name
	let spellbook_name =
	"Every Sepll in the Dungeons & Dragons 5th Edition Source Material Book \"Xanathar's Guide to Everything\"";
	// List of every spell in this folder
	let spell_list = get_all_spells_in_folder("spells/xanathars_guide_to_everything").unwrap();
	// File paths to the fonts needed
	let font_paths = FontPaths
	{
		regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
		bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
		italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
		bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
	};
	// Parameters for determining font sizes
	let font_sizes = FontSizes::new(32.0, 24.0, 12.0, 16.0, 12.0).unwrap();
	// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
	let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
	// Parameters for determining tab and newline sizes
	let spacing_options = SpacingOptions::new(7.5, 12.0, 8.0, 5.0, 6.4, 5.0).unwrap();
	// Colors for each type of text
	let text_colors = TextColors
	{
		title_color: (0, 0, 0),
		header_color: (115, 26, 26),
		body_color: (0, 0, 0),
		table_title_color: (0, 0, 0),
		table_body_color: (0, 0, 0)
	};
	// Parameters for determining the size of the page and the text margins on the page
	let page_size_options = PageSizeOptions::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
	// Parameters for determining page number behavior
	let page_number_options = PageNumberOptions::new
	(HSide::Left, false, 1, FontVariant::Regular, 12.0, 5.0, (0, 0, 0), 5.0, 4.0).unwrap();
	// File path to the background image
	let background_path = "img/parchment.jpg";
	// Image transform data for the background image
	let background_transform = ImageTransform
	{
		translate_x: Some(Mm(0.0)),
		translate_y: Some(Mm(0.0)),
		scale_x: Some(1.95),
		scale_y: Some(2.125),
		..Default::default()
	};
	// Parameters for table margins / padding and off-row color / scaling
	let table_options = TableOptions::new(10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
	// Create the spellbook
	let (doc, _) = create_spellbook
	(
		spellbook_name,
		spell_list,
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((background_path, background_transform)),
		table_options
	).unwrap();
	// Save the spellbook to a file
	let _ = save_spellbook(doc, "Xanathar's Guide to Everything Spells.pdf").unwrap();
}

// Create a spellbook with every spell from the tasha's cauldron of everything source book
#[test]
fn tashas_cauldron_of_everything()
{
	// Spellbook's name
	let spellbook_name =
	"Every Sepll in the Dungeons & Dragons 5th Edition Source Material Book \"Tasha's Cauldron of Everything\"";
	// List of every spell in this folder
	let spell_list = get_all_spells_in_folder("spells/tashas_cauldron_of_everything").unwrap();
	// File paths to the fonts needed
	let font_paths = FontPaths
	{
		regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
		bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
		italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
		bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
	};
	// Parameters for determining font sizes
	let font_sizes = FontSizes::new(32.0, 24.0, 12.0, 16.0, 12.0).unwrap();
	// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
	let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
	// Parameters for determining tab and newline sizes
	let spacing_options = SpacingOptions::new(7.5, 12.0, 8.0, 5.0, 6.4, 5.0).unwrap();
	// Colors for each type of text
	let text_colors = TextColors
	{
		title_color: (0, 0, 0),
		header_color: (115, 26, 26),
		body_color: (0, 0, 0),
		table_title_color: (0, 0, 0),
		table_body_color: (0, 0, 0)
	};
	// Parameters for determining the size of the page and the text margins on the page
	let page_size_options = PageSizeOptions::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
	// Parameters for determining page number behavior
	let page_number_options = PageNumberOptions::new
	(HSide::Left, false, 1, FontVariant::Regular, 12.0, 5.0, (0, 0, 0), 5.0, 4.0).unwrap();
	// File path to the background image
	let background_path = "img/parchment.jpg";
	// Image transform data for the background image
	let background_transform = ImageTransform
	{
		translate_x: Some(Mm(0.0)),
		translate_y: Some(Mm(0.0)),
		scale_x: Some(1.95),
		scale_y: Some(2.125),
		..Default::default()
	};
	// Parameters for table margins / padding and off-row color / scaling
	let table_options = TableOptions::new(10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
	// Create the spellbook
	let (doc, _) = create_spellbook
	(
		spellbook_name,
		spell_list,
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((background_path, background_transform)),
		table_options
	).unwrap();
	// Save the spellbook to a file
	let _ = save_spellbook(doc, "Tasha's Cauldron of Everything Spells.pdf").unwrap();
}

// Create a spellbook with every spell from the strixhaven: a curriculum of chaos source book
#[test]
fn strixhaven()
{
	// Spellbook's name
	let spellbook_name =
	"Every Sepll in the Dungeons & Dragons 5th Edition Source Material Book \"Strixhaven: A Curriculum of Chaos\"";
	// List of every spell in this folder
	let spell_list = get_all_spells_in_folder("spells/strixhaven").unwrap();
	// File paths to the fonts needed
	let font_paths = FontPaths
	{
		regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
		bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
		italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
		bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
	};
	// Parameters for determining font sizes
	let font_sizes = FontSizes::new(32.0, 24.0, 12.0, 16.0, 12.0).unwrap();
	// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
	let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
	// Parameters for determining tab and newline sizes
	let spacing_options = SpacingOptions::new(7.5, 12.0, 8.0, 5.0, 6.4, 5.0).unwrap();
	// Colors for each type of text
	let text_colors = TextColors
	{
		title_color: (0, 0, 0),
		header_color: (115, 26, 26),
		body_color: (0, 0, 0),
		table_title_color: (0, 0, 0),
		table_body_color: (0, 0, 0)
	};
	// Parameters for determining the size of the page and the text margins on the page
	let page_size_options = PageSizeOptions::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
	// Parameters for determining page number behavior
	let page_number_options = PageNumberOptions::new
	(HSide::Left, false, 1, FontVariant::Regular, 12.0, 5.0, (0, 0, 0), 5.0, 4.0).unwrap();
	// File path to the background image
	let background_path = "img/parchment.jpg";
	// Image transform data for the background image
	let background_transform = ImageTransform
	{
		translate_x: Some(Mm(0.0)),
		translate_y: Some(Mm(0.0)),
		scale_x: Some(1.95),
		scale_y: Some(2.125),
		..Default::default()
	};
	// Parameters for table margins / padding and off-row color / scaling
	let table_options = TableOptions::new(10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
	// Create the spellbook
	let (doc, _) = create_spellbook
	(
		spellbook_name,
		spell_list,
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((background_path, background_transform)),
		table_options
	).unwrap();
	// Save the spellbook to a file
	let _ = save_spellbook(doc, "Strixhaven A Curriculum of Chaos Spells.pdf").unwrap();
}

// Stress testing the text formatting
#[test]
fn necronomicon()
{
	// Spellbook's name
	let spellbook_name =
	"THE NECROBOMBINOMICON AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A";
	// List of every spell in the stress test folder
	let spell_list = get_all_spells_in_folder("spells/necronomicon").unwrap();
	// File paths to the fonts needed
	let font_paths = FontPaths
	{
		regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
		bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
		italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
		bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
	};
	// Parameters for determining font sizes
	let font_sizes = FontSizes::new(32.0, 24.0, 12.0, 16.0, 12.0).unwrap();
	// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
	let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
	// Parameters for determining tab and newline sizes
	let spacing_options = SpacingOptions::new(7.5, 12.0, 8.0, 5.0, 6.4, 5.0).unwrap();
	// Colors for each type of text
	let text_colors = TextColors
	{
		title_color: (0, 0, 0),
		header_color: (115, 26, 26),
		body_color: (0, 0, 0),
		table_title_color: (0, 0, 0),
		table_body_color: (0, 0, 0)
	};
	// Parameters for determining the size of the page and the text margins on the page
	let page_size_options = PageSizeOptions::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
	// Parameters for determining page number behavior
	let page_number_options = PageNumberOptions::new
	(HSide::Left, false, 1, FontVariant::Regular, 12.0, 5.0, (0, 0, 0), 5.0, 4.0).unwrap();
	// File path to the background image
	let background_path = "img/parchment.jpg";
	// Image transform data for the background image
	let background_transform = ImageTransform
	{
		translate_x: Some(Mm(0.0)),
		translate_y: Some(Mm(0.0)),
		scale_x: Some(1.95),
		scale_y: Some(2.125),
		..Default::default()
	};
	// Parameters for table margins / padding and off-row color / scaling
	let table_options = TableOptions::new(10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
	// Create the spellbook
	let (doc, _) = create_spellbook
	(
		spellbook_name,
		spell_list,
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((background_path, background_transform)),
		table_options
	).unwrap();
	// Save the spellbook to a file
	let _ = save_spellbook(doc, "NECRONOMICON.pdf").unwrap();
}

// For creating spellbooks for myself and friends while I work on creating a ui to use this library
// #[test]
// fn personal_spellbook()
// {
// 	// Spellbook's name
// 	let spellbook_name = "A Spellcaster's Spellbook";
// 	// Vec of spells that will be added to spellbook
// 	let mut spell_list = Vec::new();
// 	// Vec of paths to spell files that will be read from
// 	let spell_paths = vec!
// 	[
// 		"spells/players_handbook_2014/prestidigitation.spell",
// 		"spells/players_handbook_2014/mending.spell",
// 		"spells/players_handbook_2014/mage_hand.spell",
// 		"spells/players_handbook_2014/fire_bolt.spell",
// 		"spells/strixhaven/silvery_barbs.spell",
// 		"spells/players_handbook_2014/color_spray.spell",
// 		"spells/players_handbook_2014/magic_missile.spell",
// 		"spells/xanathars_guide_to_everything/ice_knife.spell",
// 		"spells/players_handbook_2014/mage_armor.spell",
// 		"spells/players_handbook_2014/unseen_servant.spell",
// 		"spells/players_handbook_2014/detect_magic.spell",
// 		"spells/players_handbook_2014/alarm.spell",
// 		"spells/players_handbook_2014/cloud_of_daggers.spell",
// 		"spells/players_handbook_2014/scorching_ray.spell"
// 	];
// 	// Attempt to loop through each spell file and convert it into a spell struct
// 	for path in spell_paths
// 	{
// 		println!("{}", path);
// 		// Convert spell file into spell struct and add it to spell_list vec
// 		spell_list.push(spells::Spell::from_file(path).unwrap());
// 	}
// 	// File paths to the fonts needed
//	let font_paths = FontPaths
//	{
//		regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
//		bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
//		italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
//		bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
//	};
//	// Parameters for determining font sizes
//	let font_sizes = FontSizes::new(32.0, 24.0, 12.0, 16.0, 12.0).unwrap();
//	// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
//	let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515).unwrap();
//	// Parameters for determining tab and newline sizes
//	let spacing_options = SpacingOptions::new(7.5, 12.0, 8.0, 5.0, 6.4, 5.0).unwrap();
//	// Colors for each type of text
//	let text_colors = TextColors
//	{
//		title_color: (0, 0, 0),
//		header_color: (115, 26, 26),
//		body_color: (0, 0, 0),
//		table_title_color: (0, 0, 0),
//		table_body_color: (0, 0, 0)
//	};
//	// Parameters for determining the size of the page and the text margins on the page
//	let page_size_options = PageSizeOptions::new(210.0, 297.0, 10.0, 10.0, 10.0, 10.0).unwrap();
//	// Parameters for determining page number behavior
//	let page_number_options = PageNumberOptions::new
//	(HSide::Left, false, 1, FontVariant::Regular, 12.0, 5.0, (0, 0, 0), 5.0, 4.0).unwrap();
//	// File path to the background image
//	let background_path = "img/parchment.jpg";
//	// Image transform data for the background image
//	let background_transform = ImageTransform
//	{
//		translate_x: Some(Mm(0.0)),
//		translate_y: Some(Mm(0.0)),
//		scale_x: Some(1.95),
//		scale_y: Some(2.125),
//		..Default::default()
//	};
//	// Parameters for table margins / padding and off-row color / scaling
//	let table_options = TableOptions::new(10.0, 8.0, 4.0, 12.0, 0.1075, 4.0, (213, 209, 224)).unwrap();
	// Create the spellbook
//	let (doc, _) = create_spellbook
//	(
//		spellbook_name,
//		spell_list,
//		font_paths,
//		font_sizes,
//		font_scalars,
//		spacing_options,
//		text_colors,
//		page_size_options,
//		Some(page_number_options),
//		Some((background_path, background_transform)),
//		table_options
//	).unwrap();
//	// Save the spellbook to a file
//	let _ = save_spellbook(doc, "Spellbook.pdf").unwrap();
// }

// #[test]
// // Creates json files for every existing spell file except the spells in the necronomicon and test folders
// fn convert_to_json()
// {
// 	let spell_folders =
//	[
//		"spells/players_handbook_2014",
//		"spells/strixhaven",
//		"spells/tashas_cauldron_of_everything",
//		"spells/xanathars_guide_to_everything"
//	];
// 	for folder in spell_folders
// 	{
// 		// Gets a list of every file in the folder
// 		let file_paths = fs::read_dir(folder).unwrap();
// 		// Loop through each file in the folder
// 		for file_path in file_paths
// 		{
// 			// Attempt to get a path to the file
// 			let file_name = file_path.unwrap().path();
// 			let file_name = file_name.to_str().unwrap();
// 			// If the file is a spell file
// 			if file_name.ends_with(".spell")
// 			{
// 				let spell = spells::Spell::from_file(file_name).unwrap();
// 				let mut json_file_name = String::from(file_name);
// 				json_file_name.truncate(json_file_name.len() - 5);
// 				json_file_name += "json";
// 				let _ = spell.to_json_file(&json_file_name, false);
// 			}
// 		}
// 	}
// }