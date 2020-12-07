use std::env;
use std::fs;
use regex::Regex;

const REQUIRED_KEYS: [PassportKey; 7] =
    [
        PassportKey::Byr,
        PassportKey::Iyr,
        PassportKey::Eyr,
        PassportKey::Hgt,
        PassportKey::Hcl,
        PassportKey::Ecl,
        PassportKey::Pid,
    ];

#[derive(Debug)]
struct Passport {
    fields: Vec<PassportField>
}

impl Passport {
    fn new(input: &str) -> Self {
        // split input to key value pairs
        // delimited by spaces or newlines
        let fields: Vec<PassportField> = input.split_whitespace().map(|i| PassportField::new(i)).collect();

        Passport { fields }
    }

    fn get_passport_keys(&self) -> Vec<&PassportKey> {
        // returns a vector of all passport keys present in the passport object
        let keys: Vec<&PassportKey> = self.fields.iter().map(|f| f.key()).collect();
        keys
    }

    fn has_valid_fields(&self) -> bool {
        self.fields.iter().all(|f| f.valid)
    }

    fn has_valid_keys(&self) -> bool {
        let mut count = 0;
        for i in &REQUIRED_KEYS {
            if self.get_passport_keys().contains(&i) {
                count += 1;
            }
        }
        count == 7
    }

    fn is_valid(&self) -> bool {
        self.has_valid_fields() && self.has_valid_keys()
    }
}

#[derive(Debug)]
struct PassportField {
    key: PassportKey,
    value: String,
    valid: bool,
}

impl PassportField {
    fn new(input: &str) -> Self {
        let input: Vec<&str> = input.split(':').collect();
        let key: PassportKey = match input[0] {
            "byr" => Some(PassportKey::Byr),
            "iyr" => Some(PassportKey::Iyr),
            "eyr" => Some(PassportKey::Eyr),
            "hgt" => Some(PassportKey::Hgt),
            "hcl" => Some(PassportKey::Hcl),
            "ecl" => Some(PassportKey::Ecl),
            "pid" => Some(PassportKey::Pid),
            "cid" => Some(PassportKey::Cid),
            _ => None,
        }.unwrap();

        let value = input[1].to_string();

        let valid = validate(&key, &value);

        PassportField { key, value, valid }
    }

    fn key(&self) -> &PassportKey { &self.key
    }

}

#[derive(Debug, PartialEq)]
enum PassportKey {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

fn validate(key: &PassportKey, value: &String) -> bool {
    match key {
        PassportKey::Byr => validate_year(value,1920, 2002),
        PassportKey::Iyr => validate_year(value,2010, 2020),
        PassportKey::Eyr => validate_year(value,2020, 2030),
        PassportKey::Hgt => validate_height(value,),
        PassportKey::Hcl => validate_haircolour(value,),
        PassportKey::Ecl => validate_eyecolour(value,),
        PassportKey::Pid => validate_pid(value,),
        PassportKey::Cid => true,
        _ => false,
    }
}

fn validate_year(value: &String, low: i32, high: i32) -> bool {
    let value = value.parse::<i32>().unwrap();
    value >= low && value <= high
}

fn validate_height(value: &String) -> bool {
    let min_cm = 150;
    let max_cm = 193;
    let min_in = 50;
    let max_in = 76;

    let re = Regex::new(r"^(\d+)(\w+)$").unwrap();
    let captures = re.captures(value).unwrap();
    let height = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let unit = captures.get(2).unwrap().as_str();

    match unit {
        "cm" => height >= min_cm && height <= max_cm,
        "in" => height >= min_in && height <= max_in,
        _ => false,
    }
}

fn validate_eyecolour(value: &String) -> bool {
    let allowed_eyecolours = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    allowed_eyecolours.contains(&value.as_str())
}

fn validate_haircolour(value: &String) -> bool {
    let re = Regex::new(r"^#([a-f]|\d){6}$").unwrap();
    re.is_match(value.as_ref())
}

fn validate_pid(value: &String) -> bool {
    let re = Regex::new(r"^\d{9}$").unwrap();
    re.is_match(value.as_ref())
}

fn main() {
    // get puzzle input
    // generate passports from input
    // passport delimited by empty newline
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = fs::read_to_string(filename).unwrap();

    let passports: Vec<Passport> = input.split("\n\n").map(|p| Passport::new(p)).collect();

    let valid_passports: usize = passports.iter().filter(|p| p.is_valid()).count();

    println!("valid passports: {}", valid_passports);

}
