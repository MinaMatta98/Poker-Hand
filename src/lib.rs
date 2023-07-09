/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
mod ranking;
mod hand;
use std::cmp::Ordering;
use ranking::Ranking;
use crate::hand::Hand;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    //Need to sort cards in a programattic manner. Maybe initialize Hand::new()
    //&["4S 5S 7H 8D JC"] <-- All split by white space. split_whitespacee and arrange into
    //Vec<Hand>
    let mut final_hand: Vec<&str> = Vec::new();
    let mut max_value = 0;

    // Iter through each hand and construct Vec<Hand>. Determine the value of the ranking and
    // compare. If eq, then a secondary check via Ranking::cmp needs to be determined. Else,
    // return the greater hand.
    for hand in hands.iter() {
        let mut init_hand = Hand::new_sorted(hand);
        let ranking = Ranking::determine_ranking(&mut init_hand);
        let value = ranking.value();
        if value.eq(&max_value) {
            match Ranking::cmp(
                ranking,
                init_hand,
                Hand::new_sorted(final_hand.first().unwrap()),
            ) {
                Ordering::Greater => final_hand = vec![hand],
                Ordering::Less => (),
                Ordering::Equal => final_hand.push(hand),
            }
        } else if value.gt(&max_value) {
            max_value = value;
            final_hand = vec![hand];
        }
    }

    //Always return Some(_) as Error handling already implemented.
    Some(final_hand)
}
