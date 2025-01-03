//! Library for making pdf documents of spells that a 5th edition D&D character has.
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
//	1. Add all 2024 Player's Handbook spells
//	2. Make it so spells can have stat blocks in them
//		- Animate Objects
//		- Create Undead
//		- Find Steed
//		- Finger of Death
//	3. Make it so multiple tables / stat blocks can be placed next to each other horizontally
