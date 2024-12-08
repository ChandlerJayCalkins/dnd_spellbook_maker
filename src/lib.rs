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

// TODO
// 1. Make it so single tokens that are too long get hyphenated using binary search of length of token
// 2. Add table parsing and writing
// 3. Add all 2024 Player's Handbook spells
// 4. Make it so spells can have stat blocks in them
