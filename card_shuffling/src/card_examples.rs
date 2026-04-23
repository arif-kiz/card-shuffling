//! Example implementations of the `Color` and `Action` traits.
//!
//! This module provides concrete types representing the standard Uno color palette
//! and the expanded action set found in the "Uno No Mercy" edition.

use card_shuffling::prelude::*;
use std::fmt;

/// Represents the color suit palette of Uno.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum UnoColor {
    #[default]
    Yellow,
    Red,
    Green,
    Blue,
    Wild,
}

impl Color for UnoColor {
    /// Converts a lowercase string identifier into an [`UnoColor`] variant.
    ///
    /// The supported identifiers exactly match the lowercase names of the variants.
    ///
    /// # Panics
    ///
    /// Panics if the provided string `color_str` is not one of: `"yellow"`, `"red"`, 
    /// `"green"`, `"blue"`, or `"wild"`.
    fn from_string(color_str: &str) -> Self {
        match color_str {
            "yellow" => Self::Yellow,
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            "wild" => Self::Wild,
            _ => panic!("Invalid UnoColor identifier: {color_str:?}"),
        }
    }

    /// Determines if this color variant represents a wild type.
    fn is_wild(&self) -> bool {
        matches!(self, Self::Wild)
    }
}

impl fmt::Display for UnoColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yellow => write!(f, "yellow"),
            Self::Red => write!(f, "red"),
            Self::Green => write!(f, "green"),
            Self::Blue => write!(f, "blue"),
            Self::Wild => write!(f, "wild"),
        }
    }
}

/// The expanded set of actions available in the "Uno No Mercy" rule variant.
///
/// This enum covers both traditional Uno cards (like numbers and skips) and the 
/// extreme penalty cards (like Draw Ten and Skip All) introduced in No Mercy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum UnoNoMercyAction {
    /// A standard numbered card with a face value ranging from 0 to 9.
    Number(u8),
    /// Forces the immediate next player to forfeit their turn entirely.
    Skip,
    /// Flips the current direction of the turn order.
    Reverse,
    /// Forces the next player to draw two cards and forfeit their turn.
    DrawTwo,
    /// Bypasses the turns of all other players, immediately granting the 
    /// current player another turn.
    SkipAll,
    /// Forces the next player to draw four cards and forfeit their turn.
    DrawFour,
    /// Permits the player to flush all cards of a matching color from their hand.
    DiscardAll,
    /// Inverts the turn order AND forces the newly determined next player to draw four.
    ReverseDrawFour,
    /// Forces the next player to draw six cards and forfeit their turn.
    DrawSix,
    /// Forces the next player to draw ten cards and forfeit their turn.
    DrawTen,
    /// A chaotic minigame card: players secretly choose a color, and those who 
    /// pick a different color than the active player must draw cards.
    ColorRoulette,
}

impl Default for UnoNoMercyAction {
    fn default() -> Self {
        Self::Number(0)
    }
}

impl Action for UnoNoMercyAction {
    /// Parses an [`UnoNoMercyAction`] from its string identifier.
    ///
    /// # Panics
    ///
    /// Panics if `action_str` does not correspond to any valid Uno No Mercy action.
    fn from_string(action_str: &str) -> Self {
        match action_str {
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
            _ => panic!("Invalid UnoNoMercyAction identifier: {action_str:?}"),
        }
    }

    /// Evaluates the heuristic impact of the action for deck shuffle quality scoring.
    ///
    /// Values scale roughly with the severity of the card's effect, from 1 (numbers) 
    /// up to 8 (`DrawTen`).
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
