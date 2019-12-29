use crate::suit::Suit;
use crate::error::ParseError;
use std::str;
use std::fmt;
use std::cmp;
use std::hash;

#[derive(Clone, Debug, cmp::Eq, cmp::PartialEq, hash::Hash)]
pub struct Card {
    number: usize,
    suit: Suit,
}

impl Card {
    pub fn new(number: usize, suit: Suit) -> Card {
        Card { number, suit }
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
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
        write!(f, "{}-{}", self.fmt_number(), self.suit)
    }
}

impl str::FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split("-").collect();
        if v.len() != 2 {
            Err(ParseError::new(s, "A card string must have 2 parts: suit and number"))
        } else {
            let n: usize = match v[0].parse() {
                Ok(x) if x > 1 && x < 11 => x,
                _ => match v[0] {
                    "A" => 1,
                    "J" => 11,
                    "Q" => 12,
                    "K" => 13,
                    _ => return Err(ParseError::new(s, "Invlid face card initial")),
                },
            };

            let suit = Suit::from_str(v[1])?;
            Ok(Card::new(n, suit))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn from_str() {
        let card = Card::from_str("A-S").unwrap();
        assert_eq!(card, Card::new(1, Suit::Spades))
    }

    #[test]
    fn to_string() {
        let card = Card::new(13, Suit::Diamonds);
        assert_eq!(card.to_string(), "K-D");
    }

    #[test]
    fn validation() {
        Card::from_str("AS").expect_err("No dash");
        Card::from_str("A--S").expect_err("Two dash");
        Card::from_str("1-S").expect_err("Invalid number");
        Card::from_str("1-M").expect_err("Invalid suit");
    }
}