use std::str::Lines;

macro_rules! assert_unreachable {
    () => {
        panic!("Unreachable code was reached!")
    };
}

#[derive(Debug)]
pub struct Garden {
    pub seeds: Vec<u64>,
    seed_to_soil_map: Vec<Vec<u64>>,
    soil_to_fertilizer_map: Vec<Vec<u64>>,
    fertilizer_to_water_map: Vec<Vec<u64>>,
    water_to_light_map: Vec<Vec<u64>>,
    light_to_temperature_map: Vec<Vec<u64>>,
    temperature_to_humidity_map: Vec<Vec<u64>>,
    humidity_to_location_map: Vec<Vec<u64>>,
}

impl Garden {
    pub fn create(input: &str) -> Self {
        let mut lines = input.lines();

        let seeds = Self::parse_seeds(&mut lines);
        let seed_to_soil_map = Self::parse_map(&mut lines);
        let soil_to_fertilizer_map = Self::parse_map(&mut lines);
        let fertilizer_to_water_map = Self::parse_map(&mut lines);
        let water_to_light_map = Self::parse_map(&mut lines);
        let light_to_temperature_map = Self::parse_map(&mut lines);
        let temperature_to_humidity_map = Self::parse_map(&mut lines);
        let humidity_to_location_map = Self::parse_map(&mut lines);

        Garden {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        }
    }

    pub fn get_nearest_location_discrete(&self) -> u64 {
        self.get_nearest_location(&self.seeds)
    }

    pub fn get_nearest_location_ranged(&self) -> u64 {
        let mut h_to_l = self.humidity_to_location_map.clone();
        h_to_l.sort_by(|a, b| a[0].cmp(&b[0]));

        for entry in h_to_l {
            match entry[..] {
                [dest_start, _, length] => {
                    let dest_end = dest_start + length;
                    for location in dest_start..dest_end {
                        let seed = self.get_seed(location);
                        if self.has_seed(seed) {
                            return location;
                        }
                    }
                },
                _ => continue,
            }
        }

        assert_unreachable!();
    }

    fn get_nearest_location(&self, seeds: &Vec<u64>) -> u64 {
        let mut locations = seeds
            .iter()
            .map(|&seed| self.get_location(seed))
            .collect::<Vec<_>>();

        locations.sort();
        locations[0]
    }

    fn get_location(&self, seed: u64) -> u64 {
        let soil = Self::traverse_map(&self.seed_to_soil_map, seed);
        let fertilizer = Self::traverse_map(&self.soil_to_fertilizer_map, soil);
        let water = Self::traverse_map(&self.fertilizer_to_water_map, fertilizer);
        let light = Self::traverse_map(&self.water_to_light_map, water);
        let temperature = Self::traverse_map(&self.light_to_temperature_map, light);
        let humidity = Self::traverse_map(&self.temperature_to_humidity_map, temperature);
        let location = Self::traverse_map(&self.humidity_to_location_map, humidity);

        location
    }

    fn get_seed(&self, location: u64) -> u64 {
        let humidity = Self::traverse_map_backwards(&self.humidity_to_location_map, location);
        let temperature = Self::traverse_map_backwards(&self.temperature_to_humidity_map, humidity);
        let light = Self::traverse_map_backwards(&self.light_to_temperature_map, temperature);
        let water = Self::traverse_map_backwards(&self.water_to_light_map, light);
        let fertilizer = Self::traverse_map_backwards(&self.fertilizer_to_water_map, water);
        let soil = Self::traverse_map_backwards(&self.soil_to_fertilizer_map, fertilizer);
        let seed = Self::traverse_map_backwards(&self.seed_to_soil_map, soil);

        seed
    }

    fn has_seed(&self, seed: u64) -> bool {
        let mut seed_iter = self.seeds.iter();

        while let (Some(&start), Some(&length)) = (seed_iter.next(), seed_iter.next()) {
            if seed >= start && seed < start + length {
                return true;
            }
        }

        false
    }

    fn parse_seeds(lines: &mut Lines) -> Vec<u64> {
        let seeds = lines.next()
            .unwrap()
            .trim_start_matches("seeds: ")
            .split(" ")
            .map(|seed| seed.parse().unwrap())
            .collect();

        lines.next(); // move past empty line

        seeds
    }

    fn parse_map(lines: &mut Lines) -> Vec<Vec<u64>> {
        lines.next(); // move past heading

        let mut map = vec![];

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            let entry = line.split(" ")
                .map(|num| num.parse().unwrap())
                .collect();

            map.push(entry);
        }

        map
    }

    fn traverse_map(map: &Vec<Vec<u64>>, input: u64) -> u64 {
        for entry in map {
            match entry[..] {
                [dest_start, source_start, length] => {
                    if input >= source_start && input < source_start + length {
                        return input - source_start + dest_start;
                    }
                }
                _ => continue,
            }
        }

        input
    }

    fn traverse_map_backwards(map: &Vec<Vec<u64>>, input: u64) -> u64 {
        for entry in map {
            match entry[..] {
                [dest_start, source_start, length] => {
                    if input >= dest_start && input < dest_start + length {
                        return input - dest_start + source_start;
                    }
                }
                _ => continue,
            }
        }

        input
    }
}