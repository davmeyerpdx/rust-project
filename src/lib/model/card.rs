use Suit::*;
use Rank::*;
use Color::*;

pub enum Suit{
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}
pub enum Rank{
    Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, 
}
pub enum Color{
    Red,
    Black,
}

pub struct Card{
    rank: Rank,
    suit: Suit,
    color: Color,
    rank_id: i32,
    value: i32,
}

pub struct Deck(Vec<Card>);

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
            Ace     =>  format!("Ace"),
            Two     =>  format!("Two"), 
            Three   =>  format!("Three"),
            Four    =>  format!("Four"), 
            Five    =>  format!("Five"), 
            Six     =>  format!("Six"), 
            Seven   =>  format!("Seven"), 
            Eight   =>  format!("Eight"), 
            Nine    =>  format!("Nine"), 
            Ten     =>  format!("Ten"), 
            Jack    =>  format!("Jack"), 
            Queen   =>  format!("Queen"), 
            King    =>  format!("King"),
        }
        
    }
}
impl Stringfy for Suit{
    fn stringfy(&self) -> String{
        match &self{
            Diamonds => format!("Diamonds"),
            Clubs    => format!("Clubs"),
            Hearts   => format!("Hearts"),
            Spades   => format!("Spades"),
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
    pub fn to_rankid(&self) -> i32{
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
     pub fn to_cardvalue(&self) -> i32{
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
    pub fn new_by_rank(crank:Rank, csuit: Suit) -> Card{
        match csuit{
            Diamonds|Hearts => {
                let rankid = crank.to_rankid();
                let cardvalue = crank. to_cardvalue();
                Card{ rank: crank,
                      suit: csuit, 
                      color: Red, 
                      rank_id: rankid, 
                      value: cardvalue}
            },
            Clubs|Spades => {
                let rankid = crank.to_rankid();
                let cardvalue = crank. to_cardvalue();
                Card{ rank: crank,
                      suit: csuit, 
                      color: Red, 
                      rank_id: rankid, 
                      value: cardvalue}
            }
        }
    }

}