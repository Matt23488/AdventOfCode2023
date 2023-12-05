use std::fs;

mod garden;
use garden::Garden;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find input file");
    let garden = Garden::create(&input);

    let answer = garden.get_nearest_location_discrete();
    println!("Part 1 answer: {answer}");

    let answer = garden.get_nearest_location_ranged();
    println!("Part 2 answer: {answer}");
}
