//! # `card_shuffling`
//!
//! A library for building, shuffling, and evaluating Uno No Mercy card decks.
//!
//! ## Quick start
//!
//! ```no_run
//! use card_shuffling::prelude::*;
//!
//! let mut deck = Cards::from_file("uno_nomercy.txt");
//! println!("Loaded {} cards", deck.len());
//!
//! let before = deck.is_shuffled_properly().quality;
//! deck.riffle_shuffle();
//! let after = deck.is_shuffled_properly().quality;
//! println!("Shuffle quality: {before} → {after}");
//! ```

// ─── Clippy configuration ─────────────────────────────────────────────────────

// Treat every Clippy warning as a hard error (mirrors `cargo clippy -- -D warnings`).
#![deny(clippy::all)]
// Opt-in to the extra pedantic lints that make library APIs cleaner.
#![warn(clippy::pedantic)]
// Useful restriction lints for a library crate.
#![warn(
    clippy::exhaustive_enums,       // remind us to add #[non_exhaustive] when needed
    clippy::exhaustive_structs,     // same for structs
    clippy::missing_errors_doc,     // every pub fn that returns Result needs docs
    clippy::missing_panics_doc,     // every pub fn that can panic needs docs
    clippy::must_use_candidate,     // flag pure fns whose return value is easy to ignore
    clippy::wildcard_imports,       // ban `use foo::*` except in prelude modules
)]
// Lints we intentionally silence for this crate.
#![allow(
    clippy::module_name_repetitions, // e.g. `cards::Cards` is fine here
)]

pub mod card;
pub mod cards;

/// Convenient re-exports for typical use.
///
/// ```no_run
/// use card_shuffling::prelude::*;
/// ```
pub mod prelude {
    pub use crate::card::{Action, Card, Color};
    pub use crate::cards::{Cards, ShuffleScore};
}
