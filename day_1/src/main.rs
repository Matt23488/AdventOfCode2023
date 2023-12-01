use std::fs;

fn main() {
    let calibration_sum = recover_calibration_value_sum("input.txt");
    println!("Part 1 Answer: {calibration_sum}");

    let calibration_sum = recover_calibration_value_sum_corrected("input.txt");
    println!("Part 2 Answer: {calibration_sum}");
}

fn recover_calibration_value_sum(filename: &str) -> u32 {
    let text = fs::read_to_string(filename).expect("Can't find file");
    
    let mut sum = 0;
    text
        .lines()
        .filter_map(|line| {
            let digits: Vec<u32> = line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect();


            match digits.len() {
                0 => None,
                _ => Some(digits.first().unwrap() * 10 + digits.last().unwrap()),
            }
        })
        .for_each(|calibration_value| sum += calibration_value);

    sum
}

fn recover_calibration_value_sum_corrected(filename: &str) -> u32 {
    let text = fs::read_to_string(filename).expect("Can't find file");
    
    let mut sum = 0;
    text
        .lines()
        .filter_map(|line| {
            let mut digits = vec![];


            let mut char_index = 0;
            for c in line.chars() {
                if c.is_numeric() {
                    digits.push(format!("{c}").parse::<u32>().unwrap());
                } else {
                    let substr = &line[char_index..];
                    if substr.starts_with("one") { digits.push(1); }
                    else if substr.starts_with("two") { digits.push(2); }
                    else if substr.starts_with("three") { digits.push(3); }
                    else if substr.starts_with("four") { digits.push(4); }
                    else if substr.starts_with("five") { digits.push(5); }
                    else if substr.starts_with("six") { digits.push(6); }
                    else if substr.starts_with("seven") { digits.push(7); }
                    else if substr.starts_with("eight") { digits.push(8); }
                    else if substr.starts_with("nine") { digits.push(9); }
                }

                char_index += 1;
            }


            match digits.len() {
                0 => None,
                _ => Some(digits.first().unwrap() * 10 + digits.last().unwrap()),
            }
        })
        .for_each(|calibration_value| sum += calibration_value);

    sum
}
