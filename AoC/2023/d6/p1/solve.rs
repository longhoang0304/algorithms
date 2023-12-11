use std::io;

fn get_nums(buffer: &String) -> Vec<u64> {
    let nums: Vec<u64>;
    let mut nums_strs: Vec<&str> = buffer.split(" ").collect();
    
    nums_strs.remove(0); // remove "xxx:"
    nums = nums_strs.iter().map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.parse::<u64>().unwrap()).collect();

    nums
}


fn binary_search_lower(max: u64, target: u64) -> Option<u64> {
    let mut lower = 0;
    let mut upper = max;
    while lower <= upper {
        let mid = (upper + lower) >> 1;
        let v = (max - mid) * mid;
        
        if mid == 0 && v > target { return Some(mid) }
        if mid == 0 && v <= target {
            lower = mid + 1;
            continue;
        }
        
        let v_lower = (max - (mid - 1)) * (mid - 1);

        if v_lower <= target && v > target { return Some(mid) }
        if v_lower <= target { lower = mid + 1; }
        else { upper = mid - 1; }
    }
    None
}

fn binary_search_higher(max: u64, target: u64) -> Option<u64> {
    let mut lower = 0;
    let mut upper = max;
    while lower <= upper {
        let mid = (upper + lower) >> 1;
        let v = (max - mid) * mid;
        
        if mid == max && v > target { return Some(mid) }
        if mid == max && v <= target {
            upper = mid - 1;
            continue;
        }
        
        let v_upper = (max - (mid + 1)) * (mid + 1);

        if v_upper <= target && v > target { return Some(mid) }
        if v_upper <= target { upper = mid - 1; }
        else { lower = mid + 1; }
    }
    None
}


fn find_fastest(time: u64, dist: u64) -> u64 {
    let lower_opt = binary_search_lower(time, dist);
    let upper_opt = binary_search_higher(time, dist);
    match (lower_opt, upper_opt) {
        (Some(lower), Some(upper)) => upper - lower + 1,
        _ => 0
    }
}


fn find_wins(times: &Vec<u64>, distances: &Vec<u64>) -> u64 {
    let mut result = 1;
    for idx in 0..times.len() {
        result *= find_fastest(times[idx], distances[idx]);
    }
    result
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    let times = get_nums(&buffer);
    buffer.clear();

    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    let distances = get_nums(&buffer);
    buffer.clear();

    let result = find_wins(&times, &distances);
    
    println!("{result}");
}
