use std::fs;

mod springs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find input file");
    let mut hot_springs = springs::HotSprings::create(&input);

    let answer = hot_springs.sum_row_arrangements();
    println!("Part 1 answer: {answer}");

    hot_springs.unfold();

    let answer = hot_springs.sum_row_arrangements();
    println!("Part 2 answer: {answer}");
}
