//! A deck of Uno cards with shuffle operations and a quality evaluator.

use crate::card::{Action, Card, Color};

// ─── ShuffleScore ─────────────────────────────────────────────────────────────

/// The result of evaluating how well a deck has been shuffled.
///
/// Returned by [`Cards::is_shuffled_properly`].
#[derive(Debug, Clone)]
pub struct ShuffleScore {
    /// Per-card window scores.
    ///
    /// Each value reflects the raw power concentration in the 7-card window
    /// centred on that card. Lower raw values mean the region is less clustered.
    pub scores: Vec<i32>,

    /// Overall shuffle quality.
    ///
    /// Computed as `Σ (ideal − raw)` across all windows, where `ideal` is
    /// the score a perfectly uniform deck would produce.
    ///
    /// - **Positive** → well shuffled (power spread evenly).
    /// - **Negative** → poorly shuffled (high-power cards are clustered).
    /// - **Zero**     → exactly at the uniform baseline.
    pub quality: i32,
}

// ─── Cards ────────────────────────────────────────────────────────────────────

/// An ordered collection of [`Card`]s representing a deck.
///
/// # Example
/// ```no_run
/// use card_shuffling::prelude::*;
///
/// let mut deck = Cards::from_file("uno_nomercy.txt");
/// println!("Loaded {} cards", deck.len());
///
/// deck.riffle_shuffle();
/// let score = deck.is_shuffled_properly();
/// println!("Quality: {}", score.quality);
/// ```
#[derive(Debug, Clone, Default)]
pub struct Cards {
    /// The ordered list of cards in this deck.
    pub cards: Vec<Card>,
}

impl Cards {
    /// Creates a deck of `size` placeholder cards (Yellow `Number(0)`).
    ///
    /// Useful as a pre-allocated buffer before filling the deck manually.
    pub fn new(size: usize) -> Self {
        Cards {
            cards: (0..size)
                .map(|_| Card::new(Color::Yellow, Action::Number(0)))
                .collect(),
        }
    }

    /// Creates an empty deck.
    ///
    /// Equivalent to `Cards::default()`.
    pub fn empty() -> Self {
        Cards::default()
    }

    /// Creates a [`Cards`] directly from an existing [`Vec<Card>`].
    pub fn from_cards(cards: Vec<Card>) -> Self {
        Cards { cards }
    }

    /// Loads a deck from a text file.
    ///
    /// Each non-empty line must have the format `<action> <color> <count>`,
    /// for example: `0 yellow 2`. Invalid or blank lines are silently skipped.
    ///
    /// Returns an empty deck if the file cannot be read.
    pub fn from_file(filename: &str) -> Self {
        if let Ok(contents) = std::fs::read_to_string(filename) {
            let cards = contents
                .lines()
                .flat_map(|line| {
                    let parts = line.split_whitespace().collect::<Vec<&str>>();
                    if parts.len() != 3 {
                        // Skip empty or invalid lines
                        return vec![].into_iter();
                    }
                    // Format: action color count  (e.g. `0 yellow 2`)
                    let action = Action::from_string(parts[0]);
                    let color  = Color::from_string(parts[1]);
                    let count: usize = parts[2].parse().unwrap_or(0);
                    vec![Card::new(color, action); count].into_iter()
                })
                .collect();
            Cards { cards }
        } else {
            Cards::empty()
        }
    }

    /// Returns the number of cards in the deck.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Returns `true` if the deck contains no cards.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Returns an iterator over the cards in the deck.
    pub fn iter(&self) -> std::slice::Iter<'_, Card> {
        self.cards.iter()
    }

    // ─── Shuffle operations ───────────────────────────────────────────────────

    /// Splits the deck in half and swaps the two halves.
    pub fn middle_split(&mut self) {
        let mid = self.cards.len() / 2;
        self.cards.rotate_left(mid);
    }

    /// Splits the deck at index `i` and swaps the two resulting parts.
    ///
    /// Does nothing if `i` is out of bounds.
    pub fn split_at(&mut self, i: usize) {
        if i <= self.cards.len() {
            self.cards.rotate_left(i);
        }
    }

    /// Moves the cards in the range `i..j` to the front of the deck.
    ///
    /// Does nothing if the range is invalid.
    pub fn take_from_middle(&mut self, i: usize, j: usize) {
        if j <= i || j > self.cards.len() {
            return;
        }
        self.cards[0..j].rotate_right(j - i);
    }

    /// Riffle shuffles the entire deck once.
    ///
    /// Splits the deck in half and interleaves the two halves together.
    pub fn riffle_shuffle(&mut self) {
        Self::riffle(&mut self.cards);
    }

    /// Riffle shuffles each half of the deck independently, then leaves them
    /// in place.
    ///
    /// Can be used as a preparation step before a full [`riffle_shuffle`](Self::riffle_shuffle).
    pub fn double_riffle_shuffle(&mut self) {
        let mid = self.cards.len() / 2;
        Self::riffle(&mut self.cards[0..mid]);
        Self::riffle(&mut self.cards[mid..]);
    }

    // ─── Private helpers ──────────────────────────────────────────────────────

    /// Riffle-shuffles an arbitrary card slice in-place.
    ///
    /// Splits the slice in half, reverses each half, then interleaves them.
    fn riffle(cards: &mut [Card]) {
        let mid = cards.len() / 2;
        let mut left = cards[..mid].to_vec();
        left.reverse();
        let mut right = cards[mid..].to_vec();
        right.reverse();

        for i in 0..cards.len() {
            if i % 2 == 0 {
                cards[i] = left.pop().unwrap();
            } else {
                cards[i] = right.pop().unwrap();
            }
        }
    }

    /// Returns the "power" of a card, used for shuffle evaluation.
    ///
    /// Higher power = more impactful card. Wild cards receive a +2 bonus.
    fn card_power(card: &Card) -> i32 {
        let mut power = match card.get_action() {
            Action::Number(_)                                          => 1,
            Action::Skip | Action::Reverse                             => 2,
            Action::DrawTwo | Action::SkipAll | Action::DiscardAll     => 3,
            Action::DrawFour                                           => 4,
            Action::ReverseDrawFour                                    => 5,
            Action::DrawSix | Action::ColorRoulette                    => 6,
            Action::DrawTen                                            => 8,
        };
        if card.get_color() == Color::Wild {
            power += 2;
        }
        power
    }

    /// Scores the power concentration in a `window`-sized region centred at `index`.
    ///
    /// Formula: `(sum_of_powers)² / window`. Higher = more clustered.
    fn evaluate_cards_in_window(cards: &[Card], index: usize, window: usize) -> i32 {
        let start = index.saturating_sub(window / 2);
        let end   = (index + window / 2 + 1).min(cards.len());

        let mut score = 0i32;
        for card in &cards[start..end] {
            score += Self::card_power(card);
            if card.get_action() == Action::DiscardAll {
                let color = card.get_color();
                score += cards.iter().filter(|c| c.get_color() == color).count() as i32 * 5;
            }
        }

        score.pow(2) / window as i32
    }

    // ─── Evaluation ───────────────────────────────────────────────────────────

    /// Evaluates how well this deck has been shuffled.
    ///
    /// Returns a [`ShuffleScore`] with per-card window scores and an overall
    /// quality value.
    ///
    /// **How the quality is computed**:
    /// The deck-wide mean card power defines a *uniform baseline*. A window
    /// whose cards all had exactly `mean_power` would score:
    ///
    /// ```text
    /// ideal = mean_power² × window_size
    /// quality = Σ (ideal − raw_window_score)
    /// ```
    ///
    /// Positive quality means the actual scores are below the baseline (good —
    /// power is spread out). Negative quality means clustering.
    ///
    /// # Example
    /// ```no_run
    /// use card_shuffling::prelude::*;
    ///
    /// let mut deck = Cards::from_file("uno_nomercy.txt");
    /// deck.riffle_shuffle();
    /// let score = deck.is_shuffled_properly();
    /// println!("Quality: {}", score.quality);
    /// ```
    pub fn is_shuffled_properly(&self) -> ShuffleScore {
        if self.cards.is_empty() {
            return ShuffleScore { scores: vec![], quality: 0 };
        }

        let window = 7usize;

        // Uniform baseline: the score a window would get if every card had
        // exactly the deck-average power.
        let total_power: i32 = self.cards.iter().map(Self::card_power).sum();
        let mean_power  = total_power / self.cards.len() as i32;
        let ideal       = mean_power * mean_power * window as i32;

        let mut scores      = Vec::with_capacity(self.cards.len());
        let mut total_score = 0i32;

        for i in 0..self.cards.len() {
            let raw = Self::evaluate_cards_in_window(&self.cards, i, window);
            scores.push(raw);
            total_score += ideal - raw; // positive when window is below ideal (good)
        }

        ShuffleScore { scores, quality: total_score }
    }
}

// ─── Trait implementations ────────────────────────────────────────────────────

impl From<Vec<Card>> for Cards {
    fn from(cards: Vec<Card>) -> Self {
        Cards { cards }
    }
}

impl IntoIterator for Cards {
    type Item = Card;
    type IntoIter = std::vec::IntoIter<Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl<'a> IntoIterator for &'a Cards {
    type Item = &'a Card;
    type IntoIter = std::slice::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.iter()
    }
}