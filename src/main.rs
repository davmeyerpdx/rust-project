mod lib;
use lib::model::*;

use std::io::{self, BufRead};

fn sample_game_loop(){
    let stdin = io::stdin();
    //------------game_init-----------
    let mut deck = Deck::new();
    deck.shuffle();

    let mut banker = Banker::new();
    let mut human = Human::new();
    banker.draw_card(&mut deck);
    banker.draw_card(&mut deck);
    human.draw_card(&mut deck);
    human.draw_card(&mut deck);
    //---------------------------------
    //TODO Human make a bet()
    //---------------------------------
    banker.check_blackjack();
    human.check_blackjack();
    if banker.blackjack && banker.check_darkcard_is_ace() {
        banker.flip_card = true;
    }
    if banker.check_lightcard_is_ace() {
        println!("Human Insurance choice: 1.yes, 2.No");
        let mut choice = String::new();
        stdin.read_line(&mut choice).expect("failed to read from stdin");
        let trimmed = choice.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => {
                if i==1 {
                    human.insurance = true;
                }
                else {
                    human.insurance = false;
                }
            },
            Err(..) => println!("this was not an integer: {}", trimmed),
        };
        if banker.blackjack {
            banker.flip_card = true;
            human.get_compensation();
        }
    }
}

fn main() {
    /*
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
    */
}
