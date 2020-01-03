use crate::error::ParseError;
use std::str;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Value(pub usize);

impl Value {
    pub fn new(number: usize) -> Value {
        if number < 1 || number > 13 {
            panic!("Value must be between 1 and 13 inclusive");
        }
        Value(number)
    }

    pub fn is_face(&self) -> bool {
        self.0 < 14 && self.0 > 10
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self.0 {
            1 => String::from("A"),
            11 => String::from("J"),
            12 => String::from("Q"),
            13 => String::from("K"),
            _ => self.0.to_string(),
        };
        write!(f, "{}", s)
    }
}

impl str::FromStr for Value {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = match s.parse() {
            Ok(x) if x > 1 && x < 11 => x,
            _ => match s {
                "A" => 1,
                "J" => 11,
                "Q" => 12,
                "K" => 13,
                _ => return Err(ParseError::new(s, "Invlid face card initial")),
            },
        };
        Ok(Value(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn hearts_colour() {
        assert!(Value(11).is_face());
        assert!(!Value(10).is_face());
    }

    #[test]
    fn from_str() {
        assert_eq!(Value::from_str("K").unwrap(), Value(13));
        Value::from_str("13").expect_err("Invalid suit");
    }
}