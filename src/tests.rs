//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//	Tests
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;
use std::path::Path;

use crate::utils::*;

// Returns default values to pass to `create_spellbook()`
fn default_spellbook_options() ->
(
	FontPaths,
	FontSizes,
	FontScalars,
	SpacingOptions,
	TextColorOptions,
	PageSizeOptions,
	PageNumberOptions,
	String,
	XObjectTransform,
	TableOptions
)
{
	// File paths to the fonts needed
	let font_paths = FontPaths
	{
		regular: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Regular.otf"),
		bold: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Bold.otf"),
		italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-Italic.otf"),
		bold_italic: String::from("fonts/TeX-Gyre-Bonum/TeX-Gyre-Bonum-BoldItalic.otf")
	};
	// Parameters for determining font sizes
	let font_sizes = FontSizes::new(32.0, 24.0, 12.0, 16.0, 12.0)
		.expect("Failed to create font sizes.");
	// Scalars used to convert the size of fonts from rusttype font units to printpdf millimeters (Mm)
	let font_scalars = FontScalars::new(0.475, 0.51, 0.48, 0.515)
		.expect("Failed to create font scalars.");
	// Parameters for determining tab and newline sizes
	let spacing_options = SpacingOptions::new(7.5, 12.0, 8.0, 5.0, 6.4, 5.0)
		.expect("Failed to create spacing options.");
	// Colors for each type of text
	let text_colors = TextColorOptions
	{
		title_color: (0, 0, 0),
		header_color: (115, 26, 26),
		body_color: (0, 0, 0),
		table_title_color: (0, 0, 0),
		table_body_color: (0, 0, 0)
	};
	// Parameters for determining the size of the page and the text margins on the page
	let page_size_options = PageSizeOptions::new(210.0, 297.0, 10.0, 10.0, 6.0, 10.0)
		.expect("Failed to create page size options.");
	// Parameters for determining page number behavior
	let page_number_options = PageNumberOptions::new
	(HSide::Left, false, 1, FontVariant::Regular, 12.0, 5.0, (0, 0, 0), 5.0, 4.0)
		.expect("Failed to create page number options.");
	// File path to the background image
	let background_path = String::from("img/parchment.jpg");
	// Image transform data for the background image
	let background_transform = XObjectTransform
	{
		scale_x: Some(1.95),
		scale_y: Some(2.125),
		..Default::default()
	};
	// Parameters for table margins / padding and off-row color / scaling
	// 2014 Player's Handbook off-row RGB: (213, 209, 224)
	let table_options = TableOptions::new(10.0, 8.0, 4.0, 12.0, 0.12, 4.4, (215, 223, 224))
		.expect("Failed to create table options.");
	// Return all options
	(
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		page_number_options,
		background_path,
		background_transform,
		table_options
	)
}

// Create a spellbook with every spell from the 2024 player's handbook
#[test]
fn players_handbook_2024()
{
	// Spellbook's name
	let spellbook_name =
	"Every Sepll in the 2024 Dungeons & Dragons 5th Edition Player's Handbook";
	// List of every spell in this folder
	let spell_list = get_all_spells_in_folder("spells/players_handbook_2024")
		.expect("Failed to collect spells from folder.");
	// Get default spellbook options
	let
	(
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		page_number_options,
		background_path,
		background_transform,
		table_options
	) = default_spellbook_options();
	// Create the spellbook
	let doc = create_spellbook
	(
		spellbook_name,
		&spell_list,
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((&background_path, background_transform)),
		table_options
	).unwrap();
	// Save the spellbook to a file
	let _ = save_spellbook(doc, "Player's Handbook 2024 Spells.pdf").unwrap();
}

// Create 2 spellbooks that combined contain every spell in the 2024 player's handbook
// Use this test instead of the players_handbook_2024 test if you are unable to view pdf documents larger than 2GB
#[test]
fn players_handbook_2024_split()
{
	// Spellbook's names
	let spellbook_name_1 =
	"Every Sepll in the 2024 Dungeons & Dragons 5th Edition Player's Handbook: Part 1";
	let spellbook_name_2 =
	"Every Sepll in the 2024 Dungeons & Dragons 5th Edition Player's Handbook: Part 2";
	// List of every spell in this folder
	let spell_list = get_all_spells_in_folder("spells/players_handbook_2024")
		.expect("Failed to collect spells from folder.");
	// Split that vec into 2 vecs
	let spell_list_1 = spell_list[..spell_list.len()/2].to_vec();
	let spell_list_2 = spell_list[spell_list.len()/2..].to_vec();
	// Get default spellbook options
	let
	(
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		page_number_options,
		background_path,
		background_transform,
		table_options
	) = default_spellbook_options();
	// Create a spellbook with the first half of the spells
	let doc_1 = create_spellbook
	(
		spellbook_name_1,
		&spell_list_1,
		font_paths.clone(),
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((&background_path, background_transform)),
		table_options
	).unwrap();
	// Save the first spellbook to a file
	let _ = save_spellbook(doc_1, "Player's Handbook 2024 Spells 1.pdf").unwrap();
	// Create a spellbook with the second half of the spells
	let doc_2 = create_spellbook
	(
		spellbook_name_2,
		&spell_list_2,
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((&background_path, background_transform)),
		table_options
	).unwrap();
	// Save the second spellbook to a file
	let _ = save_spellbook(doc_2, "Player's Handbook 2024 Spells 2.pdf").unwrap();
}

// Create a spellbook with every spell from the 2014 player's handbook
#[test]
fn players_handbook_2014()
{
	// Spellbook's name
	let spellbook_name = "Every Sepll in the 2014 Dungeons & Dragons 5th Edition Player's Handbook";
	// List of every spell in the player's handbook folder
	let spell_list = get_all_spells_in_folder("spells/players_handbook_2014")
		.expect("Failed to collect spells from folder.");
	// Get default spellbook options
	let
	(
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		page_number_options,
		background_path,
		background_transform,
		table_options
	) = default_spellbook_options();
	// Create the spellbook
	let doc = create_spellbook
	(
		spellbook_name,
		&spell_list,
		font_paths.clone(),
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((&background_path, background_transform)),
		table_options
	).unwrap();
	// Save the spellbook to a file
	let _ = save_spellbook(doc, "Player's Handbook 2024 Spells.pdf").unwrap();
}

// Creates 2 spellbooks that combined contain every spell from the 2014 player's handbook
// Use this test instead of the players_handbook_2014 test if you are unable to view pdf documents larger than 2GB
#[test]
fn players_handbook_2014_split()
{
	
	// Spellbook names
	let spellbook_name_1 =
	"Every Sepll in the 2014 Dungeons & Dragons 5th Edition Player's Handbook: Part 1";
	let spellbook_name_2 =
	"Every Sepll in the 2014 Dungeons & Dragons 5th Edition Player's Handbook: Part 2";
	// List of every spell in the player's handbook folder
	let spell_list = get_all_spells_in_folder("spells/players_handbook_2014")
		.expect("Failed to collect spells from folder.");
	// Split that vec into 2 vecs
	let spell_list_1 = spell_list[..spell_list.len()/2].to_vec();
	let spell_list_2 = spell_list[spell_list.len()/2..].to_vec();
	// Get default spellbook options
	let
	(
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		page_number_options,
		background_path,
		background_transform,
		table_options
	) = default_spellbook_options();
	// Create a spellbook with the first half of the spells
	let doc_1 = create_spellbook
	(
		spellbook_name_1,
		&spell_list_1,
		font_paths.clone(),
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((&background_path, background_transform)),
		table_options
	).unwrap();
	// Save the first spellbook to a file
	let _ = save_spellbook(doc_1, "Player's Handbook 2014 Spells 1.pdf").unwrap();
	// Create a spellbook with the second half of the spells
	let doc_2 = create_spellbook
	(
		spellbook_name_2,
		&spell_list_2,
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((&background_path, background_transform)),
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
	let spell_list = get_all_spells_in_folder("spells/xanathars_guide_to_everything")
		.expect("Failed to collect spells from folder.");
	// Get default spellbook options
	let
	(
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		page_number_options,
		background_path,
		background_transform,
		table_options
	) = default_spellbook_options();
	// Create the spellbook
	let doc = create_spellbook
	(
		spellbook_name,
		&spell_list,
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((&background_path, background_transform)),
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
	let spell_list = get_all_spells_in_folder("spells/tashas_cauldron_of_everything")
		.expect("Failed to collect spells from folder.");
	// Get default spellbook options
	let
	(
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		page_number_options,
		background_path,
		background_transform,
		table_options
	) = default_spellbook_options();
	// Create the spellbook
	let doc = create_spellbook
	(
		spellbook_name,
		&spell_list,
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((&background_path, background_transform)),
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
	let spell_list = get_all_spells_in_folder("spells/strixhaven")
		.expect("Failed to collect spells from folder.");
	// Get default spellbook options
	let
	(
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		page_number_options,
		background_path,
		background_transform,
		table_options
	) = default_spellbook_options();
	// Create the spellbook
	let doc = create_spellbook
	(
		spellbook_name,
		&spell_list,
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((&background_path, background_transform)),
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
	"THE NECROBOMBINOMICON AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A <bi> A A A A A A AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A \\<r> A A A A A A AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A \\<i> A A A A A <i> A A A A A A A A A A A A A A A A A A A A A A <r> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A";
	// List of every spell in the stress test folder
	let spell_list = get_all_spells_in_folder("spells/necronomicon")
		.expect("Failed to collect spells from folder.");
	// Get default spellbook options
	let
	(
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		_,
		_,
		_,
		table_options
	) = default_spellbook_options();
	// Parameters for determining page number behavior
	let page_number_options = PageNumberOptions::new
	(HSide::Left, true, 1, FontVariant::Regular, 12.0, 5.0, (0, 0, 0), 5.0, 4.0)
		.expect("Failed to create page number options.");
	// Create the spellbook
	let doc = create_spellbook
	(
		spellbook_name,
		&spell_list,
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		None,
		table_options
	).unwrap();
	// Save the spellbook to a file
	let _ = save_spellbook(doc, "NECRONOMICON.pdf").unwrap();
}

// For creating spellbooks for myself and friends while I work on creating a ui to use this library
#[test]
fn personal_spellbook()
{
	// Spellbook's name
	let spellbook_name = "A Spellcaster's Spellbook";
	// Vec of spells that will be added to spellbook
	let mut spell_list = Vec::new();
	// Vec of paths to spell files that will be read from
	let spell_paths = vec!
	[
		"spells/players_handbook_2024/prestidigitation.json",
		"spells/players_handbook_2024/mage_hand.json",
		"spells/players_handbook_2024/mending.json",
		"spells/players_handbook_2024/minor_illusion.json",
		"spells/players_handbook_2024/fire_bolt.json",
		"spells/players_handbook_2024/ray_of_frost.json",
		"spells/players_handbook_2024/shocking_grasp.json",
		"spells/players_handbook_2024/acid_splash.json",
		"spells/players_handbook_2024/mage_armor.json",
		"spells/players_handbook_2024/magic_missile.json",
		"spells/players_handbook_2024/chromatic_orb.json",
		"spells/players_handbook_2024/silent_image.json",
		"spells/players_handbook_2024/sleep.json",
		"spells/players_handbook_2024/disguise_self.json",
		"spells/players_handbook_2024/identify.json",
		"spells/players_handbook_2024/detect_magic.json",
		"spells/players_handbook_2024/unseen_servant.json",
		"spells/players_handbook_2024/comprehend_languages.json",
		"spells/players_handbook_2024/misty_step.json",
		"spells/players_handbook_2024/invisibility.json",
		"spells/players_handbook_2024/cloud_of_daggers.json",
		"spells/players_handbook_2024/scorching_ray.json",
		"spells/players_handbook_2024/phantasmal_force.json",
		"spells/players_handbook_2024/detect_thoughts.json",
		"spells/players_handbook_2024/enhance_ability.json",
		"spells/players_handbook_2024/hypnotic_pattern.json",
		"spells/players_handbook_2024/fireball.json",
		"spells/players_handbook_2024/fly.json"
	];
	// Attempt to loop through each spell file and convert it into a spell struct
	for path in spell_paths
	{
		// Convert spell file into spell struct and add it to spell_list vec
		spell_list.push(spells::Spell::from_json_file(path)
			.expect(format!("Failed to load spell file {}", path).as_str()));
	}
	// Get default spellbook options
	let
	(
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		page_number_options,
		background_path,
		background_transform,
		table_options
	) = default_spellbook_options();
	// Create the spellbook
	let doc = create_spellbook
	(
		spellbook_name,
		&spell_list,
		font_paths,
		font_sizes,
		font_scalars,
		spacing_options,
		text_colors,
		page_size_options,
		Some(page_number_options),
		Some((&background_path, background_transform)),
		table_options
	).unwrap();
	// Save the spellbook to a file
	let _ = save_spellbook(doc, "Spellbook.pdf").unwrap();
}

// Makes sure that creating valid spell files works
#[test]
fn create_spell_files()
{
	// Path to hand-made spell files that are compared to generated spells
	let comparison_folder = String::from("spells/necronomicon/");

	// Create the spells (necronomicon spell duplicates)
	let hell_spell = spells::Spell
	{
		name: String::from("HELL SPELL AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A"),
		level: spells::SpellField::Custom(String::from("100TH-LEVEL")),
		school: spells::SpellField::Custom(String::from("SUPER NECROMANCY")),
		is_ritual: true,
		casting_time: spells::SpellField::Controlled(spells::CastingTime::Reaction(Some(String::from("THAT YOU TAKE WHEN YOU FEEL LIKE CASTING SPELLS AND DOING MAGIC AND AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A")))),
		range: spells::SpellField::Controlled(spells::Range::Yourself(Some(spells::Aoe::Cylinder(spells::Distance::Miles(63489), spells::Distance::Miles(49729))))),
		has_v_component: true,
		has_s_component: true,
		m_components: Some(String::from("UNLIMITED POWAHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H H")),
		duration: spells::SpellField::Controlled(spells::Duration::Years(57394, true)),
		description: String::from("<ib> CASTING SPELLS AND CONJURING ABOMINATIONS <b> AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA <r> THIS SPELL ISN'T FOR <i> weak underpowered feeble wizards -_-. <r> THIS SPELL IS FOR ONLY THE MOST POWERFUL OF ARCHMAGES AND NECROMANCERS WHO CAN WIELD THE MIGHTIEST OF <bi> ARCANE ENERGY <r> WITH THE FORTITUDE OF A <ib> MOUNTAIN. <b> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A\nA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A \\<r> A A A A A \\<b> A A A A A A A \\<i> A A A A A A A \\<bi> A A A A \\<ib> A A A A A \\\\<r> A A A A \\\\\\<b> A A A A \\\\\\\\<i> A A A A \\\\\\\\\\<bi> A A A A \\\\\\\\\\\\<ib> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A\n\\[table][1]\n\\\\[table][0]\n\\\\\\[table][1]\n\\\\\\\\[table][0]\nA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\\\\\\\\\\<r>AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\\<b>AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A <i> A A A A A A A A <b>AAAAA<r>AAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A AAAAA<b>AAAAA<i>AAAAA<bi>AAAAA<r>AAAAA<ib>AAAAA<r> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A\nA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A\n[table][0]\nMORE MAGIC SPELLS AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A\nA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A\nA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A\n[table][1]\nYOU CAN'T HANDLE THIS SPELL A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A\nA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A\nA A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A"),
		upcast_description: Some(String::from("HELL ON EARTH")),
		tables: vec!
		[
			spells::Table
			{
				title: String::from("A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A"),
				column_labels: vec![String::from("COLUMN OF CHAOS"), String::from("COLUMN OF NECROMANCY")],
				cells: vec!
				[
					vec!
					[
						String::from("A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A"),
						String::from("A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A")
					],
					vec!
					[
						String::from("B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B"),
						String::from("B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B"),
						String::from("B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B")
					],
					vec!
					[
						String::from("POWER"),
						String::from("WIZARDRY")
					],
					vec!
					[
						String::new(),
						String::from("C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C"),
						String::from("C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C")
					]
				]
			},
			spells::Table
			{
				title: String::from("THIS TABLE AGAIN A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A \\A \\\\A \\\\\\A \\<title> \\\\<title> \\\\\\<title> A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A"),
				column_labels: vec![String::from("COLUMN OF CHAOS"), String::from("COLUMN OF NECROMANCY")],
				cells: vec!
				[
					vec!
					[
						String::from("A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A"),
						String::from("A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A A")
					],
					vec!
					[
						String::from("B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B"),
						String::from("B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B"),
						String::from("B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B B")
					],
					vec!
					[
						String::from("POWER"),
						String::from("WIZARDRY")
					],
					vec!
					[
						String::new(),
						String::from("C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C"),
						String::from("C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C C")
					]
				]
			}
		]
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
- Scrunching has these effects
[table][0]
- Scrunch balls (balls produced from scrunching) can be thrown and do 1d6 bludgeoning damage on hit.
Scrunch ball funny lol."),
		upcast_description: None,
		tables: vec!
		[
			spells::Table
			{
				title: String::from("Scrunching Effects"),
				column_labels: vec![String::from("Target"), String::from("Effect")],
				cells: vec!
				[
					vec!
					[
						String::from("Creature"),
						String::from("Flesh Ball")
					],
					vec!
					[
						String::from("Object"),
						String::from("Ball of that object's material")
					],
					vec!
					[
						String::from("Creature not made of flesh"),
						String::from("Ball of that creature's material")
					]
				]
			}
		]
	};
	let the_ten_hells = spells::Spell
	{
		name: String::from("The Ten Hells"),
		level: spells::SpellField::Controlled(spells::Level::Level9),
		school: spells::SpellField::Controlled(spells::MagicSchool::Necromancy),
		is_ritual: true,
		casting_time: spells::SpellField::Controlled(spells::CastingTime::Actions(1)),
		range: spells::SpellField::Controlled(spells::Range::Yourself(Some(spells::Aoe::Sphere(spells::Distance::Feet(90))))),
		has_v_component: true,
		has_s_component: false,
		m_components: Some(String::from("the nail or claw of a creature from an evil plane")),
		duration: spells::SpellField::Controlled(spells::Duration::Instant),
		description: String::from("Choose any number of creatures made of tangible matter within range. Those creatures must all make a constitution saving throw against your spell save DC. All creatures that fail this saving throw get turned inside out, immediately die, and have their souls eternally damned to all nine hells simultaneously.
Creatures that succeed the saving throw take 20d4 scrunching damage."),
		upcast_description: None,
		tables: Vec::new()
	};

	// Create vec of test spells and their file names (without extension or path)
	let spell_list = vec![(hell_spell, "hell_spell"), (power_word_scrunch, "power_word_scrunch"), (the_ten_hells, "the_ten_hells")];
	// Test to make sure spell files can be created properly
	json_file_test(&spell_list, false, "spells/tests/", &comparison_folder);
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
