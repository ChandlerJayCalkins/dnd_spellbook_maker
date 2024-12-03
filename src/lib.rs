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
// 1. Make it so SpellbookWriter can change to a new PDF document / spellbook using the same other settings.
// This might make the `generate_spellbook` function redundant, but that could be a good thing.
// 2. Make it so SpellbookWriters immediately create the title page when constructed or upon using a new pdf doc.
// 3. Finish writing SpellbookWriter constrcutor and then write a test to make sure it works.
// 4. Rewrite `write_spell_description` function to be combined with `write_textbox` so tokens get parsed and written
// at the same time. Make it so text gets written when it either switches fonts or gets too long to fit on the page.
