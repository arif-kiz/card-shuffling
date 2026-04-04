// Splits the deck into two halves and interchanges them
fn middle_split(cards: &mut Vec<i32>) {
    let mut first_half: Vec<i32> = cards[0..168/2].iter().copied().collect();
    let mut second_half: Vec<i32> = cards[168/2..168].iter().copied().collect();
    second_half.append(&mut first_half);
    *cards = second_half;
}
fn main() {
    let mut cards: Vec<_> = (1..169).into_iter().collect();
    println!("{:?}", cards);

    middle_split(&mut cards);
    println!("{:?}", cards);
}