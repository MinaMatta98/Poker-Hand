/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use thiserror::Error;

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

impl Card {
    fn value(&self) -> u8 {
        match self {
            Card::Ace => 13,
            Card::Two => 1,
            Card::Three => 2,
            Card::Four => 3,
            Card::Five => 4,
            Card::Six => 5,
            Card::Seven => 6,
            Card::Eight => 7,
            Card::Nine => 8,
            Card::Ten => 9,
            Card::Jack => 10,
            Card::Queen => 11,
            Card::King => 12,
        }
    }

    fn from_value(value: &str) -> Result<Card, CardError> {
        match value {
            "A" => Ok(Card::Ace),
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "10" => Ok(Card::Ten),
            "J" => Ok(Card::Jack),
            "Q" => Ok(Card::Queen),
            "K" => Ok(Card::King),
            _ => Err(CardError::IncorrectCard { card: value }),
        }
    }
}
//Parsing a suit may result in an error of incorrect chars are parsed.
//Maybe feed in a position later?
#[derive(Error, Debug)]
enum CardError<'a> {
    #[error(
        "{suit} does not translate to a correct suit. The only applicable suits are H, D, S, C"
    )]
    IncorrectSuit { suit: char },
    #[error(
        "{card} does not translate to a correct card. The only applicable cards span form 1-10 and include J, K, Q."
    )]
    IncorrectCard { card: &'a str },
}

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

//Hand here is defined as a pairing, e.g JH (Jack of Hearts). This may not be the actual definition of a hand.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Hand {
    suit: Suit,
    card: Card,
}

//Implementing Ord/PartialOrd will be necessary to compare n, n+1 chars. This would require a
//value hierarchy -> implement a value method for Card
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl Hand {
    //Mapping a char to a suit. Note that unwrap will be used.
    fn to_suit<'a>(suit: char) -> Result<Suit, CardError<'a>> {
        match suit.to_lowercase().to_string().as_str() {
            "h" => Ok(Suit::Hearts),
            "d" => Ok(Suit::Diamonds),
            "s" => Ok(Suit::Spades),
            "c" => Ok(Suit::Clubs),
            _ => Err(CardError::IncorrectSuit { suit }),
        }
    }

    //For every pairing (forgive the incorrect semantics), construct a Hand Struct
    fn retrieve_hand(hand: &str) -> Hand {
        let suit = hand.chars().last().unwrap();
        let val = hand.get(..hand.len() - 1).unwrap();
        Self {
            card: Card::from_value(val).unwrap(),
            suit: Self::to_suit(suit).unwrap(),
        }
    }

    //Args come in as an $[&str], where each pairing is seperated by a space char, therefore,
    //for each pairing, construct a Hand, such that a complete "Hand" is represented by a Vec<Hand>
    pub fn new_sorted(hands: &str) -> Vec<Hand> {
        let mut hands = hands
            .split_whitespace()
            .map(Self::retrieve_hand)
            .collect::<Vec<Hand>>();
        hands.sort_by(|a, b| b.card.partial_cmp(&a.card).unwrap());
        hands
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    //Need to sort cards in a programattic manner. Maybe initialize Hand::new()
    //&["4S 5S 7H 8D JC"] <-- All split by white space. split_whitespacee and arrange into
    //Vec<Hand>
    unimplemented!("Out of {:?}, which hand wins?", hands)
}
