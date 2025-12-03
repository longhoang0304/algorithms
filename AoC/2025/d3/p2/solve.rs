use std::io;

fn find_max_joltage(bat: &str, cap: usize) -> u64 {
    let battery: Vec<u32> = bat.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let n = battery.len();
    let mut q = Vec::with_capacity(cap);

    for (i, b) in battery.iter().enumerate() {
        while !q.is_empty() {
            let last = q.last().unwrap();
            if *last >= b {
                break;
            }

            if (q.len() + n - i) <= cap {
                break;
            }

            q.pop();
        }

        if q.len() >= cap {
            continue;
        }

        q.push(b);
    }

    let mut max = 0;

    for v in q {
        max = max * 10 + *v as u64;
    }

    max
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

        buffer.clear();

        result += find_max_joltage(trimmed, 12);
    }

    println!("{}", result);
}
