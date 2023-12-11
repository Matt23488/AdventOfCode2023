#[derive(Debug)]
pub struct GalaxyMap {
    expanded_rows: Vec<u64>,
    expanded_cols: Vec<u64>,
    galaxies: Vec<(usize, usize)>,
}

impl GalaxyMap {
    pub fn create(input: &str) -> Self {
        let map = input.lines()
            .map(|line| line
                .chars()
                .collect()
            )
            .collect::<Vec<Vec<_>>>();

        let height = map.len();
        assert!(height > 0);

        let width = map[0].len();
        assert!(width > 0);

        let mut expanded_rows = vec![];
        let mut empty_cells = (0..width).map(|_| true).collect::<Vec<_>>();
        let mut galaxies = vec![];

        for y in 0..height {
            if map[y].iter().all(|&cell| cell == '.') {
                expanded_rows.push(y as u64);
            }

            for x in 0..width {
                match &map[y][x] {
                    '#' => {
                        empty_cells[x] = false;
                        galaxies.push((x, y));
                    }
                    _ => (),
                }
            }
        }

        let expanded_cols = empty_cells
            .into_iter()
            .enumerate()
            .filter_map(|(i, is_empty)| {
                if is_empty {
                    Some(i as u64)
                } else {
                    None
                }
            })
            .collect();

        Self {
            expanded_rows,
            expanded_cols,
            galaxies,
        }
    }

    pub fn sum_galaxy_distances(&self, expansion_rate: u64) -> u64 {
        let mut sum = 0;
        let expansion_rate = expansion_rate - 1;

        for i in 0..(self.galaxies.len() - 1) {
            let &(ax, ay) = &self.galaxies[i];
            let (ax, ay) = (ax as u64, ay as u64);

            for j in (i + 1)..self.galaxies.len() {
                let &(bx, by) = &self.galaxies[j];
                let (bx, by) = (bx as u64, by as u64);

                let (ax, bx) = if ax > bx { (bx, ax) } else { (ax, bx) };
                let (ay, by) = if ay > by { (by, ay) } else { (ay, by) };

                let row_expansions = self.expanded_rows
                    .iter()
                    .filter(|&&row| row > ay && row < by)
                    .count() as u64;

                let col_expansions = self.expanded_cols
                    .iter()
                    .filter(|&&col| col > ax && col < bx)
                    .count() as u64;

                sum +=
                    bx - ax +
                    by - ay +
                    row_expansions * expansion_rate +
                    col_expansions * expansion_rate;
            }
        }

        sum
    }
}