mod lib;
use lib::model::*;
//;

fn main() {
    let mut a_deck = Deck::new();
    println!("{}", a_deck.stringfy());
    println!("--------------------------------");
    a_deck.shuffle();
    println!("{}", a_deck.stringfy());
}
