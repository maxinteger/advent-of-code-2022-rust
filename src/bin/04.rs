fn range_fully_contains(a1: u32, b1: u32, a2: u32, b2:u32) -> bool {
    // range 1 contains range 2
    a2 >= a1 && b2 <= b1 ||
    // range 2 contains range 1
    a1 >= a2 && b1 <= b2
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split('\n')
            .filter(|line| {
                let (range1, range2) = line.split_once(',').unwrap();
                let range1 = range1.split('-').map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>();
                let range2 = range2.split('-').map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>();

                range_fully_contains(range1[0], range1[1], range2[0], range2[1])
            })
            .count() as u32,
    )
}

fn range_overlaps(a1: u32, b1: u32, a2: u32, b2:u32) -> bool {
    a1 >= a2 && a1 <= b2 ||
    b1 <= b2 && b1 >= a2 ||
    a2 >= a1 && a2 <= b1 ||
    b2 <= b1 && b2 >= a1
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split('\n')
            .filter(|line| {
                let (range1, range2) = line.split_once(',').unwrap();
                let range1 = range1.split('-').map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>();
                let range2 = range2.split('-').map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>();

                range_overlaps(range1[0], range1[1], range2[0], range2[1])
            })
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
