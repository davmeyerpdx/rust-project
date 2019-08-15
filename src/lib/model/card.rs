use Color::*;
use Rank::*;
use Suit::*;

use rand::{thread_rng, Rng};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Color {
    Red,
    Black,
}

pub struct Card {
    rank: Rank,
    suit: Suit,
    color: Color,
    rank_id: u8,
    value: u8,
}
#[derive(Default)]
pub struct Deck(pub Vec<Card>);

pub trait Stringfy {
    fn stringfy(&self) -> String;
}

impl Stringfy for Card {
    fn stringfy(&self) -> String {
        /*
        format!("rank:{}, suit:{}, color:{}, rank_id:{}, value:{}",
            self.rank.stringfy(), self.suit.stringfy(), self.color.stringfy(), self.rank_id, self.value)
        */
        format!("{}_{}", self.rank.stringfy(), self.suit.stringfy())
    }
}

impl Stringfy for Rank {
    fn stringfy(&self) -> String {
        match &self {
            Ace => "A".to_string(),
            Two => "2".to_string(),
            Three => "3".to_string(),
            Four => "4".to_string(),
            Five => "5".to_string(),
            Six => "6".to_string(),
            Seven => "7".to_string(),
            Eight => "8".to_string(),
            Nine => "9".to_string(),
            Ten => "10".to_string(),
            Jack => "J".to_string(),
            Queen => "Q".to_string(),
            King => "K".to_string(),
        }
    }
}

impl Stringfy for Suit {
    fn stringfy(&self) -> String {
        match &self {
            Diamond => "Diamond".to_string(),
            Club => "Club".to_string(),
            Heart => "Heart".to_string(),
            Spade => "Spade".to_string(),
        }
    }
}

impl Stringfy for Color {
    fn stringfy(&self) -> String {
        match &self {
            Red => "Red".to_string(),
            Black => "Black".to_string(),
        }
    }
}

impl Rank {
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
    pub fn to_rankid(self) -> u8 {
        match self {
            Ace => 1,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
            Jack => 11,
            Queen => 12,
            King => 13,
        }
    }
    pub fn to_cardvalue(self) -> u8 {
        match self {
            Ace => 11,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
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

impl Card {
    pub fn new_by_rank(cdrank: Rank, cdsuit: Suit) -> Card {
        match cdsuit {
            Diamond | Heart => {
                let rankid = cdrank.to_rankid();
                let cardvalue = cdrank.to_cardvalue();
                Card {
                    rank: cdrank,
                    suit: cdsuit,
                    color: Red,
                    rank_id: rankid,
                    value: cardvalue,
                }
            }
            Club | Spade => {
                let rankid = cdrank.to_rankid();
                let cardvalue = cdrank.to_cardvalue();
                Card {
                    rank: cdrank,
                    suit: cdsuit,
                    color: Black,
                    rank_id: rankid,
                    value: cardvalue,
                }
            }
        }
    }
    pub fn new_by_rankid(rkid: u8, cdsuit: Suit) -> Card {
        match cdsuit {
            Diamond | Heart => {
                let rk = Rank::from_rankid(rkid).unwrap();
                Card {
                    rank: rk,
                    suit: cdsuit,
                    color: Red,
                    rank_id: rkid,
                    value: rk.to_cardvalue(),
                }
            }
            Club | Spade => {
                let rk = Rank::from_rankid(rkid).unwrap();
                Card {
                    rank: rk,
                    suit: cdsuit,
                    color: Black,
                    rank_id: rkid,
                    value: rk.to_cardvalue(),
                }
            }
        }
    }
    pub fn get_value(&self) -> u8 {
        self.value
    }
    pub fn get_suit(&self) -> Suit {
        self.suit
    }
    pub fn get_rank(&self) -> Rank {
        self.rank
    }
    pub fn get_color(&self) -> Color {
        self.color
    }
    pub fn get_rankid(&self) -> u8 {
        self.rank_id
    }
    pub fn get_pic(&self) -> String {
        format!("/{}.png", self.stringfy())
    }
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck(Vec::new());
        for rank_id in 1..=13 {
            deck.0.push(Card::new_by_rankid(rank_id, Diamond));
        }
        for rank_id in 1..=13 {
            deck.0.push(Card::new_by_rankid(rank_id, Club));
        }
        for rank_id in 1..=13 {
            deck.0.push(Card::new_by_rankid(rank_id, Heart));
        }
        for rank_id in 1..=13 {
            deck.0.push(Card::new_by_rankid(rank_id, Spade));
        }
        deck
    }
    pub fn shuffle(&mut self) {
        //Knuth shuffle algrithom
        let mut rng = thread_rng();
        for i in 0..=51 {
            let j = rng.gen_range(0, 51);
            self.0.swap(i, j)
        }
    }
    pub fn length(&self) -> u8 {
        self.0.len() as u8
    }
}

impl Stringfy for Deck {
    fn stringfy(&self) -> String {
        let mut res = String::new();
        let mut it = self.0.iter();
        for _ in 0..=51 {
            match it.next() {
                Some(x) => {
                    res.push_str(x.stringfy().as_str());
                    res.push('\n')
                }
                None => break,
            }
        }
        res
    }
}
/*
impl ToString for Card{
    fn to_string(&self) -> String{
        format!("{}_{}", self.rank.to_string(), self.suit.to_string())
    }
}*/
