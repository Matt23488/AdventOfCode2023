use std::fs;

fn main() {
    let part_1_answer = find_id_sum_of_possible_games("input.txt");
    println!("Part 1 Answer: {part_1_answer}");

    let part_2_answer = find_power_sum_of_all_games("input.txt");
    println!("Part 2 Answer: {part_2_answer}");
}

fn find_id_sum_of_possible_games(filename: &str) -> u32 {
    fs::read_to_string(filename)
        .expect("Couldn't find input file")
        .lines()
        .map(CubeGame::create)
        .filter(|game| game.is_possible_with(12, 13, 14))
        .map(|game| game.id)
        .sum()
}

fn find_power_sum_of_all_games(filename: &str) -> u32 {
    fs::read_to_string(filename)
        .expect("Couldn't find input file")
        .lines()
        .map(CubeGame::create)
        .map(CubeGame::into_power)
        .sum()
}

struct CubeGame {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

impl CubeGame {
    fn create(input: &str) -> CubeGame {
        let parts = input.split(":").collect::<Vec<_>>();
        let id_part = parts.first().unwrap();
        let rest = parts.last().unwrap();

        let id = id_part.trim_start_matches("Game ").parse().unwrap();
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let parts = rest.split(";");
        for part in parts {
            let color_counts = part.split(",");
            for color_count in color_counts {
                let parts = color_count.trim().split(" ").collect::<Vec<_>>();
                let num = parts.first().unwrap().trim().parse().unwrap();
                let color = parts.last().unwrap();

                match color {
                    &"red" => {
                        if max_red < num {
                            max_red = num;
                        }
                    }
                    &"green" => {
                        if max_green < num {
                            max_green = num;
                        }
                    }
                    &"blue" => {
                        if max_blue < num {
                            max_blue = num;
                        }
                    }
                    _ => (),
                };
            }
        }

        CubeGame {
            id,
            max_red,
            max_green,
            max_blue,
        }
    }

    fn is_possible_with(&self, red: u32, green: u32, blue: u32) -> bool {
        self.max_red <= red && self.max_green <= green && self.max_blue <= blue
    }

    fn into_power(self) -> u32 {
        self.max_red * self.max_green * self.max_blue
    }
}
