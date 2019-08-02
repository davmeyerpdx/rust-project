use crate::*;

pub trait Player{
    fn compute_value(&self) -> u8;
    fn draw_card(&mut self, deck: &mut Deck);
}

pub struct Banker{
    lightcard: Vec<Card>,
    darkcard: Option<Card>,
    blackjack: bool,
}
pub struct Human{
    lightcard: Vec<Card>,
    blackjack: bool,
    chip: i32,
    bet: i32,
    insurance: bool,
    giveup: bool,
}

impl Banker{
    pub fn new() -> Self{
        Banker{
            lightcard: Vec::new(),
            darkcard: None,
            blackjack: false,
        }
    }
}
impl Human{
    pub fn new() -> Self{
        Human{
            lightcard: Vec::new(),
            blackjack: false,
            chip: 100,
            bet: 0,
            insurance: false, 
            giveup: false,
        }
    }
}
impl Player for Banker {
    fn compute_value(&self) -> u8{
        let mut count_ace = 0;
        let mut total_value = 0;
        let mut it = self.lightcard.iter();
        loop{
            match it.next(){
                Some(x) => {
                    total_value += x.get_value();
                    if x.get_rank() == Ace{
                        count_ace += 1;
                    }
                },
                None => break
            }
        }
        match self.darkcard.as_ref(){
            Some(x) => {
                total_value += x.get_value();
                if x.get_rank() == Ace{
                    count_ace += 1;
                }
            },
            None => panic!("darkcard is empty, impossible")
        }
        if total_value > 21 {
            total_value -= 10*count_ace;
            total_value
        } else {
            total_value
        }
    }
    fn draw_card(&mut self, deck: &mut Deck){
        if self.lightcard.is_empty(){
            self.lightcard.insert(0, deck.0.pop().unwrap());
        } else if self.darkcard.is_none() {
            self.darkcard = Some(deck.0.pop().unwrap());
        } else {
            self.lightcard.push(deck.0.pop().unwrap());
        }
    }
}
impl Player for Human {
    fn compute_value(&self) -> u8{
        let mut count_ace = 0;
        let mut total_value = 0;
        let mut it = self.lightcard.iter();
        loop{
            match it.next(){
                Some(x) => {
                    total_value += x.get_value();
                    if x.get_rank() == Ace{
                        count_ace += 1;
                    }
                },
                None => break
            }
        }
        if total_value > 21 {
            total_value -= 10*count_ace;
            total_value
        } else {
            total_value
        }
    }
    fn draw_card(&mut self, deck: &mut Deck){
        self.lightcard.push(deck.0.pop().unwrap());
    }
}
impl Stringfy for Banker{
    fn stringfy(&self) -> String{
        let mut res = String::new();
        let mut it = self.lightcard.iter();
        loop{
            match it.next(){
                Some(x) => {res.push_str("lightcards: "); res.push_str(x.stringfy().as_str()); res.push('\n')},
                None => break
            }
        }
        if self.darkcard.is_some(){
            res.push_str("Darkcard: "); res.push_str(self.darkcard.as_ref().unwrap().stringfy().as_str()); res.push('\n');
        }
        res
    }
}