use crate::card::{Card, Color, Action};

pub struct Cards {
    pub cards: Vec<Card>,
}

impl Cards {
    pub fn new(size: usize) -> Self {
        Cards {
            cards: (0..size).into_iter().map(|_| Card::new(Color::Yellow, Action::Number(0))).collect(),
        }
    }

    pub fn from_file(filename: &str) -> Self {
        if let Ok(contents) = std::fs::read_to_string(filename) {
            let cards = contents.lines().flat_map(|line| {
                let parts = line.split_whitespace().collect::<Vec<&str>>();
                if parts.len() != 3 {
                    // Skip empty or invalid lines
                    return vec![].into_iter();
                }
                // The format is: action color count (e.g. `0 yellow 2`)
                let action = Action::from_string(parts[0]);
                let color = Color::from_string(parts[1]);
                let count: usize = parts[2].parse().unwrap_or(0);
                
                // Returns `count` cards of the same type via flat_map
                vec![Card::new(color, action); count].into_iter()
            }).collect();
            Cards {
                cards,
            }
        } else {
            Cards::new(0)
        }
    }

    // Middle Split
    // Split the deck into two part at a point and interchanges them
    pub fn middle_split(&mut self) {
        let mid = self.cards.len() / 2;
        self.cards.rotate_left(mid);
    }

    // Split at
    // Splits the deck into two part at a point and interchanges them
    pub fn split_at(&mut self, i: usize) {
        if i <= self.cards.len() {
            self.cards.rotate_left(i);
        }
    }

    // Riffle shuffle
    // Split the deck into two part at a point and interleave them
    pub fn riffle_shuffle(&mut self) {
        let mid = self.cards.len() / 2;
        let mut left = self.cards[..mid].to_vec();
        left.reverse();
        let mut right = self.cards[mid..].to_vec();
        right.reverse();
        
        for i in 0..self.cards.len() {
            if i % 2 == 0 {
                self.cards[i] = left.pop().unwrap();
            } else {
                self.cards[i] = right.pop().unwrap();
            }
        }
    }

    // Take from middle
    // Take cards from index i to j and move them to the beginning of the deck
    pub fn take_from_middle(&mut self, i: usize, j: usize) {
        if j <= i || j > self.cards.len() {
            return
        }
        self.cards[0..j].rotate_right(j-i);
    }

    // Helper function for card power evaluation
    fn card_power(card: &Card) -> i32 {
        let mut power = match card.get_action() {
            Action::Number(_) => 1,
            Action::Skip | Action::Reverse => 2,
            Action::DrawTwo | Action::SkipAll | Action::DiscardAll => 3,
            Action::DrawFour => 4,
            Action::ReverseDrawFour => 5,
            Action::DrawSix | Action::ColorRoulette => 6,
            Action::DrawTen => 8,
        };
        if card.get_color() == Color::Wild {
            power += 2;
        }
        power
    }

    // Helper function to evaluate pair interaction
    fn evaluate_pair(card_a: &Card, card_b: &Card, distance: i32) -> i32 {
        if distance == 0 {
            return 0; // Safety guard
        }
        
        let power_a = Self::card_power(card_a);
        let power_b = Self::card_power(card_b);
        let mut delta = 0;

        // Same color penalty
        if card_a.get_color() == card_b.get_color() {
            if card_a.get_color() == Color::Wild {
                // Penalize clustered Wilds more noticeably
                delta -= 15 / distance; 
            } else {
                // User requirement: same color should have very little decrease in points
                delta -= 2 / distance; 
            }
        } else {
            // Reward color variety
            delta += 5 / distance; 
        }

        // Same exact action penalty (e.g. chaining lots of Skips)
        if card_a.get_action() == card_b.get_action() {
            delta -= 20 / distance;
        }

        // High value card clustering
        if power_a > 3 && power_b > 3 {
            delta -= (power_a * power_b) / distance;
        } else if power_a > 3 {
            // Reward for high value card well distributed among average/low value cards
            delta += (power_a * (10 - power_b)) / distance;
        }

        delta
    }

    // Helper function to evaluate single card within window
    fn evaluate_card(cards: &[Card], index: usize, window: usize) -> i32 {
        let mut score = 0;
        let start = index.saturating_sub(window);
        let end = (index + window + 1).min(cards.len());

        let target = &cards[index];
        for j in start..end {
            if index != j {
                let dist = (index as i32 - j as i32).abs();
                score += Self::evaluate_pair(target, &cards[j], dist);
            }
        }
        score
    }

    pub fn is_shuffled_properly(&self) -> (Vec<i32>, i32) {
        let mut scores = Vec::with_capacity(self.cards.len());
        let mut total_score = 0;
        let window = 7; // User agreed to window size 7
        
        for i in 0..self.cards.len() {
            let score = Self::evaluate_card(&self.cards, i, window);
            scores.push(score);
            total_score += score;
        }
        
        (scores, total_score)
    }
}