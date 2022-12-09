use std::collections::HashSet;

use parse_display::{Display, FromStr};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
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
    fn is_near(&self, other: &Self) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
    fn get_closest(&self, other: &Self) -> Self {
        #[rustfmt::skip]
        let resutl = [
            Coord::new(1, 1), Coord::new(0, 1),Coord::new(-1, 1),
            Coord::new(1, 0),                       Coord::new(-1, 0),
            Coord::new(1, -1), Coord::new(0, -1),Coord::new(-1, -1),
        ].iter().fold((isize::MAX, Coord::new(0, 0) ), |(min, min_coord), offset|{
            let next_coord = self.add(offset);
            let next_distance = next_coord.distance(other) as isize;
            if next_distance < min {
                (next_distance, next_coord)
            } else {
                (min, min_coord)
            }
        });
        resutl.1
    }
    fn distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}


#[derive(Display, FromStr, PartialEq, Debug)]
enum Instructions {
    #[display("L {0}")]
    Left(isize),
    #[display("R {0}")]
    Right(isize),
    #[display("U {0}")]
    Up(isize),
    #[display("D {0}")]
    Down(isize),
}

impl Instructions {
    fn get_distance(&self) -> isize {
        match self {
            Instructions::Left(distance) => *distance,
            Instructions::Right(distance) => *distance,
            Instructions::Up(distance) => *distance,
            Instructions::Down(distance) => *distance,
        }
    }
}

fn _debug_separator(int: Instructions) {
    match int {
        Instructions::Left(d) => println!("=== Left {} ===", d),
        Instructions::Right(d) => println!("=== Right {} ===", d),
        Instructions::Up(d) => println!("=== Up {} ===", d),
        Instructions::Down(d) => println!("=== Down {} ===", d),
    };
}

fn _debug(head: &Coord, tail:&Coord, w: isize, h: isize) {
    for y in (0..h).rev() {
        for x in 0..w {
            let c = Coord::new(x, y);
            if head == &c {
                print!("H");
            } else if tail == &c {
                print!("T");
            } else {
                print!(".");
            }
        }
        println!();
    }
    print!("\n\n");
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut tail_coors: HashSet<Coord> = HashSet::new();
    let mut h_pos = Coord::new(0, 0);
    let mut t_pos = Coord::new(0, 0);

    tail_coors.insert(t_pos);

    input
        .trim()
        .lines()
        .map(|line| line.parse::<Instructions>())
        .for_each(|int| {
            let int = int.unwrap();
            for _ in 1..=int.get_distance() {
                h_pos = match int {
                    Instructions::Left(_) => Coord::new(h_pos.x - 1, h_pos.y),
                    Instructions::Right(_) => Coord::new(h_pos.x + 1, h_pos.y),
                    Instructions::Up(_) => Coord::new(h_pos.x, h_pos.y + 1),
                    Instructions::Down(_) => Coord::new(h_pos.x, h_pos.y - 1),
                };
                // debug(&h_pos, &t_pos, 6, 5);
                if !h_pos.is_near(&t_pos) {
                    t_pos = t_pos.get_closest(&h_pos);
                }
                tail_coors.insert(t_pos);
                // debug(&h_pos, &t_pos, 6, 5);
            }
        });
    Some(tail_coors.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut last_segment_coord: HashSet<Coord> = HashSet::new();
    let mut h_pos = Coord::new(0, 0);
    let mut tail = vec![Coord::new(0, 0); 9];

    last_segment_coord.insert(h_pos); // ass start position

    input
        .trim()
        .lines()
        .map(|line| line.parse::<Instructions>())
        .for_each(|int| {
            let int = int.unwrap();
            for _ in 1..=int.get_distance() {
                h_pos = match int {
                    Instructions::Left(_) => Coord::new(h_pos.x - 1, h_pos.y),
                    Instructions::Right(_) => Coord::new(h_pos.x + 1, h_pos.y),
                    Instructions::Up(_) => Coord::new(h_pos.x, h_pos.y + 1),
                    Instructions::Down(_) => Coord::new(h_pos.x, h_pos.y - 1),
                };

                let mut last_segment = h_pos;
                tail = tail.iter().map(|t_pos| {
                    let next_t_pos = if !last_segment.is_near(t_pos) {
                        t_pos.get_closest(&last_segment)
                    } else {
                        *t_pos
                    };
                    last_segment = next_t_pos;
                    next_t_pos
                }).collect();
                // debug(&h_pos, &t_pos, 6, 5);
                
                last_segment_coord.insert(*tail.last().unwrap());
                // debug(&h_pos, &t_pos, 6, 5);
            }
        });
    Some(last_segment_coord.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_by_name("examples", "09b.txt");
        assert_eq!(part_two(&input), Some(36));
    }
}
