mod card;
mod suit;
mod error;
mod player;
mod deck;

use crate::deck::Deck;
use crate::player::Player;

fn main() {
    let mut deck = Deck::new();
    let card = deck.draw().unwrap();
    println!("Is your card {}?", card);

    let mut p = Player::new(42);
    for _ in 0..5 {
        p.deal(deck.draw().unwrap());
    }
    println!("Next up: {}", p);
}   