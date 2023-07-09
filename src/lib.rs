mod hand;
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
mod ranking;
use std::{cmp::Ordering, println};

use crate::hand::Hand;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    //Need to sort cards in a programattic manner. Maybe initialize Hand::new()
    //&["4S 5S 7H 8D JC"] <-- All split by white space. split_whitespacee and arrange into
    //Vec<Hand>
    let mut max_value = 0;
    let mut final_hand = Vec::new();

    for hand in hands {
        let mut init_hand = Hand::new_sorted(hand);
        let ranking = ranking::Ranking::determine_ranking(&mut init_hand).value();
        match ranking.cmp(&max_value) {
            Ordering::Less => (),
            Ordering::Equal => final_hand.push(hand.to_owned()),
            Ordering::Greater => {
                max_value = ranking;
                final_hand = vec![hand];
            }
        }
    }
    Some(final_hand)
}
