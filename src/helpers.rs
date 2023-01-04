use itertools::Itertools;
use parse_display::FromStr;
use std::fmt::Debug;
use std::fmt::Display;
use std::io::Write;

/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

#[derive(FromStr, Eq, Hash, PartialEq, Clone, Copy, Default, Debug)]
#[display("{x},{y}")]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn disatance(&self, other: &Self) -> isize {
        let x = other.x - self.x;
        let y = other.y - self.y;
        ((x * x + y * y) as f64).sqrt().floor() as isize
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub struct AsciiMatrix {
    cols: usize,
    rows: usize,
    data: Vec<u8>,
}

type Neighbor = (u8, Coord);

impl AsciiMatrix {
    pub fn new(cols: usize, rows: usize, init: u8) -> AsciiMatrix {
        AsciiMatrix {
            cols,
            rows,
            data: vec![init; cols * rows],
        }
    }

    #[inline]
    fn get_idx(&self, x: usize, y: usize) -> usize {
        self.cols * y + x
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        let idx = self.get_idx(x, y);
        self.data[idx] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&u8> {
        let idx = self.get_idx(x, y);
        if idx >= self.len() {
            None
        } else {
            Some(&self.data[idx])
        }
    }

    pub fn len(&self) -> usize {
        self.cols * self.rows
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_neighbors(&self, offsets: &[Coord], target: &Coord) -> Vec<Neighbor> {
        offsets
            .iter()
            .map(|offset| offset.add(target))
            .filter_map(|coord| {
                if coord.x < 0 || coord.y < 0 {
                    None
                } else {
                    self.get(coord.x as usize, coord.y as usize)
                        .map(|val| (*val, coord) as Neighbor)
                }
            })
            .collect_vec()
    }

    pub fn get_direct_neighbors(&self, target: &Coord) -> Vec<Neighbor> {
        #[rustfmt::skip]
        let offsets = [
                                   Coord::new(0, 1),
            Coord::new(1, 0),                        Coord::new(-1, 0),
                                   Coord::new(0, -1),
        ];
        self.get_neighbors(&offsets, target)
    }

    pub fn get_all_neighbors(&self, target: &Coord) -> Vec<Neighbor> {
        #[rustfmt::skip]
        let offsets = [
            Coord::new(1,  1), Coord::new(0,  1), Coord::new(-1,  1),
            Coord::new(1,  0),                         Coord::new(-1,  0),
            Coord::new(1, -1), Coord::new(0, -1), Coord::new(-1, -1),
        ];
        self.get_neighbors(&offsets, target)
    }

    pub fn file_debug(&self, file_name: &str, sep: &str) -> std::io::Result<()> {
        let output = self
            .data
            .chunks(self.cols)
            .map(|row| {
                row.iter()
                    .map(|d| format!("{}", *d as char))
                    .collect::<Vec<String>>()
                    .join(sep)
            })
            .collect::<Vec<String>>()
            .join("\n");

        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("/workspaces/advent-of-code-rust/{}", file_name))?;
        f.write_all(output.as_bytes())?;
        f.flush()
    }
}

impl Display for AsciiMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .data
            .chunks(self.cols)
            .map(|row| format!("{:?}", row))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", string)
    }
}
