use std::io::{self, BufRead};

fn perform_op(op: &str, a: i64, b: i64) -> i64 {
    match op {
        "*" => a * b,
        "+" => a + b,
        _ => 0
    }
}

fn zip_strings(strings: &[String]) -> Vec<String> {
    let mut iters: Vec<_> = strings.iter().map(|s| s.chars()).collect();

    std::iter::from_fn(|| {
        iters.iter_mut()
            .map(|it| it.next())
            .collect::<Option<String>>()
    }).collect()
}

fn parse_number_groups(raw_numbers: Vec<String>) -> Vec<Vec<i64>> {
    raw_numbers
        .split(|s| s.is_empty())
        .map(|group| {
            group.iter()
                .map(|s| s.parse().expect("Invalid number"))
                .collect()
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines()
        .map(|l| l.expect("Failed to read line"))
        .take_while(|l| !l.is_empty())
        .collect();

    let ops: Vec<&str> = lines.last()
        .expect("No input")
        .split_whitespace()
        .collect();

    let raw_numbers: Vec<String> = zip_strings(&lines[..lines.len() - 1])
        .into_iter()
        .map(|x| x.trim().to_string())
        .collect();

    let nums = parse_number_groups(raw_numbers);

    let result: i64 = ops.iter()
        .enumerate()
        .map(|(i, op)| {
            let init = if *op == "*" { 1 } else { 0 };
            nums[i].iter().fold(init, |acc, &num| perform_op(op, acc, num))
        })
        .sum();

    println!("{result}");
}