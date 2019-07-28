enum Suit{
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}
enum Rank{
    Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, 
}
enum Color{
    Red,
    Black,
}

pub struct Card{
    rank: Rank,
    suit: Suit,
    color: Color,
    value: i32,
}

pub struct Deck(Vec<Card>);
