# card_shuffling

A Rust library for building, shuffling, and evaluating **Uno No Mercy** card decks.

It provides the core card types, several shuffle operations, and a heuristic quality
evaluator that tells you — with a signed score — how well a deck has been shuffled.

---

## Features

- **Card types** — `Color`, `Action`, and `Card` covering the full Uno No Mercy card set
- **Deck operations** — load from a text file or build from scratch
- **Shuffle algorithms**
  - Middle split
  - Split at an arbitrary index
  - Riffle shuffle
  - Double riffle shuffle
  - Take from middle
- **Shuffle quality score** — a signed integer where positive means well shuffled
  and negative means high-power cards are clustered

---

## Project structure

```
card_shuffling/
├── src/
│   ├── lib.rs        # Crate root + prelude
│   ├── card.rs       # Color, Action, Card types
│   ├── cards.rs      # Cards deck + ShuffleScore
│   └── main.rs       # Usage example (binary)
├── uno_nomercy.txt   # Deck definition file
└── Cargo.toml
```

---

## Quick start

```rust
use card_shuffling::prelude::*;

fn main() {
    // Load a deck from a definition file
    let mut deck = Cards::from_file("uno_nomercy.txt");
    println!("Loaded {} cards", deck.len());

    // Check quality before shuffling (expect negative — cards are sorted)
    let before = deck.is_shuffled_properly().quality;
    println!("Before: {before}");

    // Shuffle
    deck.riffle_shuffle();

    // Check quality after
    let after = deck.is_shuffled_properly().quality;
    println!("After:  {after}");
}
```

---

## Deck definition file format

Each line describes one card type and how many copies exist in the deck:

```
<action> <color> <count>
```

| Field    | Example values |
|----------|----------------|
| `action` | `0`–`9`, `skip`, `reverse`, `+2`, `skip_all`, `+4`, `discard_all`, `reverse_+4`, `+6`, `+10`, `color_roulette` |
| `color`  | `yellow`, `red`, `green`, `blue`, `wild` |
| `count`  | any positive integer |

**Example** (`uno_nomercy.txt`):
```
0 yellow 2
skip red 3
reverse_+4 wild 8
color_roulette wild 8
```

Blank or malformed lines are silently skipped.

---

## Shuffle operations

| Method | Description |
|--------|-------------|
| `middle_split()` | Cuts the deck exactly in half and swaps the halves |
| `split_at(i)` | Cuts at index `i` and swaps the two parts |
| `riffle_shuffle()` | Classic riffle: splits in half and interleaves |
| `double_riffle_shuffle()` | Riffles the top half and bottom half independently |
| `take_from_middle(i, j)` | Moves cards `i..j` to the front of the deck |

---

## Shuffle quality

`is_shuffled_properly()` returns a [`ShuffleScore`]:

```rust
pub struct ShuffleScore {
    pub scores:  Vec<i32>, // per-card window score (raw power concentration)
    pub quality: i32,      // overall signed quality
}
```

**How quality is computed:**

For each card, a window of 7 cards centred on it is evaluated. The window's
*raw score* is `(sum of card powers)² / 7`. A higher raw score means the
high-power cards in that region are more clustered.

The *ideal* score — what you'd get if every card had exactly the deck-average
power — is computed once:

```
ideal = mean_power² × window_size
quality = Σ (ideal − raw)  over all cards
```

| Quality | Meaning |
|---------|---------|
| > 0 | Well shuffled — power is spread evenly |
| = 0 | Exactly at the uniform baseline |
| < 0 | Poorly shuffled — high-power cards are clustered |

---

## Card power reference

Power values are used only internally by the shuffle evaluator.

| Action | Base power | Wild bonus |
|--------|-----------|------------|
| Number (0–9) | 1 | +2 |
| Skip, Reverse | 2 | — |
| Draw Two, Skip All, Discard All | 3 | — |
| Draw Four | 4 | — |
| Reverse Draw Four | 5 | — |
| Draw Six, Color Roulette | 6 | +2 |
| Draw Ten | 8 | +2 |

---

## Prelude

Import everything you need in one line:

```rust
use card_shuffling::prelude::*;
// brings in: Color, Action, Card, Cards, ShuffleScore
```

---

## License

MIT