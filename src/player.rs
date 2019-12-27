use crate::card::Card;
use std::collections::{HashSet, hash_set};
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

    pub fn has(&self, card: &Card) -> bool {
        self.hand.contains(card)
    }

    pub fn take(&mut self, card: Card) -> Option<Card> {
        if self.hand.remove(&card) {
            Some(card)
        } else {
            None
        }
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

impl IntoIterator for Player {
    type Item = Card;
    type IntoIter = hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.hand.into_iter()
    }
}