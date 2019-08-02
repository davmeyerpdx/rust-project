mod lib;
use lib::model::*;
//;

fn main() {
    let mut a_deck = Deck::new();
    a_deck.shuffle();
    let mut banker = Banker::new();
    let mut human = Human::new();
    banker.draw_card(&mut a_deck);
    banker.draw_card(&mut a_deck);
    human.draw_card(&mut a_deck);
    human.draw_card(&mut a_deck);
    println!("Banker's cards: {}--Total value of bank's cards: {}\n", banker.stringfy(), banker.compute_value());
    println!("Human's cards: {}--Total value of human's cards: {}\n", human.stringfy(), human.compute_value());
    
    println!("After drew, length of deck: {}", a_deck.length());
    //println!("{}", a_deck.stringfy());
}
