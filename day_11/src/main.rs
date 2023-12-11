use std::fs;

mod universe;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find input file");
    let map = universe::GalaxyMap::create(&input);

    let answer = map.sum_galaxy_distances(2);
    println!("Part 1 answer: {answer}");

    let answer = map.sum_galaxy_distances(1_000_000);
    println!("Part 1 answer: {answer}");
}
