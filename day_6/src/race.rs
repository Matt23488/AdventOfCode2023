#[derive(Debug)]
pub struct RaceData {
    time: u64,
    record_distance: u64,
}

#[derive(Debug)]
pub struct Races(Vec<RaceData>);

impl RaceData {
    pub fn find_hold_times_to_beat_record(&self) -> Vec<u64> {
        let range = 1..=self.time;

        range
            .map(|hold_time| hold_time * (self.time - hold_time))
            .filter(|&distance| distance > self.record_distance)
            .collect()
    }
}

impl RaceData {
    fn create(time: u64, record_distance: u64) -> Self {
        Self {
            time,
            record_distance,
        }
    }
}

impl Races {
    pub fn create(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();
        let time_line = lines[0];
        let record_line = lines[1];

        let times = time_line.trim_start_matches("Time:")
            .split(" ")
            .filter_map(get_numeric_value);

        let records = record_line.trim_start_matches("Distance:")
            .split(" ")
            .filter_map(get_numeric_value);

        let races = times.zip(records)
            .map(|(time, distance)| RaceData::create(time, distance))
            .collect();

        Self(races)
    }

    pub fn into_corrected(self) -> RaceData {
        let mut total_time = 0u64;
        let mut total_record = 0u64;

        for race in self.0 {
            let mut time = race.time;
            let mut record = race.record_distance;

            while time > 0 {
                total_time *= 10;
                time /= 10;
            }

            while record > 0 {
                total_record *= 10;
                record /= 10;
            }

            total_time += race.time;
            total_record += race.record_distance;
        }

        RaceData::create(total_time, total_record)
    }

    pub fn how_many_ways_to_beat_each_record(&self) -> Vec<u64> {
        self.0
            .iter()
            .map(|race| race.find_hold_times_to_beat_record().len() as u64)
            .collect()
    }
}

fn get_numeric_value(chunk: &str) -> Option<u64> {
    if chunk.len() == 0 {
        None
    } else {
        Some(chunk.trim().parse().unwrap())
    }
}
