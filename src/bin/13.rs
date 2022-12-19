use itertools::Itertools;
use serde_json::Value;
use std::cmp::Ordering;

fn packet_cmp(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            let left = left.as_u64().unwrap();
            let right = right.as_u64().unwrap();
            left.cmp(&right)
        }
        (Value::Array(left), Value::Array(right)) => {
            for i in 0..left.len().min(right.len()) {
                let r = packet_cmp(&left[i], &right[i]);
                if r != Ordering::Equal {
                    return r;
                }
            }
            left.len().cmp(&right.len())
        }
        (Value::Array(_), Value::Number(_)) => packet_cmp(left, &Value::Array(vec![right.clone()])),
        (Value::Number(_), Value::Array(_)) => packet_cmp(&Value::Array(vec![left.clone()]), right),
        _ => Ordering::Equal,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let num_of_right_order = input
        .trim()
        .split("\n\n")
        .enumerate()
        .map(|(idx, entry)| {
            let (left, right) = entry.split_once('\n').unwrap();
            let left: Value = serde_json::from_str(left).unwrap();
            let right: Value = serde_json::from_str(right).unwrap();

            if packet_cmp(&left, &right) == Ordering::Less {
                idx + 1
            } else {
                0
            }
        })
        .sum::<usize>();

    Some(num_of_right_order as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sep_1 = "[[2]]";
    let sep_2 = "[[6]]";
    let packets = vec![sep_1, sep_2, input]
        .join("\n")
        .trim()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .sorted_by(packet_cmp)
        .collect::<Vec<_>>();

    let sep_1 = serde_json::from_str::<Value>(sep_1).unwrap();
    let sep_2 = serde_json::from_str::<Value>(sep_2).unwrap();

    let sep_1_idx = packets
        .iter()
        .position(|item| packet_cmp(item, &sep_1) == Ordering::Equal)
        .unwrap()
        + 1;
    let sep_2_idx = packets
        .iter()
        .position(|item| packet_cmp(item, &sep_2) == Ordering::Equal)
        .unwrap()
        + 1;

    // debug
    // packets.for_each(|l| {println!("{}", serde_json::to_string(&l).unwrap())});

    Some((sep_1_idx * sep_2_idx) as u32)
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
        assert_eq!(part_two(&input), Some(140));
    }
}
