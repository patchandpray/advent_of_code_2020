use std::fs;
use std::env;

#[derive(Debug)]
struct BoardingPass {
    row: isize,
    column: isize,
}

impl BoardingPass {
    fn new(input: &str) -> Self {
        let row = isize::from_str_radix( &input[0..7], 2).unwrap();
        let column = isize::from_str_radix( &input[7..10], 2).unwrap();

        BoardingPass { row, column }
    }
    fn seat_id(&self) -> isize {
        self.row * 8 + self.column
    }
}

fn to_binary(input: &str) -> String {
    input.chars().map(|c| if c == 'B' || c == 'R'  { '1' } else { '0' }).collect()
}

fn main() {
    // collect puzzle input
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = fs::read_to_string(filename).unwrap();

    // transform string to binary representation
    let input: Vec<String> = input.lines().map(|line| to_binary(line)).collect();

    let bps: Vec<BoardingPass> = input.iter().map(|line| BoardingPass::new(line)).collect();

    let highest_seat_id: isize = bps.iter().map(|bp| bp.seat_id()).max().unwrap();

    println!("Highest seat id: {}", highest_seat_id);
}
