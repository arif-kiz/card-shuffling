//! Defines the core components of an Uno card.
//!
//! This module contains the fundamental structures and enums that make up a single card
//! in an Uno deck. It includes the [`Color`] enum for representing card colors, the 
//! [`Action`] trait defining the behavior and power of card faces, and the generic 
//! [`Card`] struct which combines a color and an action.
//! 
//! Additionally, it provides [`UnoNoMercyAction`], a specific implementation of the 
//! [`Action`] trait tailored for the "Uno No Mercy" rule set.

use std::fmt;

/// Represents the color suit of an Uno card.
///
/// A standard Uno deck consists of four primary colors (Yellow, Red, Green, Blue)
/// and a special `Wild` category for cards that transcend color restrictions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Color {
    /// The yellow color suit.
    Yellow,
    /// The red color suit.
    Red,
    /// The green color suit.
    Green,
    /// The blue color suit.
    Blue,
    /// Represents wild cards which can be played on top of any color.
    Wild,
}

impl Color {
    /// Converts a lowercase string slice into a [`Color`] variant.
    ///
    /// The supported string values are exactly the lowercase names of the variants.
    ///
    /// # Panics
    ///
    /// Panics if the provided string `s` is not one of: `"yellow"`, `"red"`, `"green"`, 
    /// `"blue"`, or `"wild"`.
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

/// Defines the behavior and properties of a card's face value or action.
///
/// This trait allows the [`Card`] struct to be generic over different Uno rule sets
/// (e.g., standard Uno, Uno Flip, Uno No Mercy). Any type implementing this trait
/// can be used as the action component of a `Card`.
pub trait Action
    where Self: Clone + Copy + PartialEq + Eq
{
    /// Parses the action from its string representation.
    ///
    /// This is typically used when loading a deck configuration from a file.
    fn from_string(s: &str) -> Self;
    
    /// Returns the "power" level of the action.
    ///
    /// Power is a heuristic metric used primarily for evaluating the quality of a 
    /// shuffle. Higher values generally correspond to more impactful cards 
    /// (e.g., a `DrawTen` has significantly more power than a simple `Number`).
    fn power(self) -> i32;
}

/// The specific actions and face values available in the "Uno No Mercy" game variant.
///
/// This enum implements the [`Action`] trait and includes both standard Uno cards
/// (numbers, skips, reverses) and the extreme penalty cards introduced in the 
/// No Mercy edition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum UnoNoMercyAction {
    /// A standard numbered card with a value from 0 to 9.
    Number(u8),
    /// Skips the next player's turn entirely.
    Skip,
    /// Reverses the current direction of play.
    Reverse,
    /// Forces the next player to draw two cards and lose their turn.
    DrawTwo,
    /// Skips the turns of all other players, giving the current player another turn.
    SkipAll,
    /// Forces the next player to draw four cards and lose their turn.
    DrawFour,
    /// Allows the player to discard all cards of a matching color from their hand.
    DiscardAll,
    /// Reverses the direction of play AND forces the new next player to draw four cards.
    ReverseDrawFour,
    /// Forces the next player to draw six cards and lose their turn.
    DrawSix,
    /// Forces the next player to draw ten cards and lose their turn.
    DrawTen,
    /// A chaotic card where players must secretly choose a color. Those who do not 
    /// match the primary player's choice must draw cards.
    ColorRoulette,
}

impl Action for UnoNoMercyAction {
    /// Parses an [`UnoNoMercyAction`] from a specific string identifier.
    ///
    /// # Panics
    ///
    /// Panics if the string does not match any known Uno No Mercy action identifier.
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

    /// Calculates the heuristic power of the card action for shuffle evaluation.
    ///
    /// The power scale is roughly proportional to the severity of the card's effect,
    /// ranging from 1 for simple number cards up to 8 for the devastating `DrawTen`.
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

/// A fundamental representation of a single card in the deck.
///
/// It combines a [`Color`] suit with a specific [`Action`] face value. By making the
/// action type generic, this struct can represent cards from various Uno editions 
/// seamlessly, provided the action implements the [`Action`] trait.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Card<Act: Action> {
    color: Color,
    action: Act,
}

impl<Act: Action> Card<Act> {
    /// Instantiates a new [`Card`] from a designated color and action.
    #[must_use]
    pub fn new(color: Color, action: Act) -> Self {
        Card { color, action }
    }

    /// Constructs a [`Card`] by parsing a space-separated string.
    ///
    /// The expected format is `"<color> <action>"`, where the color and action
    /// strings correspond to the implementations of their respective `from_string` methods.
    ///
    /// # Panics
    ///
    /// Panics if the input string does not contain at least two space-separated words,
    /// or if either the color or action cannot be parsed.
    #[must_use]
    pub fn from_string(s: &str) -> Self {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let color = Color::from_string(parts[0]);
        let action = Action::from_string(parts[1]);
        Card { color, action }
    }

    /// Retrieves the card's color suit.
    #[must_use]
    pub fn get_color(&self) -> Color {
        self.color
    }

    /// Retrieves the card's specific action or face value.
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