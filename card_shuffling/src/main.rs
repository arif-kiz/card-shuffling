mod cards;
mod card;
const NUMBER_OF_CARDS: usize = 168;
use cards::Cards;

fn main() {
    let mut cards: Cards = Cards::from_file("uno_nomercy.txt");
    println!("Total cards loaded from file: {}", cards.cards.len());
    
    println!("Fisrt: {:?} - {:?}", &cards.cards[0..5], &cards.cards[NUMBER_OF_CARDS-5..NUMBER_OF_CARDS]);
    let (_, score) = cards.is_shuffled_properly();
    println!("Shuffle score: {}", score);

    cards.middle_split();
    println!("Middle Split: {:?} - {:?}", &cards.cards[0..5], &cards.cards[NUMBER_OF_CARDS-5..NUMBER_OF_CARDS]);
    let (_, score) = cards.is_shuffled_properly();
    println!("Shuffle score: {}", score);

    cards.split_at(50);
    println!("Split at 50: {:?} - {:?}", &cards.cards[0..5], &cards.cards[NUMBER_OF_CARDS-5..NUMBER_OF_CARDS]);
    let (_, score) = cards.is_shuffled_properly();
    println!("Shuffle score: {}", score);

    cards.riffle_shuffle();
    println!("Riffle Shuffle: {:?} - {:?}", &cards.cards[0..5], &cards.cards[NUMBER_OF_CARDS-5..NUMBER_OF_CARDS]);
    // println!("{:?}", &cards.cards[NUMBER_OF_CARDS/2-5..NUMBER_OF_CARDS/2+5]);
    let (_, score) = cards.is_shuffled_properly();
    println!("Shuffle score: {}", score);

    cards.take_from_middle(20, 80);
    println!("From middle: {:?} - {:?}", &cards.cards[0..5], &cards.cards[NUMBER_OF_CARDS-5..NUMBER_OF_CARDS]);
    let (_, score) = cards.is_shuffled_properly();
    println!("Shuffle score: {}", score);
}