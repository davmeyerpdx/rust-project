#[cfg(test)]
mod basic_banker_tests {

    #[test]
    #[should_panic]
    fn test_banker() {
       
        use crate::lib::model::card::Deck;
        use crate::lib::model::player::Player;
        use crate::lib::model::player::Banker;

        let mut d = Deck::new();
        let mut b = Banker::new();
        
        b.draw_card(&mut d);
        b.draw_card(&mut d);
        
        //drawing two cards neither of which is an ace
        assert!(b.check_darkcard_is_ace());
        assert!(b.check_lightcard_is_ace());
    }
}


#[cfg(test)]
mod basic_human_tests {

    #[test]
    //BET
    fn test_human_bet () {

        use crate::lib::model::player::Human;
        let mut h = Human::new();
        
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

        use crate::lib::model::player::Human;
        let mut h = Human::new();
        
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
        
        use crate::lib::model::player::Human;
        let mut h = Human::new();

        h.bet = 1;
        h.lose();

        //amount bet will go to 0 with loss
        assert!(h.bet == 0);
    }
    
    #[test]
    //TIE
    fn test_human_tie () {

        use crate::lib::model::player::Human;
        let mut h = Human::new();

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

        use crate::lib::model::player::Human;
        let mut h = Human::new();

        h.chip      = 1;
        h.bet       = 1;
        h.insurance = true;
        h.get_2xinsurance();
        
        assert!(h.chip == 2);
    }
    
    #[test]
    //GET 2X INSURANCE
    fn test_human_loseInsurance () {

        use crate::lib::model::player::Human;
        let mut h = Human::new();

        h.bet = 2;
        h.lose_insurance();

        assert!(h.bet == 1);
    }
}


#[cfg(test)]
mod player_banker_tests {

    #[test]
    //BET
    fn test_banker_compute_value () {
        
        use crate::lib::model::card::Deck;
        use crate::lib::model::player::Player;
        use crate::lib::model::player::Banker;

        let mut d = Deck::new();
        let mut b = Banker::new();

        //before drawing cards hand value at zero
        let mut i: u8 = b.compute_value();
        assert!(i == 0);

        //after drawing cards hand value greater than zero
        b.draw_card(&mut d);

        i = b.compute_value();
        assert!(i > 0);
    }

    #[test]
    //DRAW
    fn test_banker_draw () {
        
        use crate::lib::model::card::Deck;
        use crate::lib::model::player::Player;
        use crate::lib::model::player::Banker;

        let mut d = Deck::new();
        let mut b = Banker::new();

        //we have no cards in hand at start
        assert!(b.lightcard.is_empty());
        assert!(b.darkcard.is_none());

        //drawing a card will be put into the lightcard slot...
        b.draw_card(&mut d);
        assert_eq!(false, b.lightcard.is_empty());
    
        //...but darkcard slot is still empty
        assert!(b.darkcard.is_none());
        
        //second card goes in the dark cards slot
        b.draw_card(&mut d);
        assert_eq!(false, b.darkcard.is_none());
    }

    #[test]
    //BLACKJACK
    fn test_banker_blackjack () {
        
        use crate::lib::model::card::Deck;
        use crate::lib::model::player::Player;
        use crate::lib::model::player::Banker;

        let mut d = Deck::new();
        let mut b = Banker::new();

        b.blackjack = true;
        b.check_blackjack();

        //check_blackjack w/o correct value in hand switches b.blackjack to false
        assert_eq!(false, b.blackjack);
    }
}


#[cfg(test)]
mod player_human_tests {

    #[test]
    //BET
    fn test_human_compute_value () {
        
        use crate::lib::model::card::Deck;
        use crate::lib::model::player::Player;
        use crate::lib::model::player::Human;

        let mut d = Deck::new();
        let mut h = Human::new();

        //before drawing cards hand value at zero
        let mut i: u8 = h.compute_value();
        assert!(i == 0);

        //after drawing cards hand value greater than zero
        h.draw_card(&mut d);

        i = h.compute_value();
        assert!(i > 0);
    }

    #[test]
    //DRAW
    fn test_human_draw () {
        
        use crate::lib::model::card::Deck;
        use crate::lib::model::player::Player;
        use crate::lib::model::player::Human;

        let mut d = Deck::new();
        let mut h = Human::new();

        //we have no cards in hand at start
        assert!(h.lightcard.is_empty());

        //drawing a card will be put into the lightcard slot
        h.draw_card(&mut d);
        assert_eq!(false, h.lightcard.is_empty());
    }

    #[test]
    //BLACKJACK
    fn test_human_blackjack () {
        
        use crate::lib::model::card::Deck;
        use crate::lib::model::player::Player;
        use crate::lib::model::player::Human;

        let mut d = Deck::new();
        let mut h = Human::new();

        h.blackjack = true;
        h.check_blackjack();

        //check_blackjack w/o correct value in hand switches b.blackjack to false
        assert_eq!(false, h.blackjack);
    }
}
