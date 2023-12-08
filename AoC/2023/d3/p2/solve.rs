use std::io;

type Map = Vec<Vec<char>>;

fn find_num(row: &mut Vec<char>, c: i32) -> u32 {
    let max_col = row.len() as i32;
    let mut num: Vec<u32> = Vec::new();
    let mut start = c - 1;
    let mut end = c + 1;

    num.push(row[c as usize].to_digit(10).unwrap());

    loop {
        if start >= 0 {
            let y = start as usize;
            let chr = row[y];
            if chr.is_digit(10) {
                num.insert(0, chr.to_digit(10).unwrap());
                start -= 1;
                row[y] = '.';
            }
        }
        if end < max_col {
            let y = end as usize;
            let chr = row[y];
            if chr.is_digit(10) {
                num.push(chr.to_digit(10).unwrap());
                end += 1;
                row[y] = '.';
            }
        }
        
        if (start < 0 || !row[start as usize].is_digit(10))
        && (end >= max_col || !row[end as usize].is_digit(10)) {
            break
        }
    }
    num.iter().fold(0, |acc, x| acc * 10 + x)
}

fn walk_num(map: &mut Map, r: i32, c: i32) -> u32 {
    let mut result = 1;
    let mut num_count = 0;
    let max_row = map.len() as i32;
    let max_col = map[0].len() as i32;
    const X: [i32; 3] = [-1, 0, 1];
    const Y: [i32; 3] = [-1, 0, 1];
    
    for x in X {
        for y in Y {
            let i = r + x;
            let j = c + y;
            
            if i < 0 || i >= max_row { continue }
            if j < 0 || j >= max_col { continue }
            if i == 0 && j == 0 { continue }
            if !map[i as usize][j as usize].is_digit(10) { continue }
            result *= find_num(&mut map[i as usize], j);
            num_count += 1;
        }
    }

    if num_count < 2 {
        return 0;
    }
    
    result
}

fn main() {
    let mut map: Map = Vec::new();
    let mut buffer = String::new();
    let mut result = 0;
    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer.is_empty() {
            break;
        }

        let buf = buffer.trim().chars().collect();
        map.push(buf);
        buffer.clear();
    }
    
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            let col = map[x][y];
            if col != '*' { continue }
            result += walk_num(&mut map, x as i32, y as i32);
        }
    }
    
    println!("{}", result);
}