//! Types representing individual Uno cards.
//!
//! This module provides [`Color`], [`Action`], and [`Card`] — the building
//! blocks used by [`crate::cards::Cards`] to form a deck.

use std::fmt;

/// The color of an Uno card.
///
/// `Wild` is used for cards that can be played on any color.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
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
    #[must_use]
    pub fn from_string(s: &str) -> Self {
        match s {
            "yellow" => Color::Yellow,
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            "wild" => Color::Wild,
            _ => panic!("Invalid color: {s:?}"),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Yellow => write!(f, "yellow"),
            Color::Red => write!(f, "red"),
            Color::Green => write!(f, "green"),
            Color::Blue => write!(f, "blue"),
            Color::Wild => write!(f, "wild"),
        }
    }
}

pub trait Action
    where Self: Clone + Copy + PartialEq + Eq
{
    fn from_string(s: &str) -> Self;
    fn power(self) -> i32;
}

/// The action (face value) of an Uno card.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum UnoNoMercyAction {
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

impl Action for UnoNoMercyAction {
    /// Parses an [`UnoNoMercyAction`] from a string slice.
    ///
    /// # Panics
    /// Panics if `s` does not match a known action string.
    fn from_string(s: &str) -> Self {
        match s {
            "0" => UnoNoMercyAction::Number(0),
            "1" => UnoNoMercyAction::Number(1),
            "2" => UnoNoMercyAction::Number(2),
            "3" => UnoNoMercyAction::Number(3),
            "4" => UnoNoMercyAction::Number(4),
            "5" => UnoNoMercyAction::Number(5),
            "6" => UnoNoMercyAction::Number(6),
            "7" => UnoNoMercyAction::Number(7),
            "8" => UnoNoMercyAction::Number(8),
            "9" => UnoNoMercyAction::Number(9),
            "skip" => UnoNoMercyAction::Skip,
            "reverse" => UnoNoMercyAction::Reverse,
            "+2" => UnoNoMercyAction::DrawTwo,
            "skip_all" => UnoNoMercyAction::SkipAll,
            "+4" => UnoNoMercyAction::DrawFour,
            "discard_all" => UnoNoMercyAction::DiscardAll,
            "reverse_+4" => UnoNoMercyAction::ReverseDrawFour,
            "+6" => UnoNoMercyAction::DrawSix,
            "+10" => UnoNoMercyAction::DrawTen,
            "color_roulette" => UnoNoMercyAction::ColorRoulette,
            _ => panic!("Invalid action: {s:?}"),
        }
    }

    fn power(self) -> i32 {
        match self {
            UnoNoMercyAction::Number(_) => 1,
            UnoNoMercyAction::Skip | UnoNoMercyAction::Reverse => 2,
            UnoNoMercyAction::DrawTwo | UnoNoMercyAction::SkipAll | UnoNoMercyAction::DiscardAll => 3,
            UnoNoMercyAction::DrawFour => 4,
            UnoNoMercyAction::ReverseDrawFour => 5,
            UnoNoMercyAction::DrawSix | UnoNoMercyAction::ColorRoulette => 6,
            UnoNoMercyAction::DrawTen => 8,
        }
    }
}

impl fmt::Display for UnoNoMercyAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnoNoMercyAction::Number(n) => write!(f, "{n}"),
            UnoNoMercyAction::Skip => write!(f, "skip"),
            UnoNoMercyAction::Reverse => write!(f, "reverse"),
            UnoNoMercyAction::DrawTwo => write!(f, "+2"),
            UnoNoMercyAction::SkipAll => write!(f, "skip_all"),
            UnoNoMercyAction::DrawFour => write!(f, "+4"),
            UnoNoMercyAction::DiscardAll => write!(f, "discard_all"),
            UnoNoMercyAction::ReverseDrawFour => write!(f, "reverse_+4"),
            UnoNoMercyAction::DrawSix => write!(f, "+6"),
            UnoNoMercyAction::DrawTen => write!(f, "+10"),
            UnoNoMercyAction::ColorRoulette => write!(f, "color_roulette"),
        }
    }
}

/// A single Uno card consisting of a [`Color`] and an [`Action`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Card<Act: Action> {
    color: Color,
    action: Act,
}

impl<Act: Action> Card<Act> {
    /// Creates a new [`Card`] with the given color and action.
    #[must_use]
    pub fn new(color: Color, action: Act) -> Self {
        Card { color, action }
    }

    /// Parses a [`Card`] from a string of the form `"<color> <action>"`.
    ///
    /// # Panics
    /// Panics if the string format is invalid.
    #[must_use]
    pub fn from_string(s: &str) -> Self {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let color = Color::from_string(parts[0]);
        let action = Action::from_string(parts[1]);
        Card { color, action }
    }

    /// Returns the card's [`Color`].
    #[must_use]
    pub fn get_color(&self) -> Color {
        self.color
    }

    /// Returns the card's [`Action`].
    #[must_use]
    pub fn get_action(&self) -> Act {
        self.action
    }
}

impl<Act: Action> fmt::Display for Card<Act> 
    where Act: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.color, self.action)
    }
}