use ::card::*;
use std::error::Error;
use std::str::FromStr;
use std::io;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let n: usize = input("How many players?")?.trim().parse()?;
    let c: usize = input("How many cards should each player be dealt?")?.trim().parse()?;

    // Create players
    let mut players: Vec<Player> = Vec::new();
    for i in 0..n {
        players.push(Player::new(i as isize + 1));
    }

    // Deal them in
    let mut deck = Deck::new();
    for _ in 0..c {
        deck.deal_each(&mut players);
    }

    // A player might have been dealt a complete set
    for player in &mut players {
        gofish::process_sets(player);
    }
 
    let mut winner = 0;
    let mut index = 0;
    while winner == 0 {
        let i = index;

        // Select valid player to pick from and resolve to index
        let j = gofish::select_player(&players, i)?;

        // Select valid value of card to ask for
        let v = gofish::select_value(&players[i])?;

        // Try to take card of selected value from selected player
        let mut card = gofish::try_take(&mut players[j], &v);
        
        // If miss then go fish else player will get another go 
        if card.is_none() {
            println!("Go fish!");
            card = deck.draw();
            index += 1;
        }

        // Give the player the card and check if it makes a set
        match card {
            Some(x) => {
                println!("Caught a {}", x);
                let value = x.value.clone();
                let player = &mut players[i];
                player.give(x);
                gofish::process_set(player, &value);
            },
            _ => println!("No cards left!"),
        }

        // Check for a winner
        winner = match gofish::try_winner(&players) {
            Some(x) => x.id,
            _ => 0,
        };

        if index >= players.len() {
            index = 0;
        }
    }

    println!("Player {} wins!", winner);
    Ok(())
}

fn input(message: &str) -> io::Result<String> {    
    print!("{} ", message);

    let mut s = String::new();
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut s)?;

    Ok(s)
}

mod gofish {
    use super::*;

    pub fn try_winner(players: &Vec<Player>) -> Option<&Player> {
        let mut max = 0;
        let mut winner = None;
        for p in players {
            if p.hand_count() == 0 && p.score > max {
                max = p.score;
                winner = Some(p);
            }
        }
        winner
    }

    pub fn select_player(players: &Vec<Player>, curr_player: usize) -> io::Result<usize> {
        let player = &players[curr_player];
        println!("{}", player);

        let mut j = -1;
        while j < 0 {
            j = match input("Who would you like to take from?")?.trim().parse::<isize>() {
                Ok(x) if x != player.id as isize 
                    && x <= players.len() as isize 
                    && x > 0 
                    && players[x as usize].hand_count() > 0 => x - 1,
                _ => continue,
            };
        }
        Ok(j as usize)
    }

    pub fn select_value(player: &Player) -> io::Result<Value> {
        let mut v: Option<Value> = None;
        while v.is_none() {
            v = match Value::from_str(super::input("Do you have any...")?.trim()) {
                Ok(x) => {
                    if player.has_any(&Card::new_set(&x)) {
                        Some(x)
                    } else {
                        None
                    }
                },
                Err(_) => None,
            };
        }
        Ok(v.unwrap())
    }

    pub fn try_take(other: &mut Player, value: &Value) -> Option<Card> {
        for c in Card::new_set(value) {
            let card = other.take(c);
            if card.is_some() {
                return card;
            }
        }
        None
    }

    pub fn process_set(player: &mut Player, value: &Value) {
        let set = Card::new_set(value);
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
    }

    pub fn process_sets(player: &mut Player) {
        for i in 1..14 {
            process_set(player, &Value::new(i));
        }
    }
}