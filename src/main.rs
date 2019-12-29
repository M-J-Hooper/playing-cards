use ::card::{ Deck, Player, Card, Value };
use std::error::Error;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    
    println!("How many players?");
    std::io::stdin().read_line(&mut s)?;
    let n: usize = s.trim().parse()?;

    println!("How many cards should each player be dealt?");
    s.clear();
    std::io::stdin().read_line(&mut s)?;
    let c: usize = s.trim().parse()?;

    let mut players: Vec<Player> = Vec::new();
    for i in 0..n {
        players.push(Player::new(i + 1));
    }

    let mut deck = Deck::new();
    for _ in 0..c {
        deck.deal_each(&mut players);
    }

    let mut winner = 0;
    while winner == 0 {
        for i in 1..(players.len() + 1) {
            let mut j = 0;
            {
                let player = &players[i - 1];
                println!("{}", player);

                while j == 0 {
                    println!("Who would you like to take from?");
                    s.clear();
                    std::io::stdin().read_line(&mut s)?;
                    j = match s.trim().parse::<usize>() {
                        Ok(x) if x != player.id && x <= players.len() && x > 0 => x,
                        _ => continue,
                    };
                }
            }

            let mut v: Option<Value> = None;
            while let None = v {
                println!("Do you have any...");
                s.clear();
                std::io::stdin().read_line(&mut s)?;
                v = match Value::from_str(&s) {
                    Ok(x) => Some(x),
                    Err(_) => None,
                };
            }

            let mut card: Option<Card> = None;
            {
                let other = &mut players[j - 1];
                for c in Card::set(v.unwrap()) {
                    card = other.take(c);
                    if let Some(_) = card {
                        break;
                    }
                }
            }
            
            if let None = card {
                println!("Go fish!");
                card = deck.draw();
            }

            {
                let player = &mut players[i - 1];
                match card {
                    Some(c) => {
                        println!("Caught a {}", c);
                        let n = c.value().n();
                        player.hand.insert(c);

                        let mut count = 1;
                        for cc in Card::set(Value::new(n)) {
                            if cc.value().n() == n {
                                count += 1;
                            }
                        }

                        if count == 4 {
                            for cc in Card::set(Value::new(n)) {
                                player.take(cc);
                            }
                            player.score += 1;
                        }

                    },
                    None => println!("No cards left"),
                }
            }
            
            let mut max = 0;
            for p in &players {
                if p.hand.len() == 0 && p.score > max {
                    max = p.score;
                    winner = p.id;
                }
            }
            if winner > 0 {
                break;
            }
        }
    }

    println!("Player {} wins!", winner);
    Ok(())
}