use std::fs;

mod oasis;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Coudln't find input file");
    let analysis = oasis::OasisAnaylsis::create(&input);

    let answer = analysis.extrapolated_sum(false);
    println!("Part 1 answer: {answer}");

    let answer = analysis.extrapolated_sum(true);
    println!("Part 2 answer: {answer}");
}
