use std::string::ToString;
use std::vec::Vec;

enum CardSuit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}
impl ToString for CardSuit {
    fn to_string(&self) -> String {
        let s: &str = match self {
            CardSuit::Spades => "S",
            CardSuit::Clubs => "C",
            CardSuit::Hearts => "H",
            CardSuit::Diamonds => "D"
        };
        s.to_owned()
    }
}
impl CardSuit {
    fn is_red(&self) -> bool {
        if self == CardSuit::Spades || self == CardSuit::Clubs {
            return false;
        }
        true
    }

    fn matches_suit(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
    
    fn matches_color(&self, other: &Self) -> bool {
        self.is_red() == other.is_red()
    }
}


enum CardValue {
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
impl ToString for CardValue {
    fn to_string(&self) -> String {
        let s: &str = match self {
            CardValue::Ace => "A",
            CardValue::Two => "2",
            CardValue::Three => "3",
            CardValue::Four => "4",
            CardValue::Five => "5",
            CardValue::Six => "6",
            CardValue::Seven => "7",
            CardValue::Eight => "8",
            CardValue::Nine => "9",
            CardValue::Ten => "T",
            CardValue::Jack => "J",
            CardValue::Queen => "Q",
            CardValue::King => "K",
        };
        s.to_owned()
    }
}
impl CardValue {
    fn numeric(&self) -> u8 {
        let n = match self {
            CardValue::Ace => 1,
            CardValue::Two => 2,
            CardValue::Three => 3,
            CardValue::Four => 4,
            CardValue::Five => 5,
            CardValue::Six => 6,
            CardValue::Seven => 7,
            CardValue::Eight => 8,
            CardValue::Nine => 9,
            CardValue::Ten => 10,
            CardValue::Jack => 11,
            CardValue::Queen => 12,
            CardValue::King => 13,
        };
        n
    }
    
    fn from_numeric(id: &u8) -> Self {
        match id {
            1 => CardValue::Ace,
            2 => CardValue::Two,
            3 => CardValue::Three,
            4 => CardValue::Four,
            5 => CardValue::Five,
            6 => CardValue::Six,
            7 => CardValue::Seven,
            8 => CardValue::Eight,
            9 => CardValue::Nine,
            10 => CardValue::Ten,
            11 => CardValue::Jack,
            12 => CardValue::Queen,
            13 => CardValue::King,
            _ => panic!("There was an error!"),
        }
    }
    
    fn next(&self) -> Option<Self> {
        let mut n: u8 = self.numeric();
        n += 1;
        if n == 14 {
            return None;
        }
        Some(Self::from_numeric(&n))
    }
    
    fn prev(&self) -> Option<Self> {
        let mut n: u8 = self.numeric();
        n -= 1;
        if n == 0 {
            return None;
        }
        Some(Self::from_numeric(&n))
    }
}

struct Card {
    id: String,
    suit: CardSuit,
    value: CardValue,
}
impl Card {
    fn new(suit: &CardSuit, value: &CardValue) -> Self {
        // Generate an ID.
        let mut id: String = String::new();
    }
    fn get_next(&self) -> () {
        
    }
}

fn main() {
    println!("Hello, world!");
}
