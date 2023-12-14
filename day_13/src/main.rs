use std::fs;

use day_13::MirrorValley;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find input file");
    let valley = MirrorValley::create(&input);

    let answer = valley.note_summary(0);
    println!("Part 1 answer: {answer}");

    let answer = valley.note_summary(1);
    println!("Part 2 answer: {answer}");
}
