use crate::card::Card;
use crate::suit::Suit;
use crate::player::Player;
use std::slice;
use rand::seq::SliceRandom;

pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new() -> Deck {
        let mut v: Vec<Card> = vec![];
        for i in 1..14 {
            v.push(Card::new(i, Suit::Hearts));
            v.push(Card::new(i, Suit::Diamonds));
            v.push(Card::new(i, Suit::Clubs));
            v.push(Card::new(i, Suit::Spades));
        }

        let mut deck = Deck(v);
        deck.shuffle();
        deck
    }

    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut rand::thread_rng());
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.0.pop()
    }

    pub fn deal_each(&mut self, players: &mut Vec<Player>) {
        for p in players {
            match self.draw() {
                Some(c) => p.deal(c),
                None => break,
            }
        }
    }
}

impl<'a> IntoIterator for &'a Deck {
    type Item = &'a Card;
    type IntoIter = slice::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn iter() {
        let deck = Deck::new();
        let mut n = 0;
        for _ in &deck {
            n += 1;
        }
        assert_eq!(n, 52);
        assert_eq!(deck.into_iter().count(), 52);
    }

    #[test]
    fn unique() {
        let mut set: HashSet<Card> = HashSet::new();
        let mut deck = Deck::new();
        loop {
            let c = match deck.draw() {
                Some(x) => x,
                None => break,
            };
            assert!(!set.contains(&c));
            set.insert(c);
        }
    }

    #[test]
    fn shuffle() {
        let card = Deck::new().draw().unwrap();
        let limit = 1000;
        let mut n = 0;
        while card == Deck::new().draw().unwrap() && n < limit {
            n += 1;
        }
    }
}