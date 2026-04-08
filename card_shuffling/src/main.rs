mod cards;
const NUMBER_OF_CARDS: usize = 168;
use cards::Cards;

fn main() {
    let mut cards: Cards = Cards::new(NUMBER_OF_CARDS);
    println!("Fisrt: {:?}", &cards.cards[0..10]);

    cards.middle_split();
    println!("Middle Split: {:?}", &cards.cards[..10]);

    cards.split_at(50);
    println!("Split at 50: {:?}", &cards.cards[..10]);

    cards.riffle_shuffle();
    println!("Riffle Shuffle: {:?}", &cards.cards[..10]);

    cards.take_from_middle(20, 80);
    println!("From middle: {:?}", &cards.cards[..10]);
}