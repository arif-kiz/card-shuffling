// Splits the deck into two halves and interchanges them
fn middle_split(cards: &mut Vec<i32>) {
    let mid = cards.len() / 2;
    cards.rotate_left(mid);
}

// Splits the deck into two part at a point and interchanges them
fn split_at(cards: &mut Vec<i32>, i: usize) {
    if i <= cards.len() {
        cards.rotate_left(i);
    }
}

fn main() {
    let mut cards: Vec<_> = (1..169).into_iter().collect();
    println!("Fisrt: {:?}", &cards[0..10]);

    middle_split(&mut cards);
    println!("Middle Split: {:?}", &cards[..10]);

    split_at(&mut cards, 50);
    println!("Split at 50: {:?}", &cards[..10]);
}