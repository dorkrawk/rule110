use std::io;

const PATTERN_LENGTH: usize = 10;

fn main() {
    println!("Rule 110");
    println!("========");
    println!("");

    let mut pattern_ok: bool = false;
    let mut pattern = String::new();

    while !pattern_ok {
        println!("Enter a starting pattern: ");
        io::stdin().read_line(&mut pattern).is_ok();
        pattern_ok = validate_pattern(pattern.chars().collect());
        if !pattern_ok {
            println!("Pattern must only contain 0s and 1s");
            pattern.clear();
        }
    }

    let pattern_vec: Vec<char> = build_full_pattern(pattern.chars().collect());
    let printable_pattern: String = pattern_vec.iter().cloned().collect();
    println!("full pattern: {}", printable_pattern);
    println!("pattern len: {}", printable_pattern.len());

    let subpattern: Vec<char> = pattern.chars().skip(2).take(3).collect();

    let subpattern_str: String = subpattern.iter().cloned().collect();
    //println!("{}", subpattern_str);
    //println!("{}", subpattern[1]);
}

fn next_generation(pattern: &mut Vec<u8>) -> &mut Vec<u8> {
    let pattern_len = pattern.len();
    println!("length: {}", pattern_len);
    pattern
}

fn validate_pattern(pattern: Vec<char>) -> bool {
    pattern.iter().take(pattern.len() - 1).fold(true, |is_valid, c| is_valid && ['0', '1'].contains(c) )
}

fn build_full_pattern(pattern: Vec<char>) -> Vec<char> {
    let pattern_len = pattern.len();
    if pattern_len > PATTERN_LENGTH {
        let resized_pattern = &pattern[..PATTERN_LENGTH];
        return resized_pattern.to_owned()
    } else {
        let mut resized_pattern = pattern.clone();
        resized_pattern.pop(); // remove empty last char
        let buffer_size_needed = PATTERN_LENGTH - (pattern_len - 1);
        let buffer_vec = vec!['0'; buffer_size_needed];
        resized_pattern.extend(buffer_vec.into_iter());
        return resized_pattern.to_owned()
    }
}
