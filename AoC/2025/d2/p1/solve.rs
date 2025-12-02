use std::io;
use std::cmp::max;

fn count_invalid_ids(start: u64, end: u64) -> u64 {
    let s_start = start.to_string();

    let mid_idx = max(s_start.len() / 2, 1);
    let mut mid: u64 = s_start[..mid_idx].parse().expect("Not a number!");

    let mut invalid_ids_sum: u64 = 0;

    loop {
        let s_mid = mid.to_string();
        let invalid_id: u64 = s_mid.repeat(2).parse().expect("Not a number!");

        if invalid_id >= start && invalid_id <= end {
            invalid_ids_sum += invalid_id;
        }

        if invalid_id > end {
            break;
        }

        mid += 1;
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
