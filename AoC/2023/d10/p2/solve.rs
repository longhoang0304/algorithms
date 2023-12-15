use std::io;
use std::collections::{HashSet};
use std::cmp::{max};

type Map = Vec<Vec<char>>;
type MapScore = Vec<Vec<i64>>;
type Cord = (i64, i64);

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn print_map_beauty(map: &Map) {
    for row in map.iter() {
        for c in row.iter() {
            let print_char = match *c {
                'S' => '┼',
                '.' => '.',
                '|' => '│',
                '-' => '─',
                'L' => '└',
                'J' => '┘',
                '7' => '┐',
                'F' => '┌',
                _ => '.',
            };
            print!("{print_char}");
        }
        println!("");
    }
}

fn get_next_pipe(pipe: char, direction: &Direction) -> Option<HashSet<char>> {
    if pipe == '.' { return None; }

    match direction {
        Direction::Up => Some(HashSet::from(['|','7','F','S'])),
        Direction::Down => Some(HashSet::from(['|','L','J','S'])),
        Direction::Left => Some(HashSet::from(['-','L','F','S'])),
        Direction::Right => Some(HashSet::from(['-','J','7','S'])),
    }
}

fn get_pipe_direction(pipe: char) -> Vec<Direction> {
    match pipe {
        'S' => vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right],
        'L' => vec![Direction::Up, Direction::Right],
        'J' => vec![Direction::Up, Direction::Left],
        '7' => vec![Direction::Down, Direction::Left],
        'F' => vec![Direction::Down, Direction::Right],
        '|' => vec![Direction::Down, Direction::Up],
        '-' => vec![Direction::Left, Direction::Right],
        _ => vec![],
    }
}

fn translate_direction(cord: &Cord, direction: &Direction, max_cord: &Cord) -> Cord {
    let (x, y) = cord;

    match direction {
        Direction::Up       if *x - 1 >= 0           => (*x - 1, *y),
        Direction::Down     if *x + 1 < max_cord.0   => (*x + 1, *y),
        Direction::Left     if *y - 1 >= 0           => (*x, *y - 1),
        Direction::Right    if *y + 1 < max_cord.1   => (*x, *y + 1),
        _ => (*x, *y),
    }
}

fn find_main_loop(map: &Map, start: Cord) -> MapScore  {
    let max_cord = (map.len() as i64, map[0].len() as i64);
    let mut st = vec![start];
    let mut map_score: MapScore = vec![vec![0; max_cord.1 as usize]; max_cord.0 as usize];
    let mut visited: HashSet<Cord> = HashSet::new();

    while !st.is_empty() {
        let cord = st.pop().unwrap();
        let x = cord.0 as usize;
        let y = cord.1 as usize;
        let pipe = map[x][y];
        let current_score = map_score[x][y];

        if pipe != 'S' {
            visited.insert(cord);
        }

        for direction in get_pipe_direction(pipe) {
            let new_cord = translate_direction(&cord, &direction, &max_cord);
            if visited.contains(&new_cord) { continue; }

            let nx = new_cord.0 as usize;
            let ny = new_cord.1 as usize;
            let next_pipe = map[nx][ny];
            let score = &mut map_score[nx][ny];
            
            if let Some(next_pipes) = get_next_pipe(pipe, &direction) {
                if !next_pipes.contains(&next_pipe) { continue }
                if next_pipe != 'S' { st.push(new_cord); }
                *score = max(*score, current_score + 1);
            }
        }
    }

    map_score
}

fn shoeslack_algo(start: Cord, map: &MapScore) -> i64  {
    let mut m: Vec<Cord> = vec![];
    let mut st: Vec<Cord> = vec![start];
    let mut result = 0;
    let max_cord = (map.len() as i64, map[0].len() as i64);
    let directions = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    while !st.is_empty() {
        let cord = st.pop().unwrap();
        let x = cord.0 as usize;
        let y = cord.1 as usize;
        let curr_score = map[x][y];
        m.push(cord);

        for direction in &directions {
            let new_cord = translate_direction(&cord, direction, &max_cord);
            let nx = new_cord.0 as usize;
            let ny = new_cord.1 as usize;

            if map[nx][ny] != curr_score - 1 { continue; }
            st.push(new_cord);
            break;
        }
    }

    let m_max = map[start.0 as usize][start.1 as usize] as usize;

    for (idx, n) in m.iter().enumerate() {
        let next = m[(idx + 1) % m_max];
        result += n.0 * next.1 - n.1 * next.0;
    }

    result.abs() / 2 - m_max as i64 / 2 + 1
}

fn count_enclosed_animal(map: &Map, start: Cord) -> i64 {
    let map_score = find_main_loop(map, start);

    shoeslack_algo(start, &map_score)
}

fn main() {
    let mut buffer = String::new();
    let mut map: Map = Vec::new();
    let mut row = 0;
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer == "\r\n" || buffer.is_empty() {
            break;
        }
        
        let trimmed = buffer.trim();
        let chars: Vec<char> = trimmed.chars().collect();

        for (yi, c) in chars.iter().enumerate() {
            if *c == 'S' {
                y = yi as i64;
                x = row;
                break;
            }
        }

        map.push(chars);
        buffer.clear();
        row += 1;
    }

    print_map_beauty(&map);

    let result = count_enclosed_animal(&map, (x, y));
    println!("{result}");
}
