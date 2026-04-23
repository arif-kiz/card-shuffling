//! Example: loading and shuffling an Uno No Mercy deck.
//!
//! Run with:
//! ```sh
//! cargo run
//! ```
//! Place `uno_nomercy.txt` in the project root (next to `Cargo.toml`).

mod card_examples;

use card_examples::{UnoColor, UnoNoMercyAction};
use card_shuffling::prelude::*;
use rand::rngs::ThreadRng;

/// Prints a one-line status report for the deck.
fn status(label: &str, deck: &Cards<UnoNoMercyAction, UnoColor, ThreadRng>) {
    let n = deck.len();
    let score = deck.is_shuffled_properly();
    let head  = &deck.cards[..5.min(n)];
    let tail  = &deck.cards[n.saturating_sub(5)..];
    println!("{label:<26} | quality: {:>8} | head: {head:?}", score.quality,);
    println!("{:26} |                   | tail: {tail:?}", "");
    println!();
}

/// Shuffles `deck` repeatedly until quality stops improving.
///
/// Stops when no new best is found within `patience` consecutive shuffles.
/// Only compiled when the `grind` feature is enabled:
/// ```sh
/// cargo run --release --features grind
/// ```
#[cfg(feature = "grind")]
fn grind_until_positive(deck: &mut Cards<UnoNoMercyAction, UnoColor, ThreadRng>) {
    use std::time::Instant;

    const PATIENCE: u64 = 10_000;

    println!("\n── grind mode (patience = {PATIENCE}) ──────────────────────────");
    let start    = Instant::now();
    let mut itr  = 0u64;
    let mut best = deck.is_shuffled_properly().quality;
    let mut since_best = 0u64;

    loop {
        deck.double_riffle_shuffle();
        if itr.is_multiple_of(5) {
            deck.randomize()
        }
        itr += 1;
        since_best += 1;

        let quality = deck.is_shuffled_properly().quality;

        if quality > best {
            best = quality;
            since_best = 0;
            println!("  iter {itr:>8} | ✨ new best: {best:>6}");
        }

        if itr.is_multiple_of(10_000) {
            println!("  iter {itr:>8} | quality: {quality:>6} | best: {best:>6} | no-improve: {since_best}");
        }

        if since_best >= PATIENCE {
            println!(
                "\n  🏁  Best quality {best} reached after {itr} shuffles ({:.2?})",
                start.elapsed()
            );
            status("Final deck", deck);
            break;
        }
    }
}

fn main() {
    let mut deck = Cards::from_file("uno_nomercy.txt", rand::rng());
    println!("Loaded {} cards\n", deck.len());

    status("Initial (unshuffled)", &deck);

    deck.middle_split();
    status("After middle_split", &deck);

    deck.split_at(50);
    status("After split_at(50)", &deck);

    deck.take_from_middle(20, 80);
    status("After take_from_middle", &deck);

    deck.riffle_shuffle();
    status("After riffle_shuffle", &deck);

    deck.double_riffle_shuffle();
    status("After double_riffle", &deck);

    #[cfg(feature = "grind")]
    grind_until_positive(&mut deck);

    // Iterate directly over the deck (IntoIterator for &Cards)
    let wild_indices = deck.get_wild_card_indices();
    println!("Wild cards in deck: {}", wild_indices.len());
    println!("Wild card indices: {:?}", wild_indices);

    // Consume the deck into individual cards (IntoIterator for Cards)
    let high_power_indices = deck.get_high_draw_card_indices();
    println!("High-draw cards: {}", high_power_indices.len());
    println!("High-draw card indices: {:?}", high_power_indices);
}