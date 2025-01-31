//! Library for making pdf documents of 5th edition D&D spells that are formatted like D&D source books.
//!
//! See repository for documentation on spell files.
//!
//! Repository at <https://github.com/ChandlerJayCalkins/dnd_spellbook_maker>.

pub mod spells;
mod spellbook_options;
mod spellbook_gen_types;
mod spellbook_writer;
#[cfg(test)]
mod tests;

pub mod utils;

pub use spellbook_options::*;
pub use utils::*;

// TODO
//	1. Make it so multiple tables / stat blocks can be placed next to each other horizontally
//	2. Make it so spells can have stat blocks in them
//		- Animate Dead
//		- Animate Objects
//		- Awaken
//		- Create Undead
//		- Find Steed
//		- Finger of Death
//		- Giant Insect
//		- Phantom Steed
//		- Summon Aberration
//		- Summon Beast
//		- Summon Celestial
//		- Summon Construct
//		- Summon Dragon
//		- Summon Elemental
//		- Summon Fey
//		- Summon Fiend
//		- Summon Undead
