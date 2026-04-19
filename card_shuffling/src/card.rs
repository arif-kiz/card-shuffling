//! Types representing individual Uno cards.
//!
//! This module provides [`Color`], [`Action`], and [`Card`] — the building
//! blocks used by [`crate::cards::Cards`] to form a deck.

use std::fmt;

/// The color of an Uno card.
///
/// `Wild` is used for cards that can be played on any color.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Yellow,
    Red,
    Green,
    Blue,
    /// Applies to wild-type cards that can be played on any color.
    Wild,
}

impl Color {
    /// Parses a [`Color`] from a lowercase string slice.
    ///
    /// # Panics
    /// Panics if `s` is not one of `"yellow"`, `"red"`, `"green"`, `"blue"`, `"wild"`.
    pub fn from_string(s: &str) -> Self {
        match s {
            "yellow" => Color::Yellow,
            "red"    => Color::Red,
            "green"  => Color::Green,
            "blue"   => Color::Blue,
            "wild"   => Color::Wild,
            _        => panic!("Invalid color: {s:?}"),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Yellow => write!(f, "yellow"),
            Color::Red    => write!(f, "red"),
            Color::Green  => write!(f, "green"),
            Color::Blue   => write!(f, "blue"),
            Color::Wild   => write!(f, "wild"),
        }
    }
}

/// The action (face value) of an Uno card.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    /// A numbered card (0–9).
    Number(u8),
    /// Skips the next player's turn.
    Skip,
    /// Reverses the direction of play.
    Reverse,
    /// Forces the next player to draw two cards.
    DrawTwo,
    /// Skips all other players.
    SkipAll,
    /// Forces the next player to draw four cards.
    DrawFour,
    /// Discards all cards of the chosen color from the next player's hand.
    DiscardAll,
    /// Reverses play direction and forces the next player to draw four.
    ReverseDrawFour,
    /// Forces the next player to draw six cards.
    DrawSix,
    /// Forces the next player to draw ten cards.
    DrawTen,
    /// Each player secretly picks a color; those who pick differently draw cards.
    ColorRoulette,
}

impl Action {
    /// Parses an [`Action`] from a string slice.
    ///
    /// # Panics
    /// Panics if `s` does not match a known action string.
    pub fn from_string(s: &str) -> Self {
        match s {
            "0"              => Action::Number(0),
            "1"              => Action::Number(1),
            "2"              => Action::Number(2),
            "3"              => Action::Number(3),
            "4"              => Action::Number(4),
            "5"              => Action::Number(5),
            "6"              => Action::Number(6),
            "7"              => Action::Number(7),
            "8"              => Action::Number(8),
            "9"              => Action::Number(9),
            "skip"           => Action::Skip,
            "reverse"        => Action::Reverse,
            "+2"             => Action::DrawTwo,
            "skip_all"       => Action::SkipAll,
            "+4"             => Action::DrawFour,
            "discard_all"    => Action::DiscardAll,
            "reverse_+4"     => Action::ReverseDrawFour,
            "+6"             => Action::DrawSix,
            "+10"            => Action::DrawTen,
            "color_roulette" => Action::ColorRoulette,
            _                => panic!("Invalid action: {s:?}"),
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Number(n)       => write!(f, "{n}"),
            Action::Skip            => write!(f, "skip"),
            Action::Reverse         => write!(f, "reverse"),
            Action::DrawTwo         => write!(f, "+2"),
            Action::SkipAll         => write!(f, "skip_all"),
            Action::DrawFour        => write!(f, "+4"),
            Action::DiscardAll      => write!(f, "discard_all"),
            Action::ReverseDrawFour => write!(f, "reverse_+4"),
            Action::DrawSix         => write!(f, "+6"),
            Action::DrawTen         => write!(f, "+10"),
            Action::ColorRoulette   => write!(f, "color_roulette"),
        }
    }
}

/// A single Uno card consisting of a [`Color`] and an [`Action`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    color: Color,
    action: Action,
}

impl Card {
    /// Creates a new [`Card`] with the given color and action.
    pub fn new(color: Color, action: Action) -> Self {
        Card { color, action }
    }

    /// Parses a [`Card`] from a string of the form `"<color> <action>"`.
    ///
    /// # Panics
    /// Panics if the string format is invalid.
    pub fn from_string(s: &str) -> Self {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let color  = Color::from_string(parts[0]);
        let action = Action::from_string(parts[1]);
        Card { color, action }
    }

    /// Returns the card's [`Color`].
    pub fn get_color(&self) -> Color {
        self.color
    }

    /// Returns the card's [`Action`].
    pub fn get_action(&self) -> Action {
        self.action
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.color, self.action)
    }
}