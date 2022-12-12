use parse_display::FromStr;
use std::collections::VecDeque;

#[derive(FromStr, Debug, PartialEq)]
#[display(
    r"Monkey {monkey_index}:
  Starting items: {items}
  Operation: new = old {op}
  Test: divisible by {divider}
    If true: throw to monkey {monkey_index_a}
    If false: throw to monkey {monkey_index_b}"
)]
struct PreMonkey {
    monkey_index: usize,
    items: String,
    op: String,
    divider: isize,
    monkey_index_a: usize,
    monkey_index_b: usize,
}

#[derive(FromStr, Debug, PartialEq, Clone)]
enum Op {
    #[display("+ old")]
    AddOld,
    #[display("+ {0}")]
    AddNum(isize),
    #[display("* old")]
    MultipleOld,
    #[display("* {0}")]
    Multiple(isize),
}

#[derive(Clone, Debug)]
struct Monkey {
    monkey_index: usize,
    items: VecDeque<isize>,
    op: Op,
    divider: isize,
    monkey_index_a: usize,
    monkey_index_b: usize,
    inspected: usize,
}

impl Monkey {
    fn new(prep: PreMonkey) -> Self {
        let op = prep.op.parse::<Op>().unwrap();
        let items = prep
            .items
            .split(',')
            .map(|i| i.trim().parse::<isize>().unwrap())
            .collect::<VecDeque<_>>();
        Monkey {
            monkey_index: prep.monkey_index,
            items,
            op,
            divider: prep.divider,
            monkey_index_a: prep.monkey_index_a,
            monkey_index_b: prep.monkey_index_b,
            inspected: 0,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut monkeys = input
        .trim()
        .split("\n\n")
        .map(|l| Monkey::new(l.trim().parse::<PreMonkey>().unwrap()))
        .collect::<Vec<_>>();
    
    let rounds = 0..20;
    rounds.for_each(|r| {
        for m in 0..monkeys.len() {
        
            while let Some(worry_level) = monkeys[m].items.pop_front() {
                let worry_level = match monkeys[m].op {
                    Op::AddOld => worry_level * 2,
                    Op::AddNum(val) => worry_level + val,
                    Op::MultipleOld => worry_level * worry_level,
                    Op::Multiple(val) => worry_level * val,
                };
                monkeys[m].inspected += 1;
                let worry_level = (worry_level as f64 / 3f64).floor() as isize;

                if worry_level % monkeys[m].divider == 0 {
                    let idx = monkeys[m].monkey_index_a;
                    monkeys[idx].items.push_back(worry_level);
                } else {
                    let idx = monkeys[m].monkey_index_b;
                    monkeys[idx].items.push_back(worry_level);
                }
            }
        };
    });

    monkeys.sort_by(|a, b| {a.inspected.partial_cmp(&b.inspected).unwrap()});
    Some(monkeys.iter().rev().take(2).map(|m|{m.inspected}).product::<usize>() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut monkeys = input
        .trim()
        .split("\n\n")
        .map(|l| Monkey::new(l.trim().parse::<PreMonkey>().unwrap()))
        .collect::<Vec<_>>();
    
    let rounds = 0..10_000;
    rounds.for_each(|r| {
        for m in 0..monkeys.len() {
        
            while let Some(worry_level) = monkeys[m].items.pop_front() {
                let worry_level = match monkeys[m].op {
                    Op::AddOld => worry_level * 2,
                    Op::AddNum(val) => worry_level + val,
                    Op::MultipleOld => worry_level * worry_level,
                    Op::Multiple(val) => worry_level * val,
                };
                monkeys[m].inspected += 1;
                let worry_level = (worry_level as f64 / std::f64::consts::PI).floor() as isize;

                if worry_level % monkeys[m].divider == 0 {
                    let idx = monkeys[m].monkey_index_a;
                    monkeys[idx].items.push_back(worry_level);
                } else {
                    let idx = monkeys[m].monkey_index_b;
                    monkeys[idx].items.push_back(worry_level);
                }
            }
        };
    });

    monkeys.sort_by(|a, b| {a.inspected.partial_cmp(&b.inspected).unwrap()});
    Some(monkeys.iter().rev().take(2).map(|m|{m.inspected}).product::<usize>() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
