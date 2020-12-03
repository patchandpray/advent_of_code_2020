use std::env;
use std::fs;

pub struct PasswordItem {
    policy: Policy,
    password: String,
}

impl PasswordItem {
    pub fn new(input: &str) -> Self {
        todo!()
    }
}


pub struct Policy {
    char_occurence: [i64; 2],
    char: char,
}

fn main() {

    // accept input file
    // iterate over inputs to
    // ...
    // get policy & get password from input
    // deconstruct policy
    // store policy<password> objects
    // then
    // ...
    // 
    // apply policy on password
    // returns True if valid, False if invalid
    // ...
    // count occurances of passwords adhering to policy
    // then
    // ...
    // go on holiday to tropical island 🌴

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let content = fs::read_to_string(filename).unwrap();
    let password_items: Vec<PasswordItem> = content.lines().map(|line| PasswordItem::new(line)).collect();


    println!("Hello, world!");
}