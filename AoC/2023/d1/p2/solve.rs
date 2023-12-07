use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct TrieNode {
    value: char,
    is_word: bool,
    children: HashMap<char, Vec<Box<TrieNode>>>,
}

impl TrieNode {
    fn new(value: char, is_word: bool) -> Self {
        TrieNode {
            value,
            is_word,
            children: HashMap::new(),
        }
    }

    fn get_mut(&mut self, c: char) -> Option<&mut Box<TrieNode>> {
        let children = self.children.get_mut(&c);

        if children.is_none() {
            return None;
        }

        for child in children.unwrap() {
            if child.value == c {
                return Some(child);
            }
        }

        None
    }

    fn get(&self, c: char) -> Option<&Box<TrieNode>> {
        let children = self.children.get(&c);

        if children.is_none() {
            return None;
        }

        for child in children.unwrap() {
            if child.value == c {
                return Some(child);
            }
        }

        None
    }

    fn add_child(&mut self, child: TrieNode) {
        let key = child.value;
        let boxed_child = Box::new(child);
        let children = &mut self.children.entry(key).or_insert(vec![]);

        for existed_child in children.iter() {
            if existed_child.value == key {
                return;
            }
        }

        children.push(boxed_child);
    }
}

#[derive(Debug)]
struct Trie {
    head: Box<TrieNode>,
}

impl Trie {
    fn new() -> Self {
        let node = TrieNode::new('^', false);
        let boxed_node = Box::new(node);

        return Trie { head: boxed_node };
    }

    fn add(&mut self, word: &str) {
        let word_len = word.len();
        let mut head = &mut self.head;

        for (i, c) in word.chars().enumerate() {
            if head.get_mut(c).is_none() {
                head.add_child(TrieNode::new(c, i + 1 == word_len))
            }
            head = head.get_mut(c).unwrap();
        }
    }

    fn contains(&self, word: &str) -> bool {
        let mut head = &self.head;
        for c in word.chars() {
            if head.get(c).is_none() {
                return false;
            }
            head = head.get(c).unwrap();
        }

        head.is_word
    }
}

// ============================

fn build_trie() -> (Trie, Trie) {
    let mut trie_for = Trie::new();
    let mut trie_bak = Trie::new();

    trie_for.add("one");
    trie_for.add("two");
    trie_for.add("three");
    trie_for.add("four");
    trie_for.add("five");
    trie_for.add("six");
    trie_for.add("seven");
    trie_for.add("eight");
    trie_for.add("nine");
    trie_for.add("zero");

    trie_bak.add("eno");
    trie_bak.add("owt");
    trie_bak.add("eerht");
    trie_bak.add("ruof");
    trie_bak.add("evif");
    trie_bak.add("xis");
    trie_bak.add("neves");
    trie_bak.add("thgie");
    trie_bak.add("enin");
    trie_bak.add("orez");

    (trie_for, trie_bak)
}

// ============================

fn str_to_num(num: &String) -> u32 {
    match num.as_str() {
        "one" | "eno" => 1,
        "two" | "owt" => 2,
        "three" | "eerht" => 3,
        "four" | "ruof" => 4,
        "five" | "evif" => 5,
        "six" | "xis" => 6,
        "seven" | "neves" => 7,
        "eight" | "thgie" => 8,
        "nine" | "enin" => 9,
        "zero" | "orez" => 0,
        _ => 0,
    }
}

fn detect_digit(found: &mut bool, digit: char) -> u32 {
    if *found || !digit.is_digit(10) {
        return 0;
    }

    *found = true;
    digit.to_digit(10).unwrap()
}

fn detect_word_digit(found: &mut bool, digit: char, number: &mut String, node: &Option<&Box<TrieNode>>) -> u32 {
    if *found || node.is_none() {
        return 0;
    }


    number.push(digit);
    if !node.unwrap().is_word {
        return 0;
    }

    *found = true;
    return str_to_num(number);
}

fn find_calibration_value(trie_for: &Trie, trie_bak: &Trie, s: &str) -> u32 {
    let bytes = s.as_bytes();
    let s_len = s.len();
    let mut start_idx = 0;
    let mut end_idx = bytes.len() - 1;

    let mut result = 0;
    let mut found_start = false;
    let mut found_end = false;

    let mut node_start = &trie_for.head;
    let mut node_end = &trie_bak.head;

    let mut start_word = String::new();
    let mut end_word = String::new();

    while start_idx < s_len && !(found_start && found_end) {
        let start = bytes[start_idx] as char;
        let end = bytes[end_idx] as char;
        let n_start = node_start.get(start);
        let n_end = node_end.get(end);

        result += detect_digit(&mut found_start, start) * 10;
        result += detect_word_digit(&mut found_start, start, &mut start_word, &n_start) * 10;

        result += detect_digit(&mut found_end, end);
        result += detect_word_digit(&mut found_end, end, &mut end_word, &n_end);

        if !found_start && n_start.is_none() {
            start_idx -= start_word.len();
            node_start = &trie_for.head;
            start_word.clear();
        } else if !found_start {
            node_start = n_start.unwrap();
        }

        if !found_end && n_end.is_none() {
            end_idx += end_word.len();
            node_end = &trie_bak.head;
            end_word.clear();
        } else if !found_end {
            node_end = n_end.unwrap();
        }

        start_idx += !found_start as usize;
        end_idx -= !found_end as usize;
    }

    result
}

fn main() {
    let mut buffer = String::new();
    let mut result = 0;
    let (trie_for, trie_bak) = build_trie();

    loop {
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
        if buffer == "\n" || buffer.len() == 0 {
            break;
        }

        let b = buffer.clone();
        let striped_buffer = b.trim();

        buffer.clear();
        result += find_calibration_value(&trie_for, &trie_bak, striped_buffer);
    }
    println!("{}", result);
}
