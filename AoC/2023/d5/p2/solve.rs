use std::io;
use std::collections::HashSet;

fn get_seeds(buffer: &String) -> Vec<(u64, u64)> {
    let seeds: Vec<u64>;
    let mut seed_pairs: Vec<(u64, u64)> = Vec::new();
    let mut seed_strs: Vec<&str> = buffer.split(" ").collect();
    
    seed_strs.remove(0); // remove "seed:"
    seeds = seed_strs.iter().map(|s| s.trim().parse::<u64>().unwrap()).collect();

    for idx in (0..seeds.len()).step_by(2) {
        seed_pairs.push((seeds[idx], seeds[idx + 1]));
    }

    seed_pairs
}

fn map_num(seed_pairs: &mut Vec<(u64, u64)>, mapped: &mut HashSet<usize>, map: &Vec<u64>) {
    let mut new_pairs: Vec<(u64, u64)> = Vec::new();
    loop {
        if let [dest, source, steps] = map[..] {
            for (idx, seed) in &mut seed_pairs.iter_mut().enumerate() {
                let (s_min, s_max) = (source, source + steps);
                if seed.0 + seed.1 <= s_min { continue } // check [.., a] .. [b, ..] -> not overlap
                if seed.0 >= s_max { continue } // check [.., b] .. [a, ..] -> not overlap
                if mapped.contains(&idx) { continue }
    
                mapped.insert(idx);
                
                
                if seed.0 < s_min && seed.0 + seed.1 > s_min {
                    let l = s_min - seed.0;
                    new_pairs.push((seed.0, l));
                    seed.1 -= l;
                    seed.0 = s_min;
                }

                if seed.0 == s_min && seed.1 <= steps {
                    seed.0 = dest;
                    continue;
                }

                if seed.0 == s_min && seed.1 > steps {
                    new_pairs.push((seed.0 + steps, seed.1 - steps));
                    seed.0 = dest;
                    seed.1 = steps;
                    continue;
                }

                if seed.0 > s_min && seed.0 + seed.1 <= s_max {
                    seed.0 = dest + (seed.0 - s_min);
                    continue;
                }

                if seed.0 > s_min && seed.0 + seed.1 > s_max {
                    new_pairs.push((seed.0 + steps, (seed.0 + seed.1) - s_max));
                    seed.0 = dest + (seed.0 - s_min);
                    seed.1 = steps;
                    continue;
                }

            }
        }
        if new_pairs.is_empty() { break }
        seed_pairs.append(&mut new_pairs);
        new_pairs.clear();
    }
}

fn main() {
    let mut buffer = String::new();
    let mut skip_line = true;
    let mut mapped: HashSet<usize> = HashSet::new();

    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    
    let mut seed_pairs = get_seeds(&buffer);
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
        
        map_num(&mut seed_pairs, &mut mapped, &map);
        buffer.clear();
    }
    let mut result = u64::MAX;
    for seed in seed_pairs.iter() {
        if seed.0 < result {
            result = seed.0;
        }
    }
    println!("{result}");
}