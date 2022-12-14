use serde_json::Value;
use std::cmp::Ordering;

pub fn part_one(input: &str) -> Option<u32> {
    let num_of_right_order = input
        .trim()
        .split("\n\n")
        .enumerate()
        .map(|(idx, entry)| {
            let (left, right) = entry.split_once('\n').unwrap();
            let left: Value = serde_json::from_str(left).unwrap();
            let right: Value = serde_json::from_str(right).unwrap();

            fn cmp(data: (&Value, &Value)) -> Ordering {
                match data {
                    (Value::Number(left), Value::Number(right)) => {
                        let left = left.as_u64().unwrap();
                        let right = right.as_u64().unwrap();
                        left.cmp(&right)
                    }
                    (Value::Array(left), Value::Array(right)) => {
                        for i in 0..left.len().min(right.len()) {
                            let r = cmp((&left[i], &right[i]));
                            if r != Ordering::Equal {
                                return r;
                            }
                        }
                        left.len().cmp(&right.len())
                    }
                    (Value::Array(_), Value::Number(_)) => {
                        cmp((data.0, &Value::Array(vec![data.1.clone()])))
                    }
                    (Value::Number(_), Value::Array(_)) => {
                        cmp((&Value::Array(vec![data.0.clone()]), data.0))
                    }
                    _ => Ordering::Equal,
                }
            }

            if cmp((&left, &right)) == Ordering::Less {
                println!("{}", idx + 1);
                println!("{}\n",entry);
                idx + 1
            } else {
                0
            }
        })
        .sum::<usize>();

    Some(num_of_right_order as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
