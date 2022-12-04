use itertools::Itertools;

// To help prioritize item rearrangement, every item type can be converted to a priority:
// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
fn to_priority(c: u8) -> u8 {
    if c > b'Z' {
        c - b'a' + 1
    } else {
        c - b'A' + 27
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (a, b) = line.split_at(line.len() / 2);
                let (a, b) = (a.as_bytes(), b.as_bytes());
                a.into_iter()
                    .unique()
                    .filter(|ac| b.contains(ac))
                    .map(|c| to_priority(*c) as u32)
                    .sum::<u32>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split('\n')
            .tuples::<(_, _, _)>()
            .map(|(a, b, c)| {
                let (a, b,c) = (a.as_bytes(), b.as_bytes(), c.as_bytes());
                a.into_iter()
                    .unique()
                    .filter(|ac| b.contains(ac) && c.contains(ac))
                    .map(|c| to_priority(*c) as u32)
                    .sum::<u32>()
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
