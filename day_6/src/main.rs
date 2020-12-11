use std::env;
use std::fs;

fn read_input() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).unwrap()
}

fn main() {
    let input = read_input();
    let input: Vec<&str> = input.split("\n\n").collect();
    let input: Vec<String> = input.iter().map(|l| l.replace("\n", "")).collect();
    
    let count: usize = input.iter().map(|line| count_unique(line)).sum();
    println!("sum of 'yes' count: {}", count);

}

fn count_unique(input: &String) -> usize {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort();
    chars.dedup();
    chars.len()
}