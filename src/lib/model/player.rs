use crate::*;

pub trait Player {
    //For both Banker and Human
    fn compute_value(&self) -> u8;
    fn draw_card(&mut self, deck: &mut Deck); // get a card
    fn check_blackjack(&mut self);
}

#[derive(Default)]
pub struct Banker {
    pub lightcard: Vec<Card>,
    pub darkcard: Option<Card>,     //preparing for graphic version, but it's not designed well.....
    pub blackjack: bool,
    pub flip_card: bool,
}
#[derive(Default)]
pub struct Human {
    pub lightcard: Vec<Card>,
    pub blackjack: bool,
    pub chip: u32,
    pub bet: u32,
    pub insurance: bool,
    pub giveup: bool,
}

impl Banker {
    pub fn new() -> Self {
        Banker {
            lightcard: Vec::new(),
            darkcard: None,
            blackjack: false,
            flip_card: false,
        }
    }
    pub fn check_darkcard_is_ace(&self) -> bool {
        match self.darkcard.as_ref() {
            Some(x) => {
                x.get_rank() == Ace
            }
            None => panic!("Game Init error"),
        }
    }
    pub fn check_lightcard_is_ace(&self) -> bool {
        match self.lightcard.first() {
            Some(x) => {
                x.get_rank() == Ace
            }
            None => panic!("Game Init error"),
        }
    }
}
impl Human {
    pub fn new() -> Self {
        Human {
            lightcard: Vec::new(),
            blackjack: false,
            chip: 100,
            bet: 0,
            insurance: false,
            giveup: false,
        }
    }
    pub fn add_bet(&mut self, size: u32) -> bool {
        if size <= self.chip {
            self.chip -= size;
            self.bet += size;
            true
        } else {
            false
        }
    }
    //----------To adjust the human's chips---------------
    pub fn win(&mut self) {
        if self.blackjack {
            self.chip += self.bet * 3;
        } else {
            self.chip += self.bet * 2;
        }
        self.bet = 0;
    }
    pub fn lose(&mut self) {
        self.bet = 0;
    }
    pub fn tie(&mut self) {
        self.chip += self.bet;
        self.bet = 0;
    }
    //----------To adjust the human's chips---------------
    //----------NOT implemented in graphic----------------
    pub fn get_2xinsurance(&mut self) {
        if self.insurance {
            self.chip += self.bet;
        }
    }
    pub fn lose_insurance(&mut self) {
        self.bet /= 2;
    }
    //----------NOT implemented in graphic----------------
}
impl Player for Banker {
    fn compute_value(&self) -> u8 {
        // Count how many Ace, if value > 21, an Acel should be value:1
        let mut count_ace = 0;
        let mut total_value = 0;
        for x in self.lightcard.iter(){
            total_value += x.get_value();
            if x.get_rank() == Ace {
                count_ace += 1;
            }
        }
        if self.darkcard.is_some() {
            match self.darkcard.as_ref() {
                Some(x) => {
                    total_value += x.get_value();
                    if x.get_rank() == Ace {
                        count_ace += 1;
                    }
                }
                None => panic!("darkcard is empty, impossible"),
            }
        }
        if total_value > 21 {
            total_value -= 10 * count_ace;
            total_value
        } else {
            total_value
        }
    }
    fn draw_card(&mut self, deck: &mut Deck) {
        //Get a card, for lighter vector first, and get one for darkcard
        if self.lightcard.is_empty() {
            self.lightcard.insert(0, deck.0.pop().unwrap());
        } else if self.darkcard.is_none() {
            self.darkcard = Some(deck.0.pop().unwrap());
        } else {
            self.lightcard.push(deck.0.pop().unwrap());
        }
    }
    fn check_blackjack(&mut self) {
        if self.compute_value() == 21 {
            self.blackjack = true;
        } else {
            self.blackjack = false;
        }
    }
}
impl Player for Human {
    fn compute_value(&self) -> u8 {
        let mut count_ace = 0;
        let mut total_value = 0;
        for x in self.lightcard.iter(){
            total_value += x.get_value();
            if x.get_rank() == Ace {
                count_ace += 1;
            }
        }
        if total_value > 21 {
            total_value -= 10 * count_ace;
            total_value
        } else {
            total_value
        }
    }
    fn draw_card(&mut self, deck: &mut Deck) {
        self.lightcard.push(deck.0.pop().unwrap());
    }
    fn check_blackjack(&mut self) {
        if self.compute_value() == 21 {
            self.blackjack = true;
        } else {
            self.blackjack = false;
        }
    }
}
impl Stringfy for Banker {
    fn stringfy(&self) -> String {
        let mut res = String::new();
        for x in self.lightcard.iter(){
            res.push_str("lightcards: ");
            res.push_str(x.stringfy().as_str());
            res.push('\n')
        }
        if self.darkcard.is_some() {
            res.push_str("Darkcard: ");
            res.push_str(self.darkcard.as_ref().unwrap().stringfy().as_str());
            res.push('\n');
        }
        res
    }
}
impl Stringfy for Human {
    fn stringfy(&self) -> String {
        let mut res = String::new();
        for x in self.lightcard.iter(){
            res.push_str("lightcards: ");
            res.push_str(x.stringfy().as_str());
            res.push('\n')
        }
        res
    }
}
