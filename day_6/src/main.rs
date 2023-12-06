use std::fs;

mod race;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find input file");

    let races = race::Races::create(&input);
    let ways_to_beat = races.how_many_ways_to_beat_each_record();
    
    let answer: u64 = ways_to_beat.into_iter().product();
    println!("Part 1 answer: {answer}");

    let race = races.into_corrected();
    let ways_to_beat = race.find_hold_times_to_beat_record();
    
    let answer = ways_to_beat.len();
    println!("Part 2 answer: {answer}");
}
