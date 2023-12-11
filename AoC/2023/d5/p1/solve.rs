use std::io;
use std::collections::HashSet;

fn get_seeds(buffer: &String) -> Vec<u64> {
    let seeds: Vec<u64>;
    let mut seed_strs: Vec<&str> = buffer.split(" ").collect();
    
    seed_strs.remove(0);
    seeds = seed_strs.iter().map(|s| s.trim().parse::<u64>().unwrap()).collect();

    seeds
}

fn map_num(seeds: &mut Vec<u64>, mapped: &mut HashSet<usize>, map: &Vec<u64>) {
    if let [dest, source, steps] = map[..] {
        for (idx, seed) in &mut seeds.iter_mut().enumerate() {
            if *seed < source { continue }
            if *seed >= source + steps { continue }
            if mapped.contains(&idx) { continue }

            *seed = dest + (*seed - source);
            mapped.insert(idx);
        }
    }
    
}

fn main() {
    let mut buffer = String::new();
    let mut skip_line = true;
    let mut mapped: HashSet<usize> = HashSet::new();

    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    
    let mut seeds = get_seeds(&buffer);
    println!("{:?}", seeds);
    buffer.clear();
    
    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");

        if buffer == "\n" || buffer == "\r\n" {
            buffer.clear();
            skip_line = true;
            continue;
        }

        if skip_line {
            buffer.clear();
            skip_line = false;
            mapped.clear();
            continue;
        }

        if buffer.is_empty() {
            break;
        }

        let map: Vec<u64> = buffer.split(" ").map(|data| data.trim().parse::<u64>().unwrap()).collect();
        
        map_num(&mut seeds, &mut mapped, &map);
        buffer.clear();
    }
    let mut result = u64::MAX;
    for seed in seeds.iter() {
        if *seed < result {
            result = *seed;
        }
    }
    println!("{result}");
}