pub struct Cards {
    pub cards: Vec<usize>,
}

impl Cards {
    pub fn new(size: usize) -> Self {
        Cards {
            cards: (0..size).into_iter().collect(),
        }
    }

    // Split the deck into two part at a point and interchanges them
    pub fn middle_split(&mut self) {
        let mid = self.cards.len() / 2;
        self.cards.rotate_left(mid);
    }

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
}