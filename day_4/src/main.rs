use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find input file");

    let cards = input.lines()
        .map(ScratchCard::create)
        .collect::<Vec<_>>();

    let answer = get_total_points(&cards);
    println!("Part 1 answer: {answer}");

    let answer = get_total_scratchcard_count(&cards);
    println!("Part 2 answer: {answer}");
}

fn get_total_points(cards: &Vec<ScratchCard>) -> u32 {
    cards.iter()
        .map(ScratchCard::get_points)
        .sum()
}

fn get_total_scratchcard_count(cards: &Vec<ScratchCard>) -> u32 {
    let mut counts = cards.iter().map(|_| 1).collect::<Vec<_>>();

    for i in 0..cards.len() {
        let win_count = cards[i].get_win_count();
        for j in 1..=win_count {
            counts[i + j] += counts[i];
        }
    }

    counts.iter().sum()
}

#[derive(Debug)]
struct ScratchCard {
    _id: u32,
    winning_nums: Vec<u32>,
    player_nums: Vec<u32>,
}

impl ScratchCard {
    fn create(input: &str) -> Self {
        let parts = input.split(":").collect::<Vec<_>>();
        let id_part = parts.first().unwrap();
        let rest = parts.last().unwrap();

        let id = id_part.trim_start_matches("Card").trim().parse().unwrap();

        let parts = rest.split("|").collect::<Vec<_>>();
        let winning_nums_part = parts.first().unwrap().trim();
        let player_nums_part = parts.last().unwrap().trim();

        let winning_nums = parse_nums(winning_nums_part);
        let player_nums = parse_nums(player_nums_part);

        Self {
            _id: id,
            winning_nums,
            player_nums,
        }
    }

    fn get_win_count(&self) -> usize {
        self.player_nums.iter()
            .filter(|num| self.winning_nums.contains(num))
            .count()
    }

    fn get_points(&self) -> u32 {
        let mut points = 0;

        for winning_num in self.winning_nums.iter() {
            if self.player_nums.contains(winning_num) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        points
    }
}

fn parse_nums(input: &str) -> Vec<u32> {
    input
        .split(" ")
        .filter_map(|num_str| num_str.parse().ok())
        .collect()
}
