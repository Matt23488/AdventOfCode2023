#[derive(Debug)]
pub struct Maze {
    start: usize,
    width: usize,
    height: usize,
    pipes: Vec<Pipe>,
}

#[derive(Debug)]
pub struct Pipe {
    cell: char,
    neighbor_1: usize,
    neighbor_2: usize,
    index: usize,
}

impl Maze {
    pub fn create(input: &str) -> Self {
        let cells = input.lines()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>();

        let height = input.lines().count();
        let width = cells.len() / height;

        let pipes = cells.iter()
            .enumerate()
            .map(|(i, &cell)| Pipe::create(&cells, cell, i as i32, width as i32, height as i32))
            .collect();

        Self {
            start: cells.iter().position(|&cell| cell == 'S').expect("No start cell found"),
            width,
            height,
            pipes,
        }
    }

    pub fn steps_to_farthest_point_from_start(&mut self) -> usize {
        let mut prev = self.start;
        let mut curr = self.pipes[prev].neighbor_1;
        let mut loop_size = 1;
        let mut loop_indices = vec![curr];

        while curr != self.start {
            let pipe = &self.pipes[curr];

            (prev, curr) = (
                curr,
                if pipe.neighbor_1 != prev {
                    pipe.neighbor_1
                } else {
                    pipe.neighbor_2
                }
            );

            loop_indices.push(curr);
            loop_size += 1;
        }

        self.pipes.iter_mut().for_each(|pipe| if !loop_indices.contains(&pipe.index) {
            pipe.cell = '.';
        });

        loop_size / 2
    }

    pub fn num_cell_enclosed_by_loop(&self) -> u32 {
        let mut result = 0;

        for y in 0..self.height {
            let mut inside = false;
            let mut last_shape = None;
            for x in 0..self.width {
                let i = coords_to_index((x, y), self.width);
                let pipe = &self.pipes[i];

                match (last_shape.take(), pipe.cell) {
                    (None, '|') |
                    (Some('F'), 'J') |
                    (Some('L'), '7') => inside = !inside,
                    (shape, '-') => last_shape = shape,
                    
                    (None, shape @ ('F' | 'L')) => last_shape = Some(shape),
                    (None, '.') => {
                        if inside {
                            result += 1
                        }
                    }
                    _ => (),
                }
            }
        }
        
        result
    }
}

impl Pipe {
    fn create(cells: &Vec<char>, cell: char, index: i32, width: i32, height: i32) -> Self {
        let (cell, neighbor_1, neighbor_2) = match cell {
            '.' => ('.', 0, 0),
            '|' => ('|', index - width, index + width),
            '-' => ('-', index - 1, index + 1),
            'L' => ('L', index - width, index + 1),
            'J' => ('J', index - width, index - 1),
            '7' => ('7', index - 1, index + width),
            'F' => ('F', index + 1, index + width),
            'S' => {
                let mut a = None;
                let mut b = None;
                let mut up = false;
                let mut down = false;
                let mut left = false;
                let mut right = false;

                let (x, y) = index_to_coords(index, width);

                let mut n_index = index - width;
                if y > 0 && "|7F".contains(cells[n_index as usize]) {
                    insert_next(&mut a, &mut b, n_index);
                    up = true;
                }

                n_index = index + width;
                if y < height - 1 && "|LJ".contains(cells[n_index as usize]) {
                    insert_next(&mut a, &mut b, n_index);
                    down = true;
                }

                n_index = index - 1;
                if x > 0 && "-LF".contains(cells[n_index as usize]) {
                    insert_next(&mut a, &mut b, n_index);
                    left = true;
                }

                n_index = index + 1;
                if x < width - 1 && "-J7".contains(cells[n_index as usize]) {
                    insert_next(&mut a, &mut b, n_index);
                    right = true;
                }

                let actual_cell = if up {
                    if left {
                        'J'
                    } else if down {
                        '|'
                    } else if right {
                        'L'
                    } else {
                        panic!("Impossible start cell configuration");
                    }
                } else if down {
                    if left {
                        '7'
                    } else if right {
                        'F'
                    } else {
                        panic!("Impossible start cell configuration");
                    }
                } else {
                    '-'
                };

                (
                    actual_cell,
                    a.expect("Start tile has no connecting tiles"),
                    b.expect("Start tile has only one connecting tile"),
                )
            }

            t => panic!("Unexpected tile: {t}"),
        };

        let (neighbor_1, neighbor_2, index) = (
            neighbor_1 as usize,
            neighbor_2 as usize,
            index as usize,
        );

        Self {
            cell,
            neighbor_1,
            neighbor_2,
            index,
        }
    }
}

fn index_to_coords(i: i32, width: i32) -> (i32, i32) {
    (i % width, i / width)
}

fn coords_to_index(coords: (usize, usize), width: usize) -> usize {
    coords.1 * width + coords.0
}

fn insert_next<T>(a: &mut Option<T>, b: &mut Option<T>, value: T) {
    if a.is_none() {
        let _ = a.insert(value);
    } else {
        let _ = b.insert(value);
    }
}
