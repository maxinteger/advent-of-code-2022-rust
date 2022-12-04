pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (lo, ro) = line.split_once(' ').unwrap();
                let lo = (lo.as_bytes()[0] as isize) - ('A' as isize);
                let ro = (ro.as_bytes()[0] as isize) - ('X' as isize);
                let result = match (lo - (ro + 3)).abs() % 3 {
                    0 => 3, // draw
                    1 => 6, // right opponent win
                    2 => 0, // left opponent win
                    _ => panic!(),
                };
                (result + ro + 1) as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (lo, outcome) = line.split_once(' ').unwrap();
                let lo = (lo.as_bytes()[0] as isize) - ('A' as isize);
                let outcome = (outcome.as_bytes()[0] as isize) - ('X' as isize);

                let (result, ro) = match outcome {
                    0 => (0, (lo + 2) % 3), // left opponent win
                    1 => (3, lo),           // draw
                    2 => (6, (lo + 1) % 3), // right opponent win
                    _ => panic!(),
                };
                (result + ro + 1) as u32
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
