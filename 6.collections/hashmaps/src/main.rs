use std::collections::HashMap;

fn main() {
    let mut players = HashMap::new();
    players.insert("Mary Poppins", 120);
    players.insert("Mike Mars", 75);
    players.insert("Jane Johnson", 90);
    players.insert("Pete Poppins", 170);
    
    let key = "Jane Johnson";
	
    match players.get_mut(key) {
        Some(score) => *score = 155,
        None => println!("Player {} not in the players", key),
    }
    let key = "Mike Mars";
    match players.get_mut(key) {
        Some(score) => *score = 110,
        None => println!("Player {} not in the players", key),
    }

    match players.get("Jane Johnson") {
        Some(&score) => println!("{}'s score: {}", "Jane Johnson", score),
        None => println!("Player {} not in the players", "Jane Johnson")
    }
}