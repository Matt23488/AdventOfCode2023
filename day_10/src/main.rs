use std::fs;

mod pipes;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find input file");
    let mut maze = pipes::Maze::create(&input);
    
    let answer = maze.steps_to_farthest_point_from_start();
    println!("Part 1 answer: {answer}");

    let answer = maze.num_cell_enclosed_by_loop();
    println!("Part 2 answer: {answer}");
}
