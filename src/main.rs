use ::card::{ Deck, Player, Card, Value };
use std::error::Error;
use std::str::FromStr;
use std::io;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let n: usize = input("How many players?")?.trim().parse()?;
    let c: usize = input("How many cards should each player be dealt?")?.trim().parse()?;

    let mut players: Vec<Player> = Vec::new();
    for i in 0..n {
        players.push(Player::new(i as isize + 1));
    }

    let mut deck = Deck::new();
    for _ in 0..c {
        deck.deal_each(&mut players);
    }

    let mut winner: isize = -1;
    let mut index = 0;
    while winner < 0 {
        let i = index;
        let mut j = -1;

        // Select valid player to pick from and resolve to index
        {
            let player = &players[i];
            println!("{}", player);

            while j < 0 {
                j = match input("Who would you like to take from?")?.trim().parse::<isize>() {
                    Ok(x) if x != player.id as isize && x <= players.len() as isize && x > 0 => x - 1,
                    _ => continue,
                };
            }
        }

        // Select valid value of card to ask for
        let mut v: Option<Value> = None;
        while v.is_none() {
            v = match Value::from_str(input("Do you have any...")?.trim()) {
                Ok(x) => {
                    let player = &players[i];
                    if player.has_any(&Card::new_set(&x)) {
                        Some(x)
                    } else {
                        None
                    }
                },
                Err(_) => None,
            };
        }

        // Try to take card of selected value from selected player
        let mut card: Option<Card> = None;
        {
            let other = &mut players[j as usize];
            for c in Card::new_set(&v.unwrap()) {
                card = other.take(c);
                if card.is_some() {
                    break;
                }
            }
        }
        
        // If miss then go fish else player will get another go 
        if card.is_none() {
            println!("Go fish!");
            card = deck.draw();
            index += 1;
        }

        // If player has a full set then remove and increment score
        {
            let player = &mut players[i];
            match card {
                Some(x) => {
                    println!("Caught a {}", x);
                    let n = x.value.0;
                    player.give(x);

                    let set = Card::new_set(&Value::new(n));
                    if player.has_all(&set) {
                        let s = set.iter()
                            .map(|c| c.to_string())
                            .collect::<Vec<_>>()
                            .join(", ");

                        println!("Got a set! {}", s);
                        player.score += 1;
                        for c in set {
                            player.take(c);
                        }
                    }

                },
                None => println!("No cards left!"),
            }
        }
        
        // Check for a winner
        let mut max = 0;
        for p in &players {
            if p.hand_count() == 0 && p.score > max {
                max = p.score;
                winner = p.id;
            }
        }

        if index >= players.len() {
            index = 0;
        }
    }

    println!("Player {} wins!", winner + 1);
    Ok(())
}

fn input(message: &str) -> io::Result<String> {    
    print!("{} ", message);

    let mut s = String::new();
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut s)?;

    Ok(s)
}