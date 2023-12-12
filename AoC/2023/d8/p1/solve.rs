use std::io;
use std::collections::HashMap;

type Pair = (String, String);
type Key = String;
type Network = HashMap<Key, Pair>;

fn parse_network(network: &mut Network, net: &str) {
    let net_tokens: Vec<&str> = net.split(" = ").collect();
    let key = net_tokens[0];
    let pair = net_tokens[1];
    let pair_tokens: Vec<&str> = pair.split(", ").collect();
    let left = pair_tokens[0].strip_prefix("(").unwrap();
    let right = pair_tokens[1].strip_suffix(")").unwrap();
    
    network.insert(String::from(key), (String::from(left), String::from(right)));
}

fn walk(network: &Network, path: String) -> u64 {
    let path_size = path.len();
    let path_chars: Vec<char> = path.chars().collect();
    let mut idx: usize = 0;
    let mut step_count = 0;
    let mut node = &String::from("AAA");
    let end = &String::from("ZZZ");
    
    
    while node != end {
        let step = path_chars[idx];
        let net = &network[node];

        if step == 'L' {
            node = &net.0;
        } else {
            node = &net.1;
        }

        step_count += 1;
        idx = (idx + 1) % path_size as usize;
    }

    step_count
}

fn main() {
    let mut buffer = String::new();
    let mut network: Network = HashMap::new();
    let path: String;
    

    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    path = String::from(buffer.trim());
    buffer.clear();

    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    buffer.clear();

    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer == "\r\n" || buffer.is_empty() {
            break;
        }
        
        let trimmed = buffer.trim();
        parse_network(&mut network, trimmed);
        
        buffer.clear();
    }
    
    let result = walk(&network, path);
    println!("{result}");
}
