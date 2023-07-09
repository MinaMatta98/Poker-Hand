use crate::hand::{Hand, Card};

// Each ranking must implemented in an Enum to value it later. Sort in order
#[derive(Debug)]
enum Ranking {
    // Five of each <--Highest
    FiveOfAKind,
    // Straight + Flush
    StraightFlush,
    // Four of a certain card NUMBER
    FourOfAKind,
    // Three identical cards numbers, 2 other identical numbers
    FullHouse,
    // Match by same suit, ie. all(|(m, n)| m.suit == n.suit)
    Flush,
    // Sequential, ie 10, 9, 8, 7, 6
    Straight,
    // Three identical card numbers. When cards are sorted, match with fold(0, |acc, (a, b)|
    // a.card.value == b.card.value -> acc +1 || acc)
    ThreeOfAKind,
    // Can use a duplicate match, such that if three cards out of five are left behind, it must
    // mean that two duplicates existed.
    TwoPair,
    // Same as above, but 4 remain.
    OnePair,
    // If ! others, then this
    HighCard,
}

impl Ranking {
    fn value(&self) -> u8 {
        match self {
            Ranking::FiveOfAKind => 10,
            Ranking::StraightFlush => 9,
            Ranking::FourOfAKind => 8,
            Ranking::FullHouse => 7,
            Ranking::Flush => 6,
            Ranking::Straight => 5,
            Ranking::ThreeOfAKind => 4,
            Ranking::TwoPair => 3,
            Ranking::OnePair => 2,
            Ranking::HighCard => 1,
        }
    }
}
