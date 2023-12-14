use std::io;

type Sequence = Box<Vec<i64>>;

fn parse_seq(buf: &str) -> Vec<i64> {
    let seq: Vec<&str> = buf.split(" ").collect();
    seq.iter().map(|t| t.parse::<i64>().unwrap()).collect()
}

fn find_gap(seq: &Sequence) -> (Sequence, bool) {
    let mut gap: Vec<i64> = Vec::new();
    let mut is_same = true;
    let mut prev_v = seq[1];

    gap.push(seq[1] - seq[0]);

    for v in seq.iter().skip(2) {
        let g = *v - prev_v;
        let lg = *gap.last().unwrap();
        
        if is_same && g != lg { is_same = false }
        
        prev_v = *v;
        gap.push(g);
    }

    (Box::new(gap), is_same)
}

fn find_next_value(seq: Vec<i64>) -> i64 {
    let seq_len = seq.len();
    
    if seq_len == 1 { return seq[0]; }

    let mut st: Vec<Sequence> = vec![Box::new(seq)];

    loop {
        let gap = st.last().unwrap();
        let (new_gap, is_same) = find_gap(gap);

        st.push(new_gap);
        if is_same { break }
    }

    while st.len() > 1 {
        let gap = st.pop().unwrap();
        let mut seq = st.pop().unwrap();
        let first = seq.first_mut().unwrap();

        *first -= gap.first().unwrap();
        
        st.push(seq);
    }
    
    let next_value = st.first().unwrap().first().unwrap();
    
    *next_value
}

fn main() {
    let mut buffer = String::new();
    let mut result = 0;

    loop {
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        if buffer == "\n" || buffer == "\r\n" || buffer.is_empty() {
            break;
        }
        
        let trimmed = buffer.trim();
        let seq = parse_seq(trimmed);
        
        result += find_next_value(seq);
        buffer.clear();
    }

    println!("{result}");
}
