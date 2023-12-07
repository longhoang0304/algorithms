use std::io;
use std::collections::hash_set::HashSet;
use std::collections::hash_map::HashMap;

fn get_point_from_card(game: &str, card_idx: u32, win_records: &mut HashMap<u32, u32>) {
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
    let base = *win_records.get(&card_idx).unwrap();

    for number in numbers {
        if !winning_numbers.contains(&number) { continue }
        winning += 1;
        win_records.entry(card_idx + winning).and_modify(|r| *r += base ).or_insert(base);
    }
}


fn main() {
    let mut buffer = String::new();
    let mut result = 0;
    let mut win_records: HashMap<u32, u32> = HashMap::new();
    let mut idx: u32 = 0;

    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer.is_empty() {
            break;
        }

        idx += 1;
        let card_info: Vec<&str> = buffer.split(":").map(|data| data.trim()).collect();
        
        win_records.entry(idx).and_modify(|r| *r += 1).or_insert(1);
        get_point_from_card(card_info[1], idx, &mut win_records);
        result += win_records[&idx];
        buffer.clear();
    }
    println!("{}", result);
}