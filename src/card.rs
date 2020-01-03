use crate::suit::Suit;
use crate::value::Value;
use crate::error::ParseError;
use std::str;
use std::fmt;
use std::cmp;
use std::hash;

#[derive(Clone, Debug, cmp::Eq, cmp::PartialEq, hash::Hash)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Card {
    pub fn new(number: usize, suit: Suit) -> Card {
        Card { value: Value::new(number), suit }
    }

    pub fn new_set(value: &Value) -> Vec<Card> {
        vec![
            Card { value: value.clone(), suit: Suit::Hearts },
            Card { value: value.clone(), suit: Suit::Diamonds },
            Card { value: value.clone(), suit: Suit::Clubs },
            Card { value: value.clone(), suit: Suit::Spades },
        ]
    }

    pub fn new_suit(suit: &Suit) -> Vec<Card> {
        let mut v = Vec::new();
        for i in 1..14 {
            v.push(Card { value: Value(i), suit: suit.clone() });
        }
        v
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.value, self.suit)
    }
}

impl str::FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split("-").collect();
        if v.len() != 2 {
            Err(ParseError::new(s, "A card string must have 2 parts: suit and value"))
        } else {
            let value = Value::from_str(v[0])?;
            let suit = Suit::from_str(v[1])?;
            Ok(Card { value, suit })
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
        Card::from_str("1-S").expect_err("Invalid value");
        Card::from_str("1-M").expect_err("Invalid suit");
    }
}