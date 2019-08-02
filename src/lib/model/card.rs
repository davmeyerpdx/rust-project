use Suit::*;
use Rank::*;
use Color::*;

use rand::{thread_rng, Rng};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Suit{
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Rank{
    Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, 
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Color{
    Red,
    Black,
}

pub struct Card{
    rank: Rank,
    suit: Suit,
    color: Color,
    rank_id: u8,
    value: u8,
}

pub struct Deck(pub Vec<Card>);

pub trait Stringfy{
    fn stringfy(&self) -> String;
}

impl Stringfy for Card{
    fn stringfy(&self) -> String{
        format!("rank:{}, suit:{}, color:{}, rank_id:{}, value:{}", 
            self.rank.stringfy(), self.suit.stringfy(), self.color.stringfy(), self.rank_id, self.value)
    }
}

impl Stringfy for Rank{
    fn stringfy(&self) -> String{
        match &self{
            Ace     =>  format!("A"),
            Two     =>  format!("2"), 
            Three   =>  format!("3"),
            Four    =>  format!("4"), 
            Five    =>  format!("5"), 
            Six     =>  format!("6"), 
            Seven   =>  format!("7"), 
            Eight   =>  format!("8"), 
            Nine    =>  format!("9"), 
            Ten     =>  format!("10"), 
            Jack    =>  format!("J"), 
            Queen   =>  format!("Q"), 
            King    =>  format!("K"),
        }
        
    }
}

impl Stringfy for Suit{
    fn stringfy(&self) -> String{
        match &self{
            Diamonds => format!("♦"),
            Clubs    => format!("♣"),
            Hearts   => format!("♥"),
            Spades   => format!("♠"),
        }
    }
}

impl Stringfy for Color{
    fn stringfy(&self) -> String{
        match &self{
            Red => format!("Red"),
            Black => format!("Black"),
        }
    }
}

impl Rank{
    pub fn from_rankid(id: u8) -> Option<Rank> {
        match id {
            1 => Some(Ace),
            2 => Some(Two),
            3 => Some(Three),
            4 => Some(Four),
            5 => Some(Five),
            6 => Some(Six),
            7 => Some(Seven),
            8 => Some(Eight),
            9 => Some(Nine),
            10 => Some(Ten),
            11 => Some(Jack),
            12 => Some(Queen),
            13 => Some(King),
            _ => None,
        }
    }
    pub fn to_rankid(&self) -> u8{
        match &self{
            Ace => 1,
            Two => 2, 
            Three => 3,
            Four => 4, 
            Five => 5, 
            Six =>6, 
            Seven => 7, 
            Eight => 8, 
            Nine => 9, 
            Ten => 10, 
            Jack => 11, 
            Queen => 12, 
            King => 13,
        }
    }
     pub fn to_cardvalue(&self) -> u8{
        match &self{
            Ace => 11,
            Two => 2, 
            Three => 3,
            Four => 4, 
            Five => 5, 
            Six =>6, 
            Seven => 7, 
            Eight => 8, 
            Nine => 9, 
            Ten => 10, 
            Jack => 10, 
            Queen => 10, 
            King => 10,
        }
    }
}

impl Card{
    pub fn new_by_rank(cdrank:Rank, cdsuit: Suit) -> Card{
        match cdsuit{
            Diamonds|Hearts => {
                let rankid = cdrank.to_rankid();
                let cardvalue = cdrank.to_cardvalue();
                Card{ rank: cdrank,
                      suit: cdsuit, 
                      color: Red, 
                      rank_id: rankid, 
                      value: cardvalue}
            },
            Clubs|Spades => {
                let rankid = cdrank.to_rankid();
                let cardvalue = cdrank. to_cardvalue();
                Card{ rank: cdrank,
                      suit: cdsuit, 
                      color: Black, 
                      rank_id: rankid, 
                      value: cardvalue}
            }
        }
    }
    pub fn new_by_rankid(rkid: u8, cdsuit: Suit) -> Card{
        match cdsuit{
            Diamonds|Hearts => {
                let rk = Rank::from_rankid(rkid).unwrap();
                Card{ rank: rk,
                      suit: cdsuit, 
                      color: Red, 
                      rank_id: rkid, 
                      value: rk.clone().to_cardvalue()}
            },
            Clubs|Spades => {
                let rk = Rank::from_rankid(rkid).unwrap();
                Card{ rank: rk,
                      suit: cdsuit, 
                      color: Black, 
                      rank_id: rkid, 
                      value: rk.clone().to_cardvalue()}
            },
        }
    }
    pub fn get_value(&self) -> u8 {
        self.value
    }
    pub fn get_suit(&self) -> Suit{
        self.suit
    }
    pub fn get_rank(&self) -> Rank{
        self.rank
    }
    pub fn get_color(&self) -> Color{
        self.color
    }
    pub fn get_rankid(&self) -> u8{
        self.rank_id
    }
}

impl Deck{
    pub fn new() -> Deck{
        let mut deck = Deck(Vec::new());
        for rank_id in 1..=13 {
            deck.0.push(Card::new_by_rankid(rank_id, Diamonds));
        }
        for rank_id in 1..=13 {
            deck.0.push(Card::new_by_rankid(rank_id, Clubs));
        }
        for rank_id in 1..=13 {
            deck.0.push(Card::new_by_rankid(rank_id, Hearts));
        }
        for rank_id in 1..=13 {
            deck.0.push(Card::new_by_rankid(rank_id, Spades));
        }
        deck
    }
    pub fn shuffle(&mut self){
        let mut rng = thread_rng();
        for i in 0..=51{
            let j = rng.gen_range(0, 51);
            self.0.swap(i, j)
        }
    }
    pub fn length(&self) -> u8{
        self.0.len() as u8
    }
}

impl Stringfy for Deck{
    fn stringfy(&self) -> String {
        let mut res = String::new();
        let mut it = self.0.iter();
        for _ in 0..=51{
            match it.next(){
                Some(x) => {res.push_str(x.stringfy().as_str()); res.push('\n')},
                None => break
            }
        }
        res
    }
}