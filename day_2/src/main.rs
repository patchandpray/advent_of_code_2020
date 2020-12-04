use std::env;
use std::fs;

use regex::Regex;

#[derive(Debug)]
pub struct PasswordItem {
    min: usize,
    max: usize,
    c: char,
    password: String,
}

impl PasswordItem {
    pub fn new(input: &str) -> Self {
        // deconstruct input to policy and password
        let re = Regex::new(r"^(\d+)-(\d+)\s(\w):\s(\w+)$").expect("regex failed");
        let captures = re.captures(input).unwrap();

        let min = captures.get(1).unwrap().as_str().parse().unwrap();
        let max = captures.get(2).unwrap().as_str().parse().unwrap();
        let c = captures.get(3).unwrap().as_str().chars().next().unwrap();
        let password = captures.get(4).unwrap().as_str().to_string();

        PasswordItem{min, max, c, password}
    }

    pub fn is_valid_old_policy(&self) -> bool {
        // check if char occurs min
        // check if char occurs max

        let count = &self.password.matches(self.c).count();
        !(count < &self.min || count > &self.max)
    }

    pub fn is_valid_new_policy(&self) -> bool {
        // check if c occurs on any of the two allowed positions(min, max) but not both or none

        self.password.char_indices().fold(false, |valid, (i, c)| {
            if i == self.min-1 || i == self.max-1 {
                valid ^ (c == self.c)
            } else {
                valid
            }
        })
    }
}


fn main() {
    // accept input file
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // iterate over inputs to
    // get policy & get password from input
    // deconstruct policy
    // and
    // store policy<password> objects
    let content = fs::read_to_string(filename).unwrap();
    let password_items: Vec<PasswordItem> = content.lines().map(|line| PasswordItem::new(line)).collect();

    // then
    // ...
    // apply policy on password
    // count occurances of passwords adhering to policy
    let valid_passwords: usize = password_items.iter().filter(|item| item.is_valid_old_policy()).count();
    println!("valid passwords old policy: {}", valid_passwords);

    let valid_passwords: usize = password_items.iter().filter(|item| item.is_valid_new_policy()).count();
    println!("valid passwords new policy: {}", valid_passwords);
    // ...
    // go on holiday to tropical island 🌴

}