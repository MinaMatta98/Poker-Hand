use std::cmp::Ordering;

//Need this as it has it automatically implements unique. Rules did not specify not to use.
//group_by may also be necessary.
use crate::hand::{Card, Hand};
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
            // Ensures that there is no conflict with flush
            && hand
                .iter()
                .filter(|&h| {
                    hand.iter()
                        .filter(|&x| x.card.value() == h.card.value())
                        .count()
                        > 1
                })
                .unique_by(|h| h.card.value())
                .count()
                < 2
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
        let straight = if let Some(ace) = hand.iter().position(|hand| hand.card == Card::AceHigh) {
            //AceHigh is initialized with a value of 13. Need to check for an ace-high and ace-low
            //by ensuring that all values are contained within the next two arr[].
            let values_low = [4, 3, 2, 1, 13];
            let values_high = [12, 11, 10, 9, 13];
            let low = hand
                .iter()
                .all(|hand| values_low.contains(&hand.card.value()));
            if low {
                hand.get_mut(ace).unwrap().card = Card::AceLow;
            };

            low || hand
                .iter()
                .all(|hand| values_high.contains(&hand.card.value()))
        } else {
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

    //Where rankings are equal, it is necessary to apply eq rules under:
    //https://en.wikipedia.org/wiki/List_of_poker_hands.
    //lhs and rhs are vecs of different hands to be compared.
    pub fn cmp(ranking: Self, lhs: Vec<Hand>, rhs: Vec<Hand>) -> Ordering {
        //Generic closure for grouping by values.

        let split_vec =
            |input_vec: &Vec<Hand>, required_length: usize, initial_length: usize| -> Vec<Hand> {
                let input_vec: Vec<Vec<_>> = input_vec
                    .iter()
                    .group_by(|&hand| hand.card.value())
                    .into_iter()
                    .map(|(_, group)| group.collect())
                    .collect();

                //Return a Vec<Vec<_>> after iterating and finding a specific length. 
                //Useful for ensuring that values can be grouped for both lhs and rhs and that
                //values in (lets say) a flush's tripplets can be retrieved.
                input_vec
                    .iter()
                    .find(|vec| vec.len() == required_length)
                    .map(|hand| hand.to_owned())
                    .unwrap_or_else(|| {
                        input_vec
                            .iter()
                            .filter(|vec| vec.len() != initial_length)
                            .flat_map(|hand| hand.to_owned())
                            .collect()
                    })
                    .into_iter()
                    .map(|hand| hand.to_owned())
                    .collect()
            };

        let assess_value = |lhs: &Vec<Hand>,
                            rhs: &Vec<Hand>,
                            initial_split: usize,
                            // Could be implemented as impl Fn(Vec<Hand>, Vec<Hand>) -> Ordering with
                            // static dispatch instead of dynamic dispatch with optimization.
                            // Callback is to be executed where equality is determined past the first check. 
                            callback: Box<dyn Fn(Vec<Hand>, Vec<Hand>) -> Ordering>|
         -> Ordering {
            let lhs_split = split_vec(lhs, initial_split, initial_split);
            let rhs_split = split_vec(rhs, initial_split, initial_split);

            match lhs_split
                .first()
                .unwrap()
                .card
                .value()
                .cmp(&rhs_split.first().unwrap().card.value())
            {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => {
                     // This is the second check, i.e, it has been determined that lhs and rhs are equal
                     // based on the initial main check, ie, in a full house, checking the
                    // tripplet. As the remaining checks are different, implement via a callback
                    // Fn(_) -> Ordering.
                    let lhs = split_vec(lhs, 5 - initial_split, initial_split);
                    let rhs = split_vec(rhs, 5 - initial_split, initial_split);
                    callback(lhs, rhs)
                }
                Ordering::Greater => Ordering::Greater,
            }
        };

        match ranking {
            Ranking::FourOfAKind => assess_value(
                &lhs,
                &rhs,
                4,
                Box::new(|lhs: Vec<Hand>, rhs: Vec<Hand>| {
                    //Compare the remaining card
                    lhs.first()
                        .unwrap()
                        .card
                        .value()
                        .cmp(&rhs.first().unwrap().card.value())
                }),
            ),
            Ranking::FullHouse => assess_value(
                &lhs,
                &rhs,
                3,
                Box::new(|lhs, rhs| {
                    //Compare the remaining two cards (1 only needed as equal)
                    lhs.first()
                        .unwrap()
                        .card
                        .value()
                        .cmp(&rhs.first().unwrap().card.value())
                }),
            ),
            Ranking::ThreeOfAKind => assess_value(
                &lhs,
                &rhs,
                3,
                Box::new(|lhs: Vec<Hand>, rhs: Vec<Hand>| {
                    //Check for three of a kind. If lhs and rhs are eq, 
                    //then return this closure for the highest value in remainder.
                    lhs.iter()
                        .max_by_key(|hand| hand.card.value())
                        .unwrap()
                        .card
                        .value()
                        .cmp(
                            &rhs.iter()
                                .max_by_key(|hand| hand.card.value())
                                .unwrap()
                                .card
                                .value(),
                        )
                }),
            ),
            _ => {
                //General comparison of each item in order.
                let combined: Vec<_> = lhs.iter().zip(&rhs).collect();
                if !combined.is_empty() {
                    for (first_hand_val, second_hand_val) in combined {
                        match first_hand_val
                            .card
                            .value()
                            .cmp(&second_hand_val.card.value())
                        {
                            Ordering::Greater => return Ordering::Greater,
                            Ordering::Less => return Ordering::Less,
                            Ordering::Equal => {
                                // Continue loop
                            }
                        }
                    }
                }
                Ordering::Equal
            }
        }
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
