# playing-cards
Learning Rust by playing cards.

```rust
let mut player = Player::new(42);
let mut deck = Deck::new();
let card = deck.draw();
player.give(card);

let card = Card::from_str("Q-H")?;
player.give(card);

player.score += 13;
println!("{}", player) // Player 42 with score 13: A-S, Q-H
```

See `main.rs` for an example implementation of Go Fish. 
