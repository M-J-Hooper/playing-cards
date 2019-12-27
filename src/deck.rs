use crate::card::Card;
use crate::suit::Suit;
use rand::seq::SliceRandom;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut v: Vec<Card> = vec![];
        for i in 1..13 {
            v.push(Card::new(i, Suit::Hearts));
            v.push(Card::new(i, Suit::Diamonds));
            v.push(Card::new(i, Suit::Clubs));
            v.push(Card::new(i, Suit::Spades));
        }

        let mut deck = Deck { cards: v };
        deck.shuffle();
        deck
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}