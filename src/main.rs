mod lib;
use lib::model::*;
//;

fn main() {
    let a = Card::new_by_rank(Ace, Hearts);
    println!("{:?}", a.stringfy());
    let b = Card::new_by_rankid(1, Clubs);
    println!("{:?}", b.stringfy());
}
