mod lib;
use lib::model::card::*;
use lib::model::card::Suit::*;
use lib::model::card::Rank::*;
//;

fn main() {
    let a = Card::new_by_rank(Ace, Hearts);
    println!("{:?}", a.stringfy());
}
