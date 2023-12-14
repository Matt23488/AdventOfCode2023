//! Note: The logic used here relies on how the input is crafted.
//! Namely, the fact that each rectangular field has exactly one reflection
//! both before and after fixing the smudges. If that were not true,
//! this code would fail to produce the correct answer.

use std::cmp;

#[derive(Debug)]
pub struct MirrorValley(Vec<Field>);

#[derive(Debug)]
struct Field {
    row_major: Vec<Vec<Cell>>,
    col_major: Vec<Vec<Cell>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Mirror,
    NoMirror,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

impl MirrorValley {
    pub fn create(input: &str) -> Self {
        let fields = input.split("\r\n\r\n")
            .map(Field::create)
            .collect();

        Self(fields)
    }

    pub fn note_summary(&self, smudge_count: usize) -> u64 {
        self.0
            .iter()
            .map(|field| field.summarize(smudge_count))
            .sum()
    }
}

impl Field {
    fn create(input: &str) -> Self {
        let row_major = input.lines()
            .map(Cell::create_many)
            .collect::<Vec<_>>();

        assert!(row_major.len() > 0);
        assert!(row_major[0].len() > 0);

        let mut col_major = vec![];
        col_major.reserve_exact(row_major[0].len());

        for i in 0..row_major[0].len() {
            let cols = row_major.iter()
                .map(|row| row[i])
                .collect();

            col_major.push(cols);
        }

        Self {
            row_major,
            col_major,
        }
    }

    fn summarize(&self, smudge_count: usize) -> u64 {
        Self::find_reflection(&self.col_major, &self.row_major, smudge_count).summarize()
    }

    fn find_reflection(col_major: &Vec<Vec<Cell>>, row_major: &Vec<Vec<Cell>>, smudge_count: usize) -> Reflection {
        if let Some(index) = Self::find_reflection_index(&col_major, smudge_count) {
            Reflection::Vertical(index)
        } else if let Some(index) = Self::find_reflection_index(&row_major, smudge_count) {
            Reflection::Horizontal(index)
        } else {
            panic!("No reflection found")
        }
    }

    fn find_reflection_index(field: &Vec<Vec<Cell>>, smudge_count: usize) -> Option<usize> {
        let mut reflection_index = None;

        for i in 0..(field.len() - 1) {
            let comparisons = cmp::min(i + 1, field.len() - i - 1);
            let mut difference = 0;
            for j in 0..comparisons {
                difference += field[i - j].difference(&field[i + j + 1]);
            }

            if difference == smudge_count {
                reflection_index = Some(i + 1);
                break;
            }
        }

        reflection_index
    }
}

impl Cell {
    fn create(input: char) -> Self {
        match input {
            '#' => Self::Mirror,
            '.' => Self::NoMirror,
            c => panic!("Invalid cell char: {c}"),
        }
    }

    fn create_many(input: &str) -> Vec<Self> {
        input.chars()
            .map(Self::create)
            .collect()
    }
}

impl Reflection {
    fn summarize(&self) -> u64 {
        match self {
            Self::Vertical(index) => *index as u64,
            Self::Horizontal(index) => *index as u64 * 100,
        }
    }
}

trait Difference {
    fn difference(&self, other: &Self) -> usize;
}

impl Difference for Vec<Cell> {
    fn difference(&self, other: &Self) -> usize {
        assert!(self.len() == other.len());

        self.iter()
            .zip(other)
            .filter(|(a, b)| a != b)
            .count()
    }
}
