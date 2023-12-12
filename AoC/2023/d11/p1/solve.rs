use std::io;
use std::collections::HashMap;

fn expand_galaxies(graph: &mut Vec<(u32, u32)>, empty_rows: &Vec<u32>, empty_cols: &Vec<u32>) {
    for row in empty_rows.iter().rev() {
        for galaxy in graph.iter_mut() {
            if galaxy.0 > *row {
                galaxy.0 += 1;
            }
        }
    }

    for col in empty_cols.iter().rev() {
        for galaxy in graph.iter_mut() {
            if galaxy.1 > *col {
                galaxy.1 += 1;
            }
        }
    }
}

fn find_shortest_path(node: &(u32, u32), graph: &[(u32, u32)]) -> u32 {
    let mut ret = 0;
    for n in graph {
        let c = (node.0 as i32 - n.0 as i32).abs() as u32;
        let r = (node.1 as i32 - n.1 as i32).abs() as u32;
        
        ret += c + r;
    }

    ret
}

fn sum_shortest_paths(graph: &Vec<(u32, u32)>) -> u32 {
    let mut result = 0;
    for (idx, node) in graph.iter().enumerate() {
        result += find_shortest_path(&node, &graph[idx + 1..]);
    }

    result
}

fn main() {
    let mut buffer = String::new();
    let mut galaxies: Vec<(u32, u32)> = Vec::new();
    let mut empty_rows: Vec<u32> = Vec::new();
    let mut empty_cols: Vec<u32> = Vec::new();
    let mut empty_cols_map: HashMap<usize, bool> = HashMap::new();
    let mut row = 0;
    let mut col_size = 0;
    
    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer == "\r\n" || buffer.is_empty() {
            break;
        }
        let trimmed_buffer = buffer.trim();
        
        if col_size == 0 {
            col_size = trimmed_buffer.len();
        }

        let mut empty_row = true;
        for (idx, chr) in trimmed_buffer.chars().enumerate() {
            if chr == '.' {
                continue;
            }

            empty_cols_map.entry(idx).or_insert(false);
            empty_row = false;
            galaxies.push((row, idx as u32));
        }

        if empty_row {
            empty_rows.push(row);
        }
        
        row += 1;
        buffer.clear();
    }

    for idx in 0..col_size {
        if *empty_cols_map.get(&idx).unwrap_or(&true) {
            empty_cols.push(idx as u32);
        }
    }

    expand_galaxies(&mut galaxies, &empty_rows, &empty_cols);
    let result = sum_shortest_paths(&galaxies);
    println!("{result}");
}
