mod lib;
use lib::model::*;

use std::io::{self, BufRead};

fn add_bet_(human: &mut Human){
    let stdin = io::stdin();
    println!("Pleace bet: ");
    let mut choice = String::new();
    stdin.read_line(&mut choice).expect("failed to read from stdin");
    let trimmed = choice.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => {
            human.add_bet(i);
        },
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}

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
    add_bet_(&mut human);
    //---------------------------------
    banker.check_blackjack();
    human.check_blackjack();
    println!("Banker's cards: {}--Total value of bank's cards: {}\n", banker.stringfy(), banker.compute_value());
    println!("Human's cards: {}--Total value of human's cards: {}\n", human.stringfy(), human.compute_value());
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
            human.get_2xinsurance();
        } else {
            human.lose_insurance();
        }
    }
    loop {
        println!("1. Hit");
        println!("2. Stand");
        println!("3. Double");
        println!("4. Surrender");
        let mut choice = String::new();
        stdin.read_line(&mut choice).expect("failed to read from stdin");
        let trimmed = choice.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => {
                if i==1 {
                    human.draw_card(&mut deck);
                    if human.compute_value() > 21 {
                        println!("Banker's cards: {}--Total value of bank's cards: {}\n", banker.stringfy(), banker.compute_value());
                        println!("Human's cards: {}--Total value of human's cards: {}\n", human.stringfy(), human.compute_value());
                        println!("Human BUST!!!");
                        human.lose();
                        return;
                    }
                }
                else if i==2{
                    break;
                    //Break
                }
                else if i==3{
                    add_bet_(&mut human);
                }
                else{
                    human.lose();
                    return;
                }
            },
            Err(..) => println!("this was not an integer: {}", trimmed),
        };
    }
    while banker.compute_value() < 17 {
        banker.draw_card(&mut deck);
    }
    if banker.compute_value() > 21 {
        println!("Banker's cards: {}--Total value of bank's cards: {}\n", banker.stringfy(), banker.compute_value());
        println!("Human's cards: {}--Total value of human's cards: {}\n", human.stringfy(), human.compute_value());
        println!("Human Win");
        human.win();
        return;
    }
    if banker.compute_value() == human.compute_value() {
        if banker.blackjack && human.blackjack{
            println!("Banker's cards: {}--Total value of bank's cards: {}\n", banker.stringfy(), banker.compute_value());
            println!("Human's cards: {}--Total value of human's cards: {}\n", human.stringfy(), human.compute_value());
            println!("Tie");
            human.tie();
            return;
        }
        if !banker.blackjack && human.blackjack {
            println!("Banker's cards: {}--Total value of bank's cards: {}\n", banker.stringfy(), banker.compute_value());
            println!("Human's cards: {}--Total value of human's cards: {}\n", human.stringfy(), human.compute_value());
            println!("Human Win");
            human.win();
            return;
        }
        if !banker.blackjack && !human.blackjack {
            println!("Banker's cards: {}--Total value of bank's cards: {}\n", banker.stringfy(), banker.compute_value());
            println!("Human's cards: {}--Total value of human's cards: {}\n", human.stringfy(), human.compute_value());
            println!("Human Lose");
            human.lose();
            return;
        }
    }
    else if banker.compute_value() < human.compute_value() {
        println!("Banker's cards: {}--Total value of bank's cards: {}\n", banker.stringfy(), banker.compute_value());
        println!("Human's cards: {}--Total value of human's cards: {}\n", human.stringfy(), human.compute_value());
        println!("Human Win");
        human.win();
        return;
    }
    else if banker.compute_value() > human.compute_value() {
        println!("Banker's cards: {}--Total value of bank's cards: {}\n", banker.stringfy(), banker.compute_value());
        println!("Human's cards: {}--Total value of human's cards: {}\n", human.stringfy(), human.compute_value());
        println!("Human Lose");
        human.lose();
        return;
    }
}

fn main() {
    sample_game_loop();
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
