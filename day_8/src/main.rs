use std::fs;

mod map;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find input file");
    let map = map::DesertMap::create(&input);

    let answer = map.steps_to_traverse(|&location| location == "AAA", |&location| location == "ZZZ");
    println!("Part 1 answer: {answer}");

    let answer = map.steps_to_traverse(|location| location.ends_with("A"), |location| location.ends_with("Z"));
    println!("Part 2 answer: {answer}");
}
