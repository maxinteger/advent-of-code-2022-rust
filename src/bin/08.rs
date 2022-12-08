use itertools::Itertools;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.trim().lines();
    let size = lines.clone().count();
    let matrix = lines
        .map(|l| l.as_bytes().iter().map(|ch| ch - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visible_threes: HashSet<(usize, usize)> = HashSet::new();

    for a in 1..size - 1 {
        for b in 1..size - 1 {
            let rev_b = size - b - 1;
            // left
            if (0..b).all(|i| matrix[a][i] < matrix[a][b]) {
                visible_threes.insert((a, b));
            };
            // right
            if (rev_b + 1..size).all(|i| matrix[a][i] < matrix[a][rev_b]) {
                visible_threes.insert((a, rev_b));
            };
            // top
            if (0..b).all(|i| matrix[i][a] < matrix[b][a]) {
                visible_threes.insert((b, a));
            };
            // bootm
            if (rev_b + 1..size).all(|i| matrix[i][a] < matrix[rev_b][a]) {
                visible_threes.insert((rev_b, a));
            };
        }
    }

    Some((visible_threes.len() + (size * 4 - 4)) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.trim().lines();
    let size = lines.clone().count();
    let matrix = lines
        .map(|l| l.as_bytes().iter().map(|ch| ch - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut max_scenic_score = 0;

    for a in 0..size {
        for b in 0..size {
            let a_plus_1 = a + 1;
            let b_plus_1 = b + 1;

            //Vec<(core::slice::Iter<usize>, Box<dyn Fn((&&usize)) -> bool>)>
            let mut left = (0..b).rev();
            let scenic_left = left
                .take_while_ref(|i| matrix[a][*i] < matrix[a][b])
                .count();
            let scenic_left = scenic_left + (scenic_left == 0 || left.next().is_some()) as usize;

            let mut right = b_plus_1..size;
            let scenic_right = right
                .take_while_ref(|i| matrix[a][*i] < matrix[a][b])
                .count();
            let scenic_right =
                scenic_right + (scenic_right == 0 || right.next().is_some()) as usize;

            let mut up = (0..a).rev();
            let scenic_up = up.take_while_ref(|i| matrix[*i][b] < matrix[a][b]).count();
            let scenic_up = scenic_up + (scenic_up == 0 || up.next().is_some()) as usize;

            let mut down = a_plus_1..size;
            let scenic_down = down
                .take_while_ref(|i| matrix[*i][b] < matrix[a][b])
                .count();
            let scenic_down = scenic_down + (scenic_down == 0 || down.next().is_some()) as usize;

            let scenic_score = scenic_left * scenic_right * scenic_up * scenic_down;

            println!("({},{}) {}", a, b, scenic_score);

            if max_scenic_score < scenic_score {
                max_scenic_score = scenic_score
            }
        }
    }

    Some(max_scenic_score as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(16));
    }
}
