use std::io;

fn main() {
    println!("Rule 110");
    println!("========");
    println!("");
    println!("Enter a starting patern: ");

    let mut pattern = String::new();

    io::stdin().read_line(&mut pattern).is_ok();
    println!("{}", validate_pattern(pattern.chars().collect()));

    let subpattern: Vec<char> = pattern.chars().skip(2).take(3).collect();

    let subpattern_str: String = subpattern.iter().cloned().collect();
    println!("{}", subpattern_str);
    println!("{}", subpattern[1]);
}

fn next_generation(pattern: &mut Vec<u8>) -> &mut Vec<u8> {
  let pattern_len = pattern.len();
  println!("length: {}", pattern_len);
  pattern
}

fn validate_pattern(pattern: Vec<char>) -> bool {
  pattern.iter().take(pattern.len() - 1).fold(true, |is_valid, c| is_valid && ['0', '1'].contains(c) )
}
