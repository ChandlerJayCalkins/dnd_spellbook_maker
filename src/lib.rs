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
// 1. Add table parsing and writing
// 2. Clean up `spells` module and remove support for legacy spell files (since they can't be parsed anymore)
// 3. Add all 2024 Player's Handbook spells
// 4. Make it so spells can have stat blocks in them
// 5. Make it so multiple tables can be placed next to each other horizontally
