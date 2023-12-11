use std::io;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::cmp::{Ordering, Ord, Eq, PartialEq, min};

static CARD_ORDER: Lazy<HashMap<char, u8>> = Lazy::new(|| HashMap::from([
    ('A', 14),
    ('K', 13),
    ('Q', 12),
    ('J', 11),
    ('T', 10),
    ('9', 9),
    ('8', 8),
    ('7', 7),
    ('6', 6),
    ('5', 5),
    ('4', 4),
    ('3', 3),
    ('2', 2),
]));

#[derive(Debug)]
struct Hand {
    cards: String,
    cards_type: u64,
    score: u64,
}


impl Hand {
    fn compare_card(&self, other: &Self) -> Ordering {
        let mut a = self.cards.chars();
        let mut b = other.cards.chars();
        let la = self.cards.len();
        let lb = other.cards.len();
        let size = min(la, lb);

        for _ in 0..size {
            let ca = a.next().unwrap();
            let cb = b.next().unwrap();
            if CARD_ORDER[&ca] > CARD_ORDER[&cb] { return Ordering::Greater }
            if CARD_ORDER[&ca] < CARD_ORDER[&cb] { return Ordering::Less }
        }
        
        if la == size { return Ordering::Greater }
        if lb == size { return Ordering::Less }

        Ordering::Equal
    }
}


impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cards_type < other.cards_type {
            return Ordering::Less;
        }
        if self.cards_type > other.cards_type {
            return Ordering::Greater;
        }

        let card_cmp = self.compare_card(other);
        if card_cmp != Ordering::Equal {
            return card_cmp;
        }
        
        if self.score < other.score {
            return Ordering::Less;
        }
        if self.score > other.score {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {
}


fn get_card_type(cards: &str) -> u64 {
    let mut counter: HashMap<char, u32> = HashMap::new();
    
    for c in cards.chars() {
        counter.entry(c).and_modify(|x| *x += 1).or_insert(1);
    }
    
    let counter_keys_size = counter.keys().len();
    
    if counter_keys_size == 5 {
        return 1;
    }
    
    if counter_keys_size == 1 {
        return 7;
    }
    
    for k in counter.keys() {
        if counter[k] == 4  {
            return 6;
        }

        if counter[k] == 3 && counter_keys_size == 2  {
            return 5;
        }
        
        if counter[k] == 2 && counter_keys_size == 2  {
            return 5;
        }
        
        if counter[k] == 3 && counter_keys_size == 3  {
            return 4;
        }
        
        if counter[k] == 2 && counter_keys_size == 3  {
            return 3;
        }
        
        if counter[k] == 2 && counter_keys_size == 3  {
            return 3;
        }
        
        if counter[k] == 2 && counter_keys_size == 4  {
            return 2;
        }
    }
    
    0
}

fn parse_hand(buffer: &mut String) -> Box<Hand> {
    let tokens: Vec<&str> = buffer.split(" ").collect();
    let cards_str = tokens[0];
    let score_str = tokens[1];
    let score = score_str.trim().parse::<u64>().unwrap();
    let cards = cards_str.trim();
    
    Box::new(Hand{
        cards_type: get_card_type(cards),
        cards: String::from(cards_str),
        score,
    })
    
}

fn main() {
    let mut buffer = String::new();
    let mut result = 0;
    let mut hands: Vec<Box<Hand>> = Vec::new();
    
    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer == "\r\n" || buffer.is_empty() {
            break;
        }
        
        let hand = parse_hand(&mut buffer);
        hands.push(hand);
        buffer.clear();
    }
    hands.sort();
    for (idx, hand) in hands.iter().enumerate() {
        result += (idx + 1) as u64 * hand.score;
    }
    println!("{}", result);
}
