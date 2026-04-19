//! Example: loading and shuffling an Uno No Mercy deck.
//!
//! Run with:
//! ```sh
//! cargo run
//! ```
//! Place `uno_nomercy.txt` in the project root (next to `Cargo.toml`).

use card_shuffling::prelude::*;

/// Prints a one-line status report for the deck.
fn status(label: &str, deck: &Cards) {
    let n = deck.len();
    let score = deck.is_shuffled_properly();
    let head  = &deck.cards[..5.min(n)];
    let tail  = &deck.cards[n.saturating_sub(5)..];
    println!("{label:<26} | quality: {:>8} | head: {head:?}", score.quality,);
    println!("{:26} |                   | tail: {tail:?}", "");
    println!();
}

fn main() {
    let mut deck = Cards::from_file("uno_nomercy.txt");
    println!("Loaded {} cards\n", deck.len());

    status("Initial (unshuffled)", &deck);

    deck.middle_split();
    status("After middle_split", &deck);

    deck.split_at(50);
    status("After split_at(50)", &deck);

    deck.riffle_shuffle();
    status("After riffle_shuffle", &deck);

    deck.double_riffle_shuffle();
    status("After double_riffle", &deck);

    deck.take_from_middle(20, 80);
    status("After take_from_middle", &deck);

    // Iterate directly over the deck (IntoIterator for &Cards)
    let wild_count = deck.iter().filter(|c| c.get_color() == Color::Wild).count();
    println!("Wild cards in deck: {wild_count}");

    // Consume the deck into individual cards (IntoIterator for Cards)
    let high_power: Vec<Card> = deck
        .into_iter()
        .filter(|c| matches!(c.get_action(), Action::DrawFour | Action::DrawSix | Action::DrawTen))
        .collect();
    println!("High-draw cards: {}", high_power.len());
    for card in &high_power {
        println!("  {card}");
    }
}