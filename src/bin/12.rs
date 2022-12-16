use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::fs;

use itertools::Itertools;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Default, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn disatance(&self, other: &Self) -> isize {
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

fn render_debug_map(total_path: &VecDeque<Coord>, f_score: &HashMap<Coord, usize>, map: &Vec<&[u8]>) {
    let mut debug: Vec<Vec<String>> = vec![];
    for y in 0..map.len() {
        debug.push(vec![]);
        for x in 0..map[y].len() {
            let coord = Coord::new(x as isize, y as isize);
            if total_path.contains(&coord) {
                debug[y].push("_".to_string())
            } else if f_score.contains_key(&coord) {
                debug[y].push(".".to_string())
            } else {
                let s = std::str::from_utf8(&[map[y][x]]).unwrap().to_string();
                debug[y].push(s)
            }
        }
    }
    let out = debug
        .iter()
        .map(|line| line.join(""))
        .collect::<Vec<_>>()
        .join("\n");

    fs::write("/workspaces/advent-of-code-rust/foo.txt", out).unwrap();
}

fn a_star(start: &Coord, goal: &Coord, map: &Vec<&[u8]>) -> Option<VecDeque<Coord>> {
    let infinity = usize::MAX / 2;
    let mut open_set: HashSet<Coord> = HashSet::new();
    open_set.insert(*start);
    let mut g_score: HashMap<Coord, usize> = HashMap::new();
    g_score.insert(*start, 0);
    let mut f_score: HashMap<Coord, usize> = HashMap::new();
    f_score.insert(*start, 0);
    let mut come_from: HashMap<Coord, Coord> = HashMap::new();

    fn d(current: &Coord, neighbor: &Coord, map: &Vec<&[u8]>) -> usize {
        let infinity = usize::MAX / 2;
        let v_range = 0..map.len();
        let h_range = 0..map[0].len();

        if h_range.contains(&(current.x as usize))
            && h_range.contains(&(neighbor.x as usize))
            && v_range.contains(&(current.y as usize))
            && v_range.contains(&(neighbor.y as usize))
        {
            let curr = map[current.y as usize][current.x as usize];
            let next = map[neighbor.y as usize][neighbor.x as usize];

            if curr == b'z' && next == b'E' {
                0
            } else if (curr == b'S' || curr >= next || curr + 1 == next) && next != b'E' {
                1
            } else {
                infinity
            }
        } else {
            infinity
        }
    }

    while !open_set.is_empty() {
        let current = open_set
            .iter()
            .min_by_key(|coord| f_score.get(coord).unwrap_or(&infinity))
            .unwrap()
            .clone();
        if current == *goal {
            let mut cur = current;
            let mut total_path: VecDeque<Coord> = VecDeque::new();
            total_path.push_front(cur);
            while come_from.keys().contains(&cur) {
                cur = come_from[&cur];
                total_path.push_front(cur)
            }
            render_debug_map(&total_path, &f_score, map);
            return Some(total_path);
        } else {
            open_set.remove(&current);
            for offset in [
                Coord::new(-1, 0),
                Coord::new(1, 0),
                Coord::new(0, -1),
                Coord::new(0, 1),
            ] {
                let neighbor = current.add(&offset);
                let tentative_g_score = g_score.get(&current).unwrap_or(&infinity)
                    + d(&current, &neighbor, map);

                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&infinity) {
                    come_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(
                        neighbor,
                        tentative_g_score + neighbor.disatance(goal) as usize,
                    );
                    open_set.insert(neighbor);
                }
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut start = Coord::default();
    let mut goal = Coord::default();

    let map = input
        .trim()
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            let row = line.as_bytes();
            if let Some(col) = row.iter().position(|c| *c == b'E') {
                goal = Coord::new(col as isize, row_idx as isize);
            };
            if let Some(col) = row.iter().position(|c| *c == b'S') {
                start = Coord::new(col as isize, row_idx as isize);
            };
            row
        })
        .collect::<Vec<_>>();

    Some((a_star(&start, &goal, &map).unwrap().len() - 1) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut starts: Vec<Coord> = vec![];
    let mut goal = Coord::default();

    let map = input
        .trim()
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            let row = line.as_bytes();
            row.iter().enumerate().for_each(|(col_idx, ch)| {
                if *ch == b'S' || *ch == b'a' {
                    starts.push(Coord::new(col_idx as isize, row_idx as isize));
                }
                else if *ch == b'E' {
                    goal = Coord::new(col_idx as isize, row_idx as isize);
                }
            });
            row
        })
        .collect::<Vec<_>>();

    starts.iter().map(|start| { 
        if let Some(res) = a_star(start, &goal, &map) {
            (res.len() - 1) as u32
        } else {
            u32::MAX
        }
     }).min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
