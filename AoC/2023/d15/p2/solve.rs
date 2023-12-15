use std::io;

type Lense = (String, u8);
type Boxes = Vec<Vec<Lense>>;

fn hash(word: &str) -> u64 {
    let mut hash = 0;

    for c in word.bytes() {
        hash += c as u64;
        hash *= 17;
        hash %= 256;
    }

    hash
}

fn remove_lense(word: String, boxes: &mut Boxes) {
    let box_idx = hash(word.as_str()) as usize;
    let lense_box = &mut boxes[box_idx];

    if lense_box.is_empty() { return; }
    for i in 0..lense_box.len() {
        if lense_box[i].0 == word {
            lense_box.remove(i);
            break;
        }
    }
}

fn replace_or_insert_lense(word: String, boxes: &mut Boxes) {
    let tokens: Vec<&str> = word.as_str().split("=").collect();
    let lense_name = tokens[0];
    let lense_focus_str = tokens[1];
    let lense_focus = lense_focus_str.parse::<u8>().unwrap();
    let box_idx = hash(lense_name) as usize;
    let lense_box = &mut boxes[box_idx];

    if lense_box.is_empty() {
        lense_box.push((String::from(lense_name), lense_focus));
        return;
    }

    for lense in lense_box.iter_mut() {
        if lense.0 != lense_name { continue; }
        lense.1 = lense_focus;
        return;
    }

    lense_box.push((String::from(lense_name), lense_focus));
}

fn manage_lenses(word: &str, boxes: &mut Boxes) {
    let mut wrapped = String::from(word);
    let chars = wrapped.chars();

    if chars.last().unwrap() == '-' {
        wrapped.pop();
        remove_lense(wrapped, boxes);
        return;
    }
    replace_or_insert_lense(wrapped, boxes);
}

fn calc_box_focus(box_idx: usize, lense_box: &Vec<Lense>) -> u64 {
    let mut focusing_score = 0;

    for (idx, lense) in lense_box.iter().enumerate() {
        focusing_score += (box_idx as u64 + 1) * (idx as u64 + 1) * (lense.1 as u64);
    }

    focusing_score
}

fn calc_focus_power(sen: &str) -> u64 {
    let mut boxes: Boxes = vec![];
    let mut result = 0;
    let words: Vec<&str> = sen.split(",").collect();

    for _ in 0..256 {
        boxes.push(Vec::new());
    }
    
    for word in words {
        manage_lenses(word, &mut boxes);
    }

    for (idx, lense_box) in boxes.iter().enumerate() {
        result += calc_box_focus(idx, &lense_box);
    }

    result
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).expect("Failed to read line");

    let trimmed = buffer.trim();
    println!("{}", calc_focus_power(trimmed));
}
