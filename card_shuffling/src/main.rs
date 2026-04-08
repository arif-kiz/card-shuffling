const NUMBER_OF_CARDS: usize = 168;

// Splits the deck into two halves and interchanges them
fn middle_split(cards: &mut Vec<usize>) {
    let mid = cards.len() / 2;
    cards.rotate_left(mid);
}

// Splits the deck into two part at a point and interchanges them
fn split_at(cards: &mut Vec<usize>, i: usize) {
    if i <= cards.len() {
        cards.rotate_left(i);
    }
}

// Riffle shuffle
fn riffle_shuffle(cards: &mut Vec<usize>) {
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

// Take from middle
fn take_from_middle(cards: &mut Vec<usize>, i: usize, j: usize) {
    if j <= i || j > cards.len() {
        return
    }
    cards[0..j].rotate_right(j-i);
}

fn main() {
    let mut cards: Vec<_> = (0..NUMBER_OF_CARDS).into_iter().collect();
    println!("Fisrt: {:?}", &cards[0..10]);

    middle_split(&mut cards);
    println!("Middle Split: {:?}", &cards[..10]);

    split_at(&mut cards, 50);
    println!("Split at 50: {:?}", &cards[..10]);

    riffle_shuffle(&mut cards);
    println!("Riffle Shuffle: {:?}", &cards[..10]);

    take_from_middle(&mut cards, 20, 80);
    println!("From middle: {:?}", &cards[..10]);
}