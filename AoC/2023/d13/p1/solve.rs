use std::io;
use std::cmp::min;

type Map = Vec<Vec<char>>;

fn is_vertica_reflection(map: &Map, col1: usize, col2: usize) -> bool {
    for r in map.iter() {
        if r[col1] != r[col2] { return false; }
    }
    true
}

fn check_vertical_reflection(map: &Map, col1: usize, col2: usize) -> bool {
    let max_col = map[0].len();
    let steps = min(col1, max_col - col2 - 1);

    for idx in 1..=steps {
        let c1 = col1 - idx;
        let c2 = col2 + idx;
        if !is_vertica_reflection(map, c1, c2) { return false; }
    }
    true
}

fn get_vertical_reflection(map: &Map) -> i64 {
    let max_col = map[0].len();
    for idx in 1..max_col {
        if !is_vertica_reflection(map, idx - 1, idx) { continue; }
        if check_vertical_reflection(map, idx - 1, idx) { return idx as i64; }
    }
    0
}

// ================

fn is_hortizontal_reflection(map: &Map, row1: usize, row2: usize) -> bool {
    for (idx, v) in map[row1].iter().enumerate() {
        if *v != map[row2][idx] { return false; }
    }
    true
}

fn check_horizontal_reflection(map: &Map, row1: usize, row2: usize) -> bool {
    let max_row = map.len();
    let steps = min(row1, max_row - row2 - 1);

    for idx in 1..=steps {
        let r1 = row1 - idx;
        let r2 = row2 + idx;
        if !is_hortizontal_reflection(map, r1, r2) { return false; }
    }
    true
}

fn get_horizontal_reflection(map: &Map) -> i64 {
    let max_row = map.len();
    for idx in 1..max_row {
        if !is_hortizontal_reflection(map, idx - 1, idx) { continue; }
        if check_horizontal_reflection(map, idx - 1, idx) { return idx as i64; }
    }
    0
}

fn count_reflections(map: &Map) -> i64 {
    let h = get_horizontal_reflection(map);
    let v = get_vertical_reflection(map);

    v + h * 100
}

fn main() {
    let mut buffer = String::new();
    let mut map: Map = Vec::new();
    let mut result = 0;

    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer.is_empty() {
            result += count_reflections(&map);
            break;
        }

        if buffer == "\n" || buffer == "\r\n" {
            result += count_reflections(&map);
            map.clear();
            buffer.clear();
            continue;
        }
        
        let trimmed = buffer.trim();
        let chars: Vec<char> = trimmed.chars().collect();

        map.push(chars);
        buffer.clear();
    }
    println!("{result}");
}
