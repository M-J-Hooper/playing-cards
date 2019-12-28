use crate::error::ParseError;
use std::str;
use std::cmp;
use std::hash;

#[derive(Clone, Debug, cmp::Eq, cmp::PartialEq, hash::Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub fn is_red(&self) -> bool {
        match *self {
            Self::Hearts => true,
            Self::Diamonds => true,
            _ => false,
        }
    } 

    pub fn is_black(&self) -> bool {
        !self.is_red()
    }

    pub fn get_initial(&self) -> &str {
        match self {
            Self::Hearts => "H",
            Self::Diamonds => "D",
            Self::Clubs => "C",
            Self::Spades => "S",
        } 
    }
}

impl str::FromStr for Suit {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "H" => Ok(Self::Hearts),
            "D" => Ok(Self::Diamonds),
            "C" => Ok(Self::Clubs),
            "S" => Ok(Self::Spades),
            _ => Err(ParseError::new(s, "Invalid suit initial")),
        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hearts_colour() {
        assert!(Suit::Hearts.is_red());
        assert!(!Suit::Hearts.is_black());
    }
}