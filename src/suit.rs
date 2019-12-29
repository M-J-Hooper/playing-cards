use crate::error::ParseError;
use std::str;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Hearts => "H",
            Self::Diamonds => "D",
            Self::Clubs => "C",
            Self::Spades => "S",
        };
        write!(f, "{}", s)
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
    use std::str::FromStr;

    #[test]
    fn hearts_colour() {
        assert!(Suit::Hearts.is_red());
        assert!(!Suit::Hearts.is_black());
    }

    #[test]
    fn from_str() {
        assert_eq!(Suit::from_str("C").unwrap(), Suit::Clubs);
        Suit::from_str("A").expect_err("Invalid suit");
    }
}