use std::io;
use std::collections::hash_set::HashSet;

fn get_point_from_card(game: &str) -> u32 {
    let ticket_parts : Vec<&str> = game.split("|").map(|p| p.trim()).collect();
    let winning_numbers: HashSet<u32> = ticket_parts[0]
        .split(" ")
        .map(|n| n.trim())
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let numbers: Vec<u32> = ticket_parts[1]
        .split(" ")
        .map(|n| n.trim())
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let mut winning = 0;

    for number in numbers {
        if !winning_numbers.contains(&number) { continue };
        winning += 1;
    }

    if winning == 0 { return 0 }
    1 << (winning - 1)
}


fn main() {
    let mut buffer = String::new();
    let mut result = 0;
    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer.is_empty() {
            break;
        }

        let card_info: Vec<&str> = buffer.split(":").map(|data| data.trim()).collect();
        
        result += get_point_from_card(card_info[1]);
        buffer.clear();
        
    }
    println!("{}", result);
}