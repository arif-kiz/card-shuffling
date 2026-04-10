#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Yellow,
    Red,
    Green,
    Blue,
    Wild,
}

impl Color {
    pub fn from_string(s: &str) -> Self {
        match s {
            "yellow" => Color::Yellow,
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            "wild" => Color::Wild,
            _ => panic!("Invalid color"),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Yellow => write!(f, "yellow"),
            Color::Red => write!(f, "red"),
            Color::Green => write!(f, "green"),
            Color::Blue => write!(f, "blue"),
            Color::Wild => write!(f, "wild"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Number(u8),
    Skip,
    Reverse,
    DrawTwo,
    SkipAll,
    DrawFour,
    DiscardAll,
    ReverseDrawFour,
    DrawSix,
    DrawTen,
    ColorRoulette,
}

impl Action {
    pub fn from_string(s: &str) -> Self {
        match s {
            "0" => Action::Number(0),
            "1" => Action::Number(1),
            "2" => Action::Number(2),
            "3" => Action::Number(3),
            "4" => Action::Number(4),
            "5" => Action::Number(5),
            "6" => Action::Number(6),
            "7" => Action::Number(7),
            "8" => Action::Number(8),
            "9" => Action::Number(9),
            "skip" => Action::Skip,
            "reverse" => Action::Reverse,
            "+2" => Action::DrawTwo,
            "skip_all" => Action::SkipAll,
            "+4" => Action::DrawFour,
            "discard_all" => Action::DiscardAll,
            "reverse_+4" => Action::ReverseDrawFour,
            "+6" => Action::DrawSix,
            "+10" => Action::DrawTen,
            "color_roulette" => Action::ColorRoulette,
            _ => panic!("Invalid action"),
        }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Number(n) => write!(f, "{}", n),
            Action::Skip => write!(f, "skip"),
            Action::Reverse => write!(f, "reverse"),
            Action::DrawTwo => write!(f, "+2"),
            Action::SkipAll => write!(f, "skip_all"),
            Action::DrawFour => write!(f, "+4"),
            Action::DiscardAll => write!(f, "discard_all"),
            Action::ReverseDrawFour => write!(f, "reverse_+4"),
            Action::DrawSix => write!(f, "+6"),
            Action::DrawTen => write!(f, "+10"),
            Action::ColorRoulette => write!(f, "color_roulette"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    color: Color,
    action: Action,
}

impl Card {
    pub fn new(color: Color, action: Action) -> Self {
        Card {
            color,
            action,
        }
    }

    #[allow(dead_code)]
    pub fn from_string(s: &str) -> Self {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let color = Color::from_string(parts[0]);
        let action = Action::from_string(parts[1]);
        Card {
            color,
            action,
        }
    }

    #[allow(dead_code)]
    pub fn get_color(&self) -> Color {
        self.color
    }

    #[allow(dead_code)]
    pub fn get_action(&self) -> Action {
        self.action
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.color, self.action)
    }
}