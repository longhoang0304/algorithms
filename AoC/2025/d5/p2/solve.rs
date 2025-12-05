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

fn count_fresh_ids(ranges: &[Range]) -> u64 {
    if ranges.is_empty() {
        return 0;
    }

    let mut result = 0;

    for range in ranges {
        result += range.end - range.start + 1;
    }

    result
}

fn main() {
    let mut buffer = String::new();
    let mut ranges: Vec<Range> = Vec::new();

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

    println!("{}", count_fresh_ids(&merged_ranges));
}
