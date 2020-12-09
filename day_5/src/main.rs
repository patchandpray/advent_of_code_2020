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
    let bps: Vec<BoardingPass> = input.lines().map(|line| BoardingPass::new(to_binary(line).as_ref())).collect();

    // get seat id's
    let highest_seat_id: isize = bps.iter().map(|bp| bp.seat_id()).max().unwrap();
    let lowest_seat_id: isize = bps.iter().map(|bp| bp.seat_id()).min().unwrap();

    println!("Highest seat id: {}", highest_seat_id);
    println!("Lowest seat id: {}", lowest_seat_id);

    // determine your seat
    for i in lowest_seat_id..highest_seat_id {
        if !(bps.iter().any(|bp| bp.seat_id() == i)) {
            println!("Your seat id: {}", i);
        }
    }
}
