use std::io;

const PATTERN_LENGTH: usize = 80;
const GENERATION_COUNT: usize = 20;

fn main() {
    println!("Rule 110");
    println!("========");
    println!("");

    let mut pattern_ok: bool = false;
    let mut pattern = String::new();
    let mut generation = 1;

    while !pattern_ok {
        println!("Enter a starting pattern: ");
        io::stdin().read_line(&mut pattern).is_ok();
        pattern_ok = validate_pattern(pattern.chars().collect());
        if !pattern_ok {
            println!("Pattern must only contain 0s and 1s");
            pattern.clear();
        }
    }

    let mut pattern_vec: Vec<char> = build_full_pattern(pattern.chars().collect());
    let printable_pattern: String = pattern_vec.iter().cloned().collect();
    println!("{}", printable_pattern); // print the full original pattern

    while generation <= GENERATION_COUNT {
        let next_gen = next_generation(&pattern_vec);
        let printable_next_gen: String = next_gen.iter().cloned().collect();
        println!("{}", printable_next_gen);
        pattern_vec = next_gen.clone();
        generation += 1;
    }
}

fn next_generation(pattern: &Vec<char>) -> Vec<char> {
    let mut next_pattern = Vec::new();
    next_pattern.push('0');
    let mut char_index = 1;
    let max_char_index = pattern.len() - 1;
    while char_index < max_char_index {
        let sub_pattern_start = char_index - 1;
        let sub_pattern_end = char_index + 2;
        let next_char = next_value(&pattern[sub_pattern_start..sub_pattern_end]);
        next_pattern.push(next_char);
        char_index += 1;
    }
    next_pattern.push(pattern[char_index]);
    next_pattern.to_owned()
}

fn next_value(sub_pattern: &[char]) -> char {
    // I'd like to use slice matching but it's not supported yet
    if sub_pattern == ['1', '1', '1'] || sub_pattern == ['0', '0', '1'] || sub_pattern == ['0', '0', '0'] {
        '0'
    } else {
        '1'
    }
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
