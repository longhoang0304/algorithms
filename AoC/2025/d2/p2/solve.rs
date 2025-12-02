use std::io;
use std::cmp::min;
use std::collections::HashSet;

fn count_invalid_ids(start: u64, end: u64) -> u64 {
    let s_start = start.to_string();
    let s_end = end.to_string();
    let l_start = s_start.len();
    let l_end = s_end.len();

    let mid_idx = l_end / 2 + l_end % 2;
    let mid: u64 = s_end[..mid_idx].parse().expect("Not a number!");

    let mut invalid_ids_sum: u64 = 0;
    let mut saved: HashSet<u64> = HashSet::new();

    for i in 1..=mid {
        let s_i = i.to_string();
        let l_i = s_i.len();
        for l in l_start..=l_end {
            let repeat = l / l_i;
            if repeat == 1 {
                continue;
            }

            let invalid_id: u64 = s_i.repeat(l / l_i + min(1, l % l_i)).parse().expect("Not a number!");
            if invalid_id >= start && invalid_id <= end && !saved.contains(&invalid_id) {
                saved.insert(invalid_id);
                invalid_ids_sum += invalid_id;
            }

            if invalid_id > end {
                break
            }
        }
    }

    invalid_ids_sum
}

fn main() {
    let mut buffer = String::new();
    let mut result = 0;
    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer == "\r\n" || buffer.len() == 0 {
            break;
        }

        let b = buffer.clone();
        let trimmed = b.trim();
        let ranges: Vec<&str> = trimmed.split(",").collect();

        buffer.clear();

        for range in ranges.iter() {
            let token: Vec<&str> = range.split("-").collect();
            let start: u64 = token[0].parse().expect("Not a number!");
            let end: u64 = token[1].parse().expect("Not a number!");
            result += count_invalid_ids(start, end);
        }
    }

    println!("{}", result);
}
