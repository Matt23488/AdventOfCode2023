use std::fs;

mod camel_cards;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find input file");
    let game = camel_cards::Game::create(&input);
    
    let answer = game.total_winnings();
    println!("Part 1 answer: {answer}");

    let answer = game.into_jacks_to_jokers().total_winnings();
    println!("Part 2 answer: {answer}");
}
