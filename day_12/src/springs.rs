use std::collections::HashMap;

#[derive(Debug)]
pub struct HotSprings(Vec<Row>);

#[derive(Debug)]
struct Row {
    counts: Vec<usize>,
    springs: String,
}

impl HotSprings {
    pub fn create(input: &str) -> Self {
        let rows = input.lines()
            .map(Row::create)
            .collect();

        Self(rows)
    }

    pub fn sum_row_arrangements(&self) -> u64 {
        self.0
            .iter()
            .map(Row::arrangements)
            .sum()
    }

    pub fn unfold(&mut self) {
        self.0
            .iter_mut()
            .for_each(|row| row.unfold());
    }
}

impl Row {
    fn create(input: &str) -> Self {
        let (springs, counts) = match &input.split(" ").collect::<Vec<_>>()[..] {
            [springs, counts] => (String::from(*springs), *counts),
            _ => panic!("Invalid row format"),
        };

        let counts = counts.split(",")
            .map(|count| count.parse().expect("Invalid group count format"))
            .collect();

        Self {
            counts,
            springs,
        }
    }

    fn unfold(&mut self) {
        self.counts = self.counts.repeat(5);
        self.springs = vec![
            &self.springs[..],
            &self.springs[..],
            &self.springs[..],
            &self.springs[..],
            &self.springs[..],
        ].join("?");
    }

    fn arrangements(&self) -> u64 {
        let mut memo = HashMap::new();
        self.arrangements_recursive(&self.springs, &self.counts[..], 0, &mut memo)
    }

    fn arrangements_recursive<'a>(&self, springs: &'a str, counts: &'a [usize], current_group_size: usize, memo: &mut HashMap<(&'a str, &'a [usize], usize), u64>) -> u64 {
        if let Some(&result) = memo.get(&(springs, counts, current_group_size)) {
            result
        } else if counts.len() == 0 {
            if springs.chars().all(|c| c != '#') {
                memo.insert((springs, counts, current_group_size), 1);
                1
            } else {
                memo.insert((springs, counts, current_group_size), 0);
                0
            }
        } else if springs.len() == 0 {
            if counts.len() == 1 && counts[0] == current_group_size {
                memo.insert((springs, counts, current_group_size), 1);
                1
            } else {
                memo.insert((springs, counts, current_group_size), 0);
                0
            }
        } else {
            match springs.chars().nth(0) {
                Some('.') => {
                    if current_group_size != counts[0] {
                        if current_group_size == 0 {
                            let result = self.arrangements_recursive(&springs[1..], counts, 0, memo);
                            memo.insert((springs, counts, current_group_size), result);
                            result
                        } else {
                            memo.insert((springs, counts, current_group_size), 0);
                            0
                        }
                    } else {
                        let result = self.arrangements_recursive(&springs[1..], &counts[1..], 0, memo);
                        memo.insert((springs, counts, current_group_size), result);
                        result
                    }
                }
                Some('#') => {
                    if current_group_size == counts[0] {
                        memo.insert((springs, counts, current_group_size), 0);
                        0
                    } else {
                        let result = self.arrangements_recursive(&springs[1..], counts, current_group_size + 1, memo);
                        memo.insert((springs, counts, current_group_size), result);
                        result
                    }
                }
                Some('?') => {
                    if current_group_size == counts[0] {
                        let result = self.arrangements_recursive(&springs[1..], &counts[1..], 0, memo);
                        memo.insert((springs, counts, current_group_size), result);
                        result
                    } else if current_group_size == 0 {
                        let a = self.arrangements_recursive(&springs[1..], counts, 0, memo);
                        let b = self.arrangements_recursive(&springs[1..], counts, current_group_size + 1, memo);
                        let result = a + b;
                        memo.insert((springs, counts, current_group_size), result);
                        result
                    } else {
                        let result = self.arrangements_recursive(&springs[1..], counts, current_group_size + 1, memo);
                        memo.insert((springs, counts, current_group_size), result);
                        result
                    }
                }
                c => panic!("Unexpected char: {c:?}"),
            }
        }
    }
}
