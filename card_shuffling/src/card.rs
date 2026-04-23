//! Defines the core components of an Uno card.
//!
//! This module contains the fundamental structures and enums that make up a single card
//! in an Uno deck. It includes the [`Color`] enum for representing card colors, the 
//! [`Action`] trait defining the behavior and power of card faces, and the generic 
//! [`Card`] struct which combines a color and an action.
//! 
//! Additionally, it provides [`UnoNoMercyAction`], a specific implementation of the 
//! [`Action`] trait tailored for the "Uno No Mercy" rule set.

use std::fmt::{self, Debug};

pub trait Color 
    where Self: Clone + Copy + PartialEq + Eq + Default
{
    fn from_string(s: &str) -> Self;
    fn is_wild(&self) -> bool;
}

/// Defines the behavior and properties of a card's face value or action.
///
/// This trait allows the [`Card`] struct to be generic over different Uno rule sets
/// (e.g., standard Uno, Uno Flip, Uno No Mercy). Any type implementing this trait
/// can be used as the action component of a `Card`.
pub trait Action
    where Self: Clone + Copy + PartialEq + Eq + Default
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

/// A fundamental representation of a single card in the deck.
///
/// It combines a [`Color`] suit with a specific [`Action`] face value. By making the
/// action type generic, this struct can represent cards from various Uno editions 
/// seamlessly, provided the action implements the [`Action`] trait.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Card<Act: Action, Col: Color> {
    color: Col,
    action: Act,
}

impl<Act: Action, Col: Color> Card<Act, Col> {
    /// Instantiates a new [`Card`] from a designated color and action.
    #[must_use]
    pub fn new(color: Col, action: Act) -> Self {
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
    pub fn get_color(&self) -> Col {
        self.color
    }

    /// Retrieves the card's specific action or face value.
    #[must_use]
    pub fn get_action(&self) -> Act {
        self.action
    }
}

impl<Act: Action, Col: Color> fmt::Display for Card<Act, Col> 
    where Act: fmt::Display,
          Col: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.color, self.action)
    }
}