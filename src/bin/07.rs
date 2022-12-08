use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
enum Inputs {
    #[display("$ cd /")]
    CmdCdRoot(),
    #[display("$ cd ..")]
    CmdCdUp(),
    #[display("$ cd {0}")]
    CmdCd(String),
    #[display("$ ls")]
    CmdLs,
    #[display("dir {0}")]
    EntryDir(String),
    #[display("{0} {1}")]
    EntryFile(usize, String),
}

const ALL_SPACE: usize = 70_000_000;
const UPDATE_SIZE: usize = 30_000_000;
const SIZE_LIMIT: usize = 100_000;

pub fn part_one(input: &str) -> Option<u32> {
    let mut stack = vec![0];
    let mut resutls: Vec<usize> = vec![];
    input
        .trim()
        .lines()
        .map(|line| line.parse::<Inputs>())
        .for_each(|item| match item.unwrap() {
            Inputs::EntryFile(size, _) => stack.push(size),
            Inputs::CmdCd(_) => stack.push(0),
            Inputs::CmdCdUp() => {
                let mut parent_size = 0;
                while let Some(next) = stack.pop() {
                    parent_size += next;
                    if next == 0 {
                        break;
                    }
                }
                if parent_size <= SIZE_LIMIT {
                    resutls.push(parent_size)
                }
                stack.push(parent_size)
            }
            _ => (),
        });
    let root_size = stack.iter().sum();
    if root_size <= SIZE_LIMIT {
        resutls.push(root_size)
    }
    Some(resutls.iter().sum::<usize>() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut stack = vec![0];
    let mut dir_sizes: Vec<usize> = vec![];
    input
        .trim()
        .lines()
        .map(|line| line.parse::<Inputs>())
        .for_each(|item| match item.unwrap() {
            Inputs::EntryFile(size, _) => stack.push(size),
            Inputs::CmdCd(_) => stack.push(0),
            Inputs::CmdCdUp() => {
                let mut parent_size = 0;
                while let Some(next) = stack.pop() {
                    parent_size += next;
                    if next == 0 {
                        break;
                    }
                }
                dir_sizes.push(parent_size);
                stack.push(parent_size)
            }
            _ => (),
        });

    let mut parent_size = 0;
    while let Some(next) = stack.pop() {
        parent_size += next;
        if next == 0 {
            break;
        }
    }
    dir_sizes.push(parent_size);
    stack.push(parent_size);
    let root_size: usize = stack.iter().sum();
    dir_sizes.push(root_size);
    let available_space = ALL_SPACE - root_size;
    let space_needed = UPDATE_SIZE - available_space;
    dir_sizes.sort();
    let result = dir_sizes.iter().find(|s| **s > space_needed).unwrap();
    Some(*result as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
