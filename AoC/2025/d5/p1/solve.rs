use std::io;

#[derive(Debug, Copy, Clone)]
struct Range {
    start: u64,
    end: u64,
}


fn parse_range(range_str: &str) -> Range {
    let range_data: Vec<u64> = range_str.split("-").map(|data| data.trim().parse::<u64>().unwrap()).collect();

    Range {
        start: range_data[0],
        end: range_data[1],
    }
}

fn merge_ranges(ranges: &[Range]) -> Vec<Range> {
    let mut merged_ranges = Vec::new();

    for range in ranges {
        if merged_ranges.is_empty() {
            merged_ranges.push(*range);
            continue;
        }

        let latest_range = merged_ranges.last_mut().unwrap();

        // Ranges don't overlap and aren't adjacent: [1, 3], [5, 7]
        if latest_range.end + 1 < range.start {
            merged_ranges.push(*range);
            continue;
        }

        // Ranges overlap or are adjacent: extend the end if needed
        if range.end > latest_range.end {
            latest_range.end = range.end;
        }

    }

    merged_ranges
}

fn find_number_in_range(number: u64, ranges: &[Range]) -> bool {
    if ranges.is_empty() {
        return false;
    }

    let mut low = 0;
    let mut hig = ranges.len();

    while low < hig {
        let mid = low + (hig - low) / 2;
        let range = &ranges[mid];
        if range.start <= number && range.end >= number {
            return true;
        }

        if range.start > number {
            hig = mid;
            continue;
        }

        if range.end < number {
            low = mid + 1;
            continue;
        }
    }

    false
}

fn main() {
    let mut buffer = String::new();
    let mut ranges: Vec<Range> = Vec::new();
    let mut result = 0;

    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer == "\r\n" || buffer.len() == 0 {
            break;
        }

        let b = buffer.clone();
        let trimmed = b.trim();

        buffer.clear();

        ranges.push(parse_range(trimmed));
    }

    ranges.sort_by(|a, b| {
        if a.start.cmp(&b.start) == std::cmp::Ordering::Equal {
            a.end.cmp(&b.end)
        } else {
            a.start.cmp(&b.start)
        }
    });

    let merged_ranges = merge_ranges(&ranges);

    buffer.clear();

    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer == "\r\n" || buffer.len() == 0 {
            break;
        }

        let b = buffer.clone();
        let trimmed = b.trim();

        buffer.clear();

        result += find_number_in_range(trimmed.parse::<u64>().unwrap(), &merged_ranges) as u64;
    }

    println!("{}", result);
}
