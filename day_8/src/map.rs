use std::collections::HashMap;

#[derive(Debug)]
pub struct DesertMap {
    path: String,
    locations: HashMap<String, (String, String)>,
}

impl DesertMap {
    pub fn create(input: &str) -> Self {
        let mut lines = input.lines();

        let path = lines.next().unwrap().into();

        lines.next();

        let mut locations = HashMap::new();

        lines
            .for_each(|line| {
                let name = line[..3].into();
                let left = line[7..10].into();
                let right = line[12..15].into();

                locations.insert(name, (left, right));
            });

        Self {
            path,
            locations,
        }
    }

    // NOTE: I don't think this will work in general, but after analyzing the input a bit
    // I discovered that the number of steps it takes to get from the start to the end
    // is the same on every iteration of looping the path over and over forever. This
    // is true of at least every location that ends in an 'A'. This is likely by design
    // to make this solution possible, where all you have to do is calculate the steps
    // individually and then calculate the least common multiple of all the results.
    pub fn steps_to_traverse<S, E>(&self, start_predicate: S, end_predicate: E) -> u64
    where
        S: Fn(&&String) -> bool,
        E: Fn(&&String) -> bool,
    {
        self.locations
            .keys()
            .filter(start_predicate)
            .map(|location| self.steps_to_traverse_single(location, &end_predicate))
            .fold(1, lcm)
    }
}

impl DesertMap {
    fn steps_to_traverse_single<E>(&self, start: &String, end_predicate: E) -> u64
    where
        E: Fn(&&String) -> bool,
    {
        let mut steps = 0;
        let mut current = start;

        loop {
            for step in self.path.chars() {
                if end_predicate(&current) {
                    return steps;
                }

                let (left, right) = self.locations.get(current).unwrap();

                current = match step {
                    'L' => left,
                    _ => right,
                };

                steps += 1;
            }
        }
    }
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        (min, max) = (max, min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            break min;
        }

        max = min;
        min = res;
    }
}
