use std::io;
use std::collections::{HashMap, HashSet};
use std::cmp::max;

type NetNode = (String, String);
type Node<'a> = (&'a String, usize);
type Key = String;
type Network = HashMap<Key, NetNode>;
type Path<'a> = HashMap<usize, Vec<&'a String>>;
type Cycle<'a> = HashMap<usize, &'a String>;
type Visit<'a> = HashMap<usize, HashSet<Node<'a>>>;

fn parse_network(network: &mut Network, net: &str) {
    let net_tokens: Vec<&str> = net.split(" = ").collect();
    let key = net_tokens[0];
    let pair = net_tokens[1];
    let pair_tokens: Vec<&str> = pair.split(", ").collect();
    let left = pair_tokens[0].strip_prefix("(").unwrap();
    let right = pair_tokens[1].strip_suffix(")").unwrap();
    
    network.insert(String::from(key), (String::from(left), String::from(right)));
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut va = a;
    let mut vb = b;
    while va != vb {
        if va > vb { va -= vb }
        else { vb -= va }
    }
    va
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

// note: 
// use cycle detection
// for example
// first A takes 2 steps to the cycle, the cycle has 3 step
// secon A takes 3 steps to the cycle, the cycle has 4 step
// third A takes 10 steps and no cycle
// result will be
// 2 + 3 + 10 + 3 * 4
// write a function to detect cycle + no. steps to cycle
fn walk(network: &Network, path: String) -> (Cycle, Path) {
    let path_size = path.len();
    let path_chars: Vec<char> = path.chars().collect();

    let mut idx: usize = 0;
    let mut nodes: Vec<&String> = network.keys().filter(|k| k.chars().last().unwrap() == 'A').collect();
    let mut paths: Path = HashMap::new();
    let mut visited: Visit = HashMap::new();
    let mut cycles: Cycle = HashMap::new();

    let nodes_len = nodes.len();

    loop {
        let step = path_chars[idx];

        for (node_idx, node) in nodes.iter_mut().enumerate() {
            if cycles.contains_key(&node_idx) { continue } 

            let next_pair = &network[*node];
            let next: &String;

            if step == 'L' { next = &next_pair.0; }
            else { next = &next_pair.1; }

            let end_path = next_pair.0 == **node && next_pair.1 == **node;
            let n: Node = (next, idx);

            // found cycle or the end of the path without any cycle
            if end_path || visited.entry(node_idx).or_insert(HashSet::new()).contains(&n) {
                cycles.entry(node_idx).or_insert(next);
                continue;
            }
        
            visited.entry(node_idx).and_modify(|v| { v.insert(n); });
            paths.entry(node_idx).and_modify(|p| p.push(next)).or_insert(vec![next]);
            
            *node = next;
        }

        if cycles.keys().len() == nodes_len { break }
        idx = (idx + 1) % path_size as usize;
    }

    (cycles, paths)
}

fn calc_steps(network: &Network, path: String) -> u64 {
    let (cycles, paths) = walk(network, path);
    let mut cycle_counts: Vec<i64> = Vec::new();
    let mut path_counts: Vec<i64> = Vec::new();
    let mut result = 0;
    let mut max_path = 0;

    for (idx, node) in cycles.iter() {
        let path_len: i64 = paths[idx].len() as i64;
        let cidx: i64 = paths[idx].iter().position(|p| p == node).unwrap() as i64;

        cycle_counts.push(path_len - cidx);
        path_counts.push(cidx);
        max_path = max(max_path, cidx);
    }

    let mut meet_cycle = 1;
    
    for (idx, c) in cycle_counts.iter().enumerate() {
        meet_cycle = lcm(meet_cycle, *c);

        if max_path == path_counts[idx] { continue }
        if *c >= max_path { continue }

        // handle this case
        // 1: A -> (B -> Z)
        // 2: A -> B -> Z
        // 3: A -> (B -> C -> Z)
        // we need to find where the cycle start
        // in the example above, if we expand the path, we will know it start at step 1 (A -> B)
        // 1: A -> (B) -> Z
        // 2: A -> (B) -> Z
        // 3: A -> (B) -> C
        // from the point all cycle meet -> we can sum the LCM of all cycles
        let unwalked_path = max_path - path_counts[idx];
        result += unwalked_path / *c + (unwalked_path % *c != 0) as i64;
    }

    result += meet_cycle;

    result as u64
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
    
    let result = calc_steps(&network, path);
    println!("{result}");
}
