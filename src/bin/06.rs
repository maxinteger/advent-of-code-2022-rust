use std::collections::HashSet;

fn solution(input: &str, window_size: usize) -> usize {
    input
        .trim()
        .as_bytes()
        .windows(window_size)
        .take_while(|w| w.iter().collect::<HashSet<_>>().len() != window_size)
        .count()
        + window_size
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solution(input, 4) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solution(input, 14) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let samples = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for (input, result) in samples {
            assert_eq!(part_one(input), Some(result));
        }
    }

    #[test]
    fn test_part_two() {
        let samples = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        for (input, result) in samples {
            assert_eq!(part_two(input), Some(result));
        }
    }
}
