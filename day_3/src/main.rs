use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find input file");
    let part_data = PartData::create(&input);

    let answer = part_data.get_part_number_sum();
    println!("Part 1 answer: {answer}");

    let answer = part_data.get_gear_ratio_sum();
    println!("Part 2 answer: {answer}");
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct GearIndex(usize, usize);

#[derive(Debug)]
struct PartData {
    part_numbers: Vec<u32>,
    gear_map: HashMap<GearIndex, Vec<u32>>,
}

impl PartData {
    fn create(input: &str) -> PartData {
        let lines = input.lines().collect::<Vec<_>>();
    
        let mut part_numbers = vec![];
        let mut gear_map: HashMap<GearIndex, Vec<u32>> = HashMap::new();
        let mut line_index = 0;
    
        for line in lines.iter() {
            let mut potential_part_number = 0;
            let mut is_part_number = false;
            let mut gear_indices = vec![];
    
            for (i, c) in line.char_indices() {
                match c {
                    d @ '0'..='9' => {
                        potential_part_number *= 10;
                        potential_part_number += d.to_digit(10).unwrap();
    
                        // Check current line
                        if i > 0 {
                            if let Some(c) = line.chars().nth(i - 1) {
                                validate_part_number(c, &mut is_part_number);
                                update_gear_indices(c, i - 1, line_index, &mut gear_indices);
                            }
                        }

                        if let Some(c) = line.chars().nth(i + 1) {
                            validate_part_number(c, &mut is_part_number);
                            update_gear_indices(c, i + 1, line_index, &mut gear_indices);
                        }

                        // Check previous line
                        if line_index > 0 {
                            if let Some(line) = lines.get(line_index - 1) {
                                if i > 0 {
                                    if let Some(c) = line.chars().nth(i - 1) {
                                        validate_part_number(c, &mut is_part_number);
                                        update_gear_indices(c, i - 1, line_index - 1, &mut gear_indices);
                                    }
                                }

                                if let Some(c) = line.chars().nth(i) {
                                    validate_part_number(c, &mut is_part_number);
                                    update_gear_indices(c, i, line_index - 1, &mut gear_indices);
                                }
        
                                if let Some(c) = line.chars().nth(i + 1) {
                                    validate_part_number(c, &mut is_part_number);
                                    update_gear_indices(c, i + 1, line_index - 1, &mut gear_indices);
                                }
                            }
                        }

                        // Check next line
                        if let Some(line) = lines.get(line_index + 1) {
                            if i > 0 {
                                if let Some(c) = line.chars().nth(i - 1) {
                                    validate_part_number(c, &mut is_part_number);
                                    update_gear_indices(c, i - 1, line_index + 1, &mut gear_indices);
                                }
                            }

                            if let Some(c) = line.chars().nth(i) {
                                validate_part_number(c, &mut is_part_number);
                                update_gear_indices(c, i, line_index + 1, &mut gear_indices);
                            }
    
                            if let Some(c) = line.chars().nth(i + 1) {
                                validate_part_number(c, &mut is_part_number);
                                update_gear_indices(c, i + 1, line_index + 1, &mut gear_indices);
                            }
                        }
                    }
                    _ => {
                        if is_part_number {
                            part_numbers.push(potential_part_number);
                        }

                        for gear_index in gear_indices.iter() {
                            if let Some(gear_nums) = gear_map.get_mut(gear_index) {
                                gear_nums.push(potential_part_number);
                            } else {
                                gear_map.insert(*gear_index, vec![potential_part_number]);
                            }
                        }
    
                        potential_part_number = 0;
                        is_part_number = false;
                        gear_indices = vec![];
                    }
                }
            }
    
            // Clean up edge-case after loop has ended
            if is_part_number {
                part_numbers.push(potential_part_number);
            }

            for gear_index in gear_indices.iter() {
                if let Some(gear_nums) = gear_map.get_mut(gear_index) {
                    gear_nums.push(potential_part_number);
                } else {
                    gear_map.insert(*gear_index, vec![potential_part_number]);
                }
            }
    
            line_index += 1;
        }
    
        PartData {
            part_numbers,
            gear_map,
        }
    }

    fn get_part_number_sum(&self) -> u32 {
        self.part_numbers.iter().sum()
    }

    fn get_gear_ratio_sum(&self) -> u32 {
        self.gear_map.iter()
            .filter_map(|(_, nums)| match nums[..] { [a, b] => Some(a * b), _ => None })
            .sum()
    }
}

fn validate_part_number(c: char, is_part_number: &mut bool) {
    if !c.is_numeric() && c != '.' {
        *is_part_number = true;
    }
}

fn update_gear_indices(c: char, i: usize, line_index: usize, gear_indices: &mut Vec<GearIndex>) {
    if c == '*' {
        let index = GearIndex(line_index, i);
        if !gear_indices.contains(&index) {
            gear_indices.push(index);
        }
    }
}
