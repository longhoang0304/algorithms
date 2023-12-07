use std::io;

fn find_calibration_value(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let mut start_idx = 0;
    let mut end_idx = bytes.len() - 1;

    let mut result = 0;
    let mut found_start = false;
    let mut found_end = false;

    while start_idx <= end_idx && !(found_start && found_end) {
        let start = bytes[start_idx] as char;
        let end = bytes[end_idx] as char;

        if !found_start && start.is_digit(10) {
            let v = start.to_digit(10).unwrap();
            result += v * 10;
            found_start = true;
        }

        if !found_end && end.is_digit(10) {
            let v = end.to_digit(10).unwrap();
            result += v;
            found_end = true;
        }

        start_idx += !found_start as usize;
        end_idx -= !found_end as usize;
    }
    result
}


fn main() {
    let mut buffer = String::new();
    let mut result = 0;
    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer.len() == 0 {
            break;
        }
        let b = buffer.clone();
        let striped_buffer = b.trim();

        buffer.clear();
        result += find_calibration_value(striped_buffer);
    }
    println!("{}", result);
}
