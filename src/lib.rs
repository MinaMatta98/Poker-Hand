/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

//Implementing types for different suits will allow for checking equality later
//if necessary
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

// Each card type is necessary when checking for equality and attributing a value
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    unimplemented!("Out of {:?}, which hand wins?", hands)
}
