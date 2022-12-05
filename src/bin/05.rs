use regex::Regex;

pub fn part_one(input: &str) -> Option<String> {
    let col_width = 4;
    let (stack_input, moves_input) = input.split_once("\n\n").unwrap();
    let num_of_stacks = (input.lines().next()?.len() + 1) / col_width;
    let mut stacks: Vec<Vec<u8>> = vec![vec![]; num_of_stacks];

    stack_input.lines().rev().skip(1).for_each(|line| {
        let row = line.as_bytes();
        let mut col_index = 1; // first crate letter after '['
        for col_num in 0..num_of_stacks {
            if !row[col_index].is_ascii_whitespace() {
                stacks[col_num].push(row[col_index])
            }
            col_index += col_width;
        }
    });

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    moves_input.lines().for_each(|line| {
        let nums = re
            .captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|n| n.map_or(0, |n| n.as_str().parse::<usize>().unwrap()))
            .collect::<Vec<usize>>();
        let (count, from, to) = (nums[0], nums[1] - 1, nums[2] - 1);

        for _c in 0..count {
            let val = stacks[from].pop().unwrap();
            stacks[to].push(val)
        }
    });

    let result = stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap_or(&b' '))
        .collect::<Vec<_>>();

    Some(std::str::from_utf8(&result[..]).unwrap().to_string())
}

pub fn part_two(input: &str) -> Option<String> {
    let col_width = 4;
    let (stack_input, moves_input) = input.split_once("\n\n").unwrap();
    let num_of_stacks = (input.lines().next()?.len() + 1) / col_width;
    let mut stacks: Vec<Vec<u8>> = vec![vec![]; num_of_stacks];

    stack_input.lines().rev().skip(1).for_each(|line| {
        let row = line.as_bytes();
        let mut col_index = 1; // first crate letter after '['
        for col_num in 0..num_of_stacks {
            if !row[col_index].is_ascii_whitespace() {
                stacks[col_num].push(row[col_index])
            }
            col_index += col_width;
        }
    });

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    moves_input.lines().for_each(|line| {
        let nums = re
            .captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|n| n.map_or(0, |n| n.as_str().parse::<usize>().unwrap()))
            .collect::<Vec<usize>>();
        let (count, from, to) = (nums[0], nums[1] - 1, nums[2] - 1);

        let at = stacks[from].len() - count;
        let mut values = stacks[from].split_off(at);
        stacks[to].append(&mut values);
    });

    let result = stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap_or(&b' '))
        .collect::<Vec<_>>();

    Some(std::str::from_utf8(&result[..]).unwrap().to_string())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
