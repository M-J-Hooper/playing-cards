use crate::card::Card;
use std::collections::{HashSet, hash_set};
use std::fmt;

#[derive(Debug)]
pub struct Player {
    pub id: usize,
    pub score: usize,
    pub hand: HashSet<Card>,
}

impl Player {
    pub fn new(id: usize) -> Player {
        Player { id, score: 0, hand: HashSet::new() }
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
        
        write!(f, "Player: id {}, score {}, cards {}", self.id, self.score, s)
    }
}

impl<'a> IntoIterator for &'a Player {
    type Item = &'a Card;
    type IntoIter = hash_set::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.hand.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::Deck;
    use std::str::FromStr;

    #[test]
    fn hand() {
        let mut p = Player::new(52);
        
        let c = Card::from_str("4-H").unwrap();
        p.deal(c.clone());
        assert!(p.has(&c.clone()));

        p.take(c.clone());
        assert!(!p.has(&c.clone()));
    }

    #[test]
    fn iter() {
        let mut player = Player::new(52);
        let mut deck = Deck::new();
        for i in 0..10 {
            assert_eq!(player.into_iter().count(), i);

            let card = deck.draw().unwrap();
            player.deal(card);
        }

        let mut n = 0;
        for _ in &player {
            n += 1;
        }
        assert_eq!(n, 10);
    }
}