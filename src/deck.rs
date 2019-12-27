use crate::card::Card;
use crate::suit::Suit;
use std::vec;
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
}

impl IntoIterator for Deck {
    type Item = Card;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}