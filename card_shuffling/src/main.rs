mod cards;
const NUMBER_OF_CARDS: usize = 168;
use cards::Cards;

fn main() {
    let mut cards: Cards = Cards::new(NUMBER_OF_CARDS);
    println!("Fisrt: {:?} - {:?}", &cards.cards[0..10], &cards.cards[NUMBER_OF_CARDS-10..NUMBER_OF_CARDS]);
    println!("Is shuffled properly: {}", cards.is_shuffled_properly());

    cards.middle_split();
    println!("Middle Split: {:?} - {:?}", &cards.cards[0..10], &cards.cards[NUMBER_OF_CARDS-10..NUMBER_OF_CARDS]);
    println!("Is shuffled properly: {}", cards.is_shuffled_properly());

    cards.split_at(50);
    println!("Split at 50: {:?} - {:?}", &cards.cards[0..10], &cards.cards[NUMBER_OF_CARDS-10..NUMBER_OF_CARDS]);
    println!("Is shuffled properly: {}", cards.is_shuffled_properly());

    cards.riffle_shuffle();
    println!("Riffle Shuffle: {:?} - {:?}", &cards.cards[0..10], &cards.cards[NUMBER_OF_CARDS-10..NUMBER_OF_CARDS]);
    // println!("{:?}", &cards.cards[NUMBER_OF_CARDS/2-5..NUMBER_OF_CARDS/2+5]);
    println!("Is shuffled properly: {}", cards.is_shuffled_properly());

    cards.take_from_middle(20, 80);
    println!("From middle: {:?} - {:?}", &cards.cards[0..10], &cards.cards[NUMBER_OF_CARDS-10..NUMBER_OF_CARDS]);
    println!("Is shuffled properly: {}", cards.is_shuffled_properly());
}