use std::io;

fn rotate(data: &str, current: i32) -> i32 {
    let rotate_direction = data.chars().nth(0).expect("Invalid length");
    let rotate_value: i32 = data[1..].parse().expect("Not a number!");
    if rotate_direction == 'L' {
        return ((current - rotate_value) % 100 + 100) % 100;
    }

    if rotate_direction == 'R' {
        return (current + rotate_value) % 100;
    }
    current
}

fn main() {
    let mut buffer = String::new();
    let mut result = 0;
    let mut current = 50;
    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer == "\r\n" || buffer.len() == 0 {
            break;
        }
        let b = buffer.clone();
        let striped_buffer = b.trim();

        buffer.clear();
        current = rotate(striped_buffer, current);
        if current == 0 {
            result += 1;
        }
    }

    println!("{}", result);
}
