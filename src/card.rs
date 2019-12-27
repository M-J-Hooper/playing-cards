use crate::suit::Suit;
use crate::error::ParseError;
use std::str;
use std::fmt;
use std::cmp;
use std::hash;

#[derive(cmp::Eq, cmp::PartialEq, hash::Hash)]
pub struct Card {
    number: usize,
    suit: Suit,
}

impl Card {
    pub fn new(number: usize, suit: Suit) -> Card {
        Card { number, suit }
    }

    fn fmt_number(&self) -> String {
        match self.number {
            1 => String::from("A"),
            11 => String::from("J"),
            12 => String::from("Q"),
            13 => String::from("K"),
            _ => self.number.to_string(),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.fmt_number(), self.suit.get_initial())
    }
}

impl str::FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split("-").collect();
        if v.len() != 2 {
            Err(ParseError::new(s, "A card string must have 2 parts: suit and number"))
        } else {
            let n = match v[0] {
                "A" => 1,
                "J" => 11,
                "Q" => 12,
                "K" => 13,
                _ => v[0].parse().unwrap(), // TODO: Unwrap -> ?
            };

            if n < 1 || n > 13 {
                Err(ParseError::new(s, "A card number must be between 1 and 13 inclusive"))
            } else {
                let suit = Suit::from_str(v[1])?;
                Ok(Card::new(n, suit))
            }
        }
    }
}