mod card;
mod suit;
mod error;

use std::str::FromStr;
use crate::card::Card;

fn main() {
    let card = Card::from_str("A S").unwrap();
    println!("Is your card {}?", card);
}
