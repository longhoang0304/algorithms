use std::io;

type Map = Vec<Vec<char>>;

fn count_col_load(map: &Map, col: usize) -> i64 {
    let max_len = map.len() as i64;
    let mut max_point = map.len() as i64;
    let mut score = 0;
    
    for (idx, r) in map.iter().enumerate() {
        if r[col] == '#' {
            max_point = max_len - (idx as i64 + 1);
            continue;
        }
        if r[col] != 'O' { continue; }
        score += max_point;
        max_point -= 1;
    }

    score as i64
}

fn count_load(map: Map) -> i64 {
    let max_col = map[0].len();
    let mut result = 0;

    for idx in 0..max_col {
        result += count_col_load(&map, idx);
    }

    result
}

fn main() {
    let mut buffer = String::new();
    let mut map: Map = Vec::new();

    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer == "\r\n" || buffer.is_empty() {
            break;
        }
        
        let trimmed = buffer.trim();
        let chars: Vec<char> = trimmed.chars().collect();

        map.push(chars);
        buffer.clear();
    }
    println!("{}", count_load(map));
}
