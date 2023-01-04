use advent_of_code::helpers::AsciiMatrix;
use itertools::Itertools;
use advent_of_code::helpers::*;

fn parse(input: &str) -> (isize, AsciiMatrix) {
    
    let mut m: AsciiMatrix = AsciiMatrix::new(1000, 256, b' ');
    let mut max_y = 0;
    input.trim().lines().for_each(|line| {
        line.split(" -> ")
            .map(|coord| coord.parse::<Coord>().unwrap())
            .tuple_windows()
            .for_each(|(Coord { x: ax, y: ay }, Coord { x: bx, y: by })| {
                for x in ax.min(bx)..=ax.max(bx) {
                    for y in ay.min(by)..=ay.max(by) {
                        m.set(x as usize, y as usize, b'#');
                        max_y = max_y.max(y);
                    }
                }
            })
    });
    (max_y, m)
}

pub fn part_one(input: &str) -> Option<u32> {
    let ( max_y, mut m) = parse(input);

    let offsets = [Coord::new(0, 1), Coord::new(-1, 1), Coord::new(1, 1)];
    let mut count = 0;
    'main: loop {
        let mut sand = Coord::new(500, 0);
        loop {
            let new_sand = offsets.iter().find_map(|c| {
                let sand = c.add(&sand);
                let val = m.get(sand.x as usize, sand.y as usize);
                if val.is_some() && *val.unwrap() == b' '  {
                    Some(sand)
                } else {
                    None
                }
            });
            if let Some(s) = new_sand {
                if s.y > max_y {
                    break 'main;
                }
                sand = s
            } else {
                m.set(sand.x as usize, sand.y as usize, b'o');
                // m.file_debug("14-debug.txt", "");
                count += 1;
                break;
            }
        }
    }
    m.file_debug("14-debug.txt", "");
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let ( max_y, mut m) = parse(input);

    let offsets = [Coord::new(0, 1), Coord::new(-1, 1), Coord::new(1, 1)];
    let mut count = 0;
    'main: loop {
        let mut sand = Coord::new(500, 0);
        loop {
            let new_sand = offsets.iter().find_map(|c| {
                let sand = c.add(&sand);
                let val = m.get(sand.x as usize, sand.y as usize);
                if val.is_some() && *val.unwrap() == b' ' && sand.y < max_y + 2  {
                    Some(sand)
                } else {
                    None
                }
            });
            if let Some(s) = new_sand {
                sand = s
            } else {
                m.set(sand.x as usize, sand.y as usize, b'o');
                // m.file_debug("14-debug.txt", "");
                count += 1;
                if sand.y == 0 {
                    break 'main;
                }
                break;
            }
        }
    }
    m.file_debug("14-debug.txt", "");
    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
