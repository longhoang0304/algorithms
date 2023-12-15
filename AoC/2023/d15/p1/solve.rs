use std::io;

fn hash(word: &str) -> u64 {
    let mut hash = 0;

    for c in word.bytes() {
        hash += c as u64;
        hash *= 17;
        hash %= 256;
    }

    hash
}

fn sum_hash(sen: &str) -> u64 {
    let words: Vec<&str> = sen.split(",").collect();
    let mut result = 0;
    
    for word in words {
        result += hash(word);
    }

    result
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).expect("Failed to read line");

    let trimmed = buffer.trim();
    println!("{}", sum_hash(trimmed));
}
