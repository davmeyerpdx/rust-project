use crate::*;

pub trait Player{
    fn compute_value(&self) -> u8;
    fn draw_card(&mut self, deck: &mut Deck);
    fn check_blackjack(&mut self);
}

pub struct Banker{
    pub lightcard: Vec<Card>,
    pub darkcard: Option<Card>,
    pub blackjack: bool,
    pub flip_card: bool,
}
pub struct Human{
    lightcard: Vec<Card>,
    pub blackjack: bool,
    pub chip: u32,
    pub bet: u32,
    pub insurance: bool,
    pub giveup: bool,
}

impl Banker{
    pub fn new() -> Self{
        Banker{
            lightcard: Vec::new(),
            darkcard: None,
            blackjack: false,
            flip_card: false,
        }
    }
    pub fn check_darkcard_is_ace(&self) -> bool {
        match self.darkcard.as_ref(){
            Some(x) => {
                if x.get_rank() == Ace {
                    return true;
                } else {
                    return false;
                }
            },
            None => panic!("Game Init error"),
        }


    }
     pub fn check_lightcard_is_ace(&self) -> bool {
        match self.lightcard.first(){
            Some(x) => {
                if x.get_rank() == Ace {
                    return true;
                } else {
                    return false;
                }
            },
            None => panic!("Game Init error"),
        }
    }
}

#[cfg(test)]
mod banker_tests {

    #[test]
    #[should_panic]
    fn test_banker() {
       
        use super::card::Deck;
        use crate::lib::model::player::Player;

        let mut d = Deck::new();
        //d.shuffle();

        let mut b = super::Banker {
          
            lightcard: Vec::new(),
            darkcard:  None,
            blackjack: false,
            flip_card: false
        };

        b.draw_card(&mut d);
        b.draw_card(&mut d);
        
        assert!(b.check_darkcard_is_ace());
        assert!(b.check_lightcard_is_ace());
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
    pub fn add_bet(&mut self, size: u32) -> bool {
        if size <= self.chip {
            self.chip -= size;
            self.bet += size;
            return true;
        }
        else {
            return false;
        }
    }
    pub fn win(&mut self) {
        if self.blackjack{
            self.chip += self.bet*3;
        }
        else {
            self.chip += self.bet*2;
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
    pub fn get_2xinsurance(&mut self){
        if self.insurance == true{
            self.chip += self.bet;
        }
    }
    pub fn lose_insurance(&mut self){
        self.bet /= 2;
    }
}
        
/*
{
    
    lightcard: Vec::new(),
    blackjack: false,
    chip: 102,
    bet: 0,
    insurance: false, 
    giveup: false,
};
*/

#[cfg(test)]
mod Human_tests {

    #[test]
    //BET
    fn test_human_bet () {

        let mut h = super::Human::new();
        
        h.chip = 102;

        //able to add the bet
        assert!(h.add_bet(101));
        //is bet value updated?
        assert!(h.bet == 101);

        //show that we cant bet when we dont have enough money
        assert_eq!(false, h.add_bet(100));
    }
    
    #[test]
    //WIN
    fn test_human_win () {

        let mut h = super::Human::new();
        
        h.chip      = 1;
        h.blackjack = true;
        h.bet       = 1;
        h.win();

        //winnings with blackjack = x + 3x
        assert!(h.chip == 4);

        h.chip      = 1;
        h.blackjack = false;
        h.bet       = 1;
        h.win();

        //winnings w/o blackjack = x + 2x
        assert!(h.chip == 3);
    }

    #[test]
    //LOSE
    fn test_human_lose () {
        
        let mut h = super::Human::new();

        h.bet = 1;
        h.lose();

        //amount bet will go to 0 with loss
        assert!(h.bet == 0);
    }
    
    #[test]
    //TIE
    fn test_human_tie () {

        let mut h = super::Human::new();

        h.chip = 1;
        h.bet  = 1;
        h.tie();

        //get chip back from bet
        assert!(h.chip == 2);
        //chips in bet get set back to zero
        assert!(h.bet  == 0);
    }

    #[test]
    //GET 2X INSURANCE
    fn test_human_getInsurance () {

        let mut h = super::Human::new();

        h.chip      = 1;
        h.bet       = 1;
        h.insurance = true;
        h.get_2xinsurance();
        
        assert!(h.chip == 2);
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
        if self.darkcard.is_some(){
            match self.darkcard.as_ref(){
                Some(x) => {
                    total_value += x.get_value();
                    if x.get_rank() == Ace{
                        count_ace += 1;
                    }
                },
                None => panic!("darkcard is empty, impossible")
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
        if self.lightcard.is_empty(){
            self.lightcard.insert(0, deck.0.pop().unwrap());
        } else if self.darkcard.is_none() {
            self.darkcard = Some(deck.0.pop().unwrap());
        } else {
            self.lightcard.push(deck.0.pop().unwrap());
        }
    }
    fn check_blackjack(&mut self){
        if self.compute_value() == 21 {
            self.blackjack = true;
        } else {
            self.blackjack = false;
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
    fn check_blackjack(&mut self){
        if self.compute_value() == 21 {
            self.blackjack = true;
        } else {
            self.blackjack = false;
        }
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
impl Stringfy for Human{
    fn stringfy(&self) -> String{
        let mut res = String::new();
        let mut it = self.lightcard.iter();
        loop{
            match it.next(){
                Some(x) => {res.push_str("lightcards: "); res.push_str(x.stringfy().as_str()); res.push('\n')},
                None => break
            }
        }
        res
    }
}
