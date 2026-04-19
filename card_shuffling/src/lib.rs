//! # card_shuffling
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
