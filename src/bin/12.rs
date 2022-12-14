use std::collections::{HashMap, HashSet, VecDeque};

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
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut paths: Vec<HashSet<Coord>> = vec![];
    let start = Coord::default();
    let mut goal = Coord::default();

    paths.push(HashSet::new());
    paths[0].insert(start);
    let map = input
        .trim()
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            let row = line.as_bytes();
            if let Some(col) = row.iter().position(|c| *c == b'E') {
                goal = Coord::new(col as isize, row_idx as isize);
            };
            row
        })
        .collect::<Vec<_>>();

    fn a_star(start: Coord, goal: Coord, map: Vec<&[u8]>) -> Option<VecDeque<Coord>> {
        let infinity = usize::MAX / 2;
        let mut open_set: HashSet<Coord> = HashSet::new();
        open_set.insert(start);
        let mut g_score: HashMap<Coord, usize> = HashMap::new();
        g_score.insert(start, 0);
        let mut f_score: HashMap<Coord, usize> = HashMap::new();
        f_score.insert(start, 0);
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
                let a = map[neighbor.y as usize][neighbor.x as usize];
                let b = map[current.y as usize][current.x as usize];
                if (a == b'E' && b == b'z') || a == b || a == b + 1 {
                    1
                } else {
                    infinity
                }
            } else {
                infinity
            }
        }

        while open_set.len() > 0 {
            let current = open_set
                .iter()
                .max_by_key(|coord| f_score.get(coord).unwrap_or(&infinity))
                .unwrap()
                .clone();
            if current == goal {
                let mut cur = current;
                let mut total_path: VecDeque<Coord> = VecDeque::new();
                total_path.push_front(cur);
                while come_from.keys().contains(&cur) {
                    cur = come_from[&cur];
                    total_path.push_front(cur)
                }
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
                    let tentative_g_score =
                        g_score.get(&current).unwrap_or(&infinity) + d(&current, &neighbor, &map);

                    if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&infinity) {
                        come_from.insert(neighbor, current);
                        g_score.insert(neighbor, tentative_g_score);
                        f_score.insert(neighbor, tentative_g_score + 1);
                        if !open_set.contains(&neighbor) {
                            open_set.insert(neighbor);
                        }
                    }
                }
            }
        }
        None
    }

    Some((a_star(start, goal, map).unwrap().len() - 1) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
