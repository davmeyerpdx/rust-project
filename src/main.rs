mod lib;
use lib::model::*;
//;

fn main() {
    let mut a_deck = Deck::new();
    // println!("{}", a_deck.stringfy());
    // println!("--------------------------------");
    a_deck.shuffle();
    println!("length of deck: {}", a_deck.length());

    let mut banker = Banker::new();
    banker.draw_card(&mut a_deck);
    // banker.draw_card(&mut a_deck);
    // banker.draw_card(&mut a_deck);
    println!("{}", banker.stringfy());
    println!("after drew, length of deck: {}", a_deck.length());
    println!("{}", a_deck.stringfy());
}
