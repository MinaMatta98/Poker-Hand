//Need this as it has it automatically implements unique. Rules did not specify not to use.
//group_by may also be necessary.
use crate::hand::Hand;
use itertools::Itertools;

// Each ranking must implemented in an Enum to value it later. Sort in order
#[derive(Debug)]
pub enum Ranking {
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
    //fn for determining how many of a specific kind there are.
    //Using unique_by means that a final vec is filtered. So if 4 are equal, two cards remain.
    //can use the same fn with different counts as a requirement for 3x, 4x, 5x.
    fn of_a_kind(hand: &[Hand], requirement: usize) -> bool {
        hand.iter()
            .unique_by(|hand| hand.card.value())
            .count()
            .eq(&requirement)
    }

    // Collecting into multiple vecs of different types. This would mean that all values are
    // grouped together. If we iter and count over this and two remain, that means full-house.
    fn partition_hands(hand: &mut [Hand]) -> Vec<Hand> {
        hand.iter()
            .unique_by(|hand| hand.card.value())
            .map(|hand| hand.to_owned())
            .collect()
    }

    pub fn determine_ranking(hand: &mut [Hand]) -> Self {
        //All five must be of the same value, via card.value()
        if Self::of_a_kind(hand, 1) {
            return Self::FiveOfAKind;
        }

        //A flush requires that all the suits are the same. Get the first item's 
        //suit and match all against it.
        let flush = hand
            .iter()
            .all(|current_hand| current_hand.suit == hand[0].suit);

        // It's required that n = (n - 1) +1. Therefore, obtain two vecs and compare based on
        // position. Two possible solutions (of the top of my head), vec.iter().enumerate() w/
        // (index, item) against (index+1, item).
        // Could also use zip, such that [&[1,2,3,4,5], &[6,7,8,9,10]] -> [(1,6),(2,7),(3,8),(4,9),(5,10)]
        // Use the same vec, but skip 1 index, such that Vec(Hand(index), Hand(index+1))
        let straight = {
            hand.iter()
                .zip(hand.iter().skip(1))
                .all(|(a, b)| a.card.value() == b.card.value() + 1)
        };

        //
        if flush && straight {
            return Self::StraightFlush;
        }

        // Four of a kind means that two cards remain.
        if Self::of_a_kind(hand, 2) {
            return Self::FourOfAKind;
        }

        // Four of a kind means that two cards remain.
        let partitioned_hands = Self::partition_hands(hand);

        // Two vecs seperated means that two vecs of equal values exist.
        if partitioned_hands.len() == 2 {
            return Self::FullHouse;
        }

        if flush {
            return Self::Flush;
        }

        if straight {
            return Self::Straight;
        }

        // Three of a kind means that 3 cards remain (by value).
        if Self::of_a_kind(hand, 3) {
            return Self::ThreeOfAKind;
        }

        // Vec.len after partition can help determine type if it already passes other tests.
        match partitioned_hands.len() {
            3 => {
                return Self::TwoPair;
            }
            4 => {
                return Self::OnePair;
            }
            _ => {}
        }

        Self::HighCard
    }

    // All ranking must be attributed to an initial value.
    pub fn value(&self) -> u8 {
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
