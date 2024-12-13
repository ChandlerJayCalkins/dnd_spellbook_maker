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
// 1. Rewrite `write_textbox`
// 2. Clean up the `write_textbox` method in `SpellbookWriter`
// 3. Add a `TableTag` variant to the `Token` enum and make it so centered textboxes can have tables.
// 4. Add table parsing and writing
// 5. Clean up `spells` module and remove support for legacy spell files (since they can't be parsed anymore)
// 6. Add all 2024 Player's Handbook spells
// 7. Make it so spells can have stat blocks in them
// 8. Make it so multiple tables can be placed next to each other horizontally
