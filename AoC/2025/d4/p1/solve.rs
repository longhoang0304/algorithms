use std::io;
const X_MOVE: [i32; 8] = [-1, -1, -1,  0, 0,  1, 1, 1];
const Y_MOVE: [i32; 8] = [-1,  0,  1, -1, 1, -1, 0, 1];

fn count_nearby_paper_rolls(grid: &Vec<Vec<u32>>, x: i32, y: i32) -> u32 {
    let mut accessible_paper_roll = 0;
    let lx = grid.len() as i32;
    let ly = grid[0].len() as i32;

    for mi in 0..8 {
        let xx = x + X_MOVE[mi];
        let yy = y + Y_MOVE[mi];

        if xx < 0 || xx >= lx as i32 || yy < 0 || yy >= ly {
            continue;
        }

        accessible_paper_roll += grid[xx as usize][yy as usize]
    }

    accessible_paper_roll
}

fn count_paper_rolls(grid: &Vec<Vec<u32>>) -> u32 {
    let mut accessible_paper_roll = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == 0 {
                continue;
            }

            if count_nearby_paper_rolls(grid, x as i32, y as i32) > 3 {
                continue;
            }
            accessible_paper_roll += 1;
        }
    }

    accessible_paper_roll
}

fn parse_row(row_str: &str) -> Vec<u32> {
    let mut row = Vec::new();

    for c in row_str.chars() {
       if c == '.' {
           row.push(0);
       } else {
           row.push(1);
       }
    }

    row
}

fn main() {
    let mut buffer = String::new();
    let mut grid: Vec<Vec<u32>> = Vec::new();

    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer == "\r\n" || buffer.len() == 0 {
            break;
        }

        let b = buffer.clone();
        let trimmed = b.trim();

        buffer.clear();

        grid.push(parse_row(trimmed));
    }

    println!("{}", count_paper_rolls(&grid));
}
