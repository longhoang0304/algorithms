use std::io::{self, BufRead};

fn perform_op(op: &str, a: i64, b: i64) -> i64 {
    match op {
        "*" => a * b,
        "+" => a + b,
        _ => 0
    }
}

fn main() {
    let stdin = io::stdin();
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut ops: Vec<String> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            break;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        if tokens[0] == "*" || tokens[0] == "+" {
            ops = tokens.into_iter().map(String::from).collect();
        } else {
            numbers.push(
                tokens.into_iter()
                    .map(|s| s.parse().expect("Invalid number"))
                    .collect()
            );
        }
    }

    let result: i64 = ops.iter().enumerate().map(|(i, op)| {
        let init = if op == "*" { 1 } else { 0 };
        numbers.iter().fold(init, |acc, row| perform_op(op, acc, row[i]))
    }).sum();

    println!("{result}");
}