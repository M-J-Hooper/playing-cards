use crate::card::Card;
use std::collections::HashSet;
use std::fmt;

pub struct Player {
    id: usize,
    hand: HashSet<Card>,
}

impl Player {
    pub fn new(id: usize) -> Player {
        Player { id, hand: HashSet::new() }
    }

    pub fn deal(&mut self, card: Card) {
        self.hand.insert(card);
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.hand.iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        
        write!(f, "Player {}: {}", self.id, s)
    }
}