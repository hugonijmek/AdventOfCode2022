use std::fs;
use day03::process_part2;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part2(&file))
}
