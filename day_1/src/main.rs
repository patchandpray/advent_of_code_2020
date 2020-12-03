use std::env;
use std::fs;

fn main() {

    // get inputs from file
    let args: Vec<String> = env::args().collect();
    let filename= &args[1];
    let inputs = fs::read_to_string(filename).unwrap();

    // Collect newline delimited inputs into a vector of things we can calculate on
    let inputs = inputs.lines().map(|line| line.parse::<u64>()).collect::<Result<Vec<u64>, _>>().unwrap();

    let result = get_2020(inputs);

    match result {
        Some(value) => println!("{}", value),
        None => println!("No result"),
    };
}

fn get_2020(inputs: Vec<u64>) -> Option<u64> {
    // Get an item from inputs, calculate versus all other items in inputs if sum == 2020
    // If sum == 2020 multiply the inputs
    for x in &inputs {
        for y in &inputs {
            if x + y == 2020 {
                println!("{} + {} = 2020", x, y);
                return Some(x * y)
            }
        }
    };
    None
}