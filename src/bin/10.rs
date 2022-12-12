use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
enum Instructions {
    #[display("noop")]
    Noop,
    #[display("addx {0}")]
    AddX(isize),
}
pub fn part_one(input: &str) -> Option<u32> {
    let signal_cycles: [isize; 6] = [20, 60, 100, 140, 180, 220];
    Some(
        input
            .trim()
            .lines()
            .map(|line| line.parse::<Instructions>().unwrap())
            .fold(vec![] as Vec<isize>, |mut acc, instriction| {
                acc.push(0);
                if let Instructions::AddX(val) = instriction {
                    acc.push(val)
                }
                acc
            })
            .iter()
            .enumerate()
            .fold(
                (1, vec![] as Vec<isize>),
                |(mut x, mut signals), (cycle, add_x)| {
                    let cycle = cycle as isize + 1;
                    if signal_cycles.contains(&cycle) {
                        signals.push(cycle * x)
                    }

                    x += *add_x;
                    (x, signals)
                },
            )
            .1
            .iter()
            .sum::<isize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let screen = input
        .trim()
        .lines()
        .map(|line| line.parse::<Instructions>().unwrap())
        .fold(vec![] as Vec<isize>, |mut acc, instriction| {
            acc.push(0);
            if let Instructions::AddX(val) = instriction {
                acc.push(val)
            }
            acc
        })
        .iter()
        .enumerate()
        .fold(
            (1, vec![] as Vec<String>),
            |(mut x, mut crt), (cycle, add_x)| {
                let cycle = cycle as isize;
                let h_coord = cycle % 40;

                if h_coord == 0 {
                    crt.push("\n".to_string());
                }
                let pixel = if (x - 1) <= h_coord && (x + 1) >= h_coord {
                    "#"
                } else {
                    "."
                };
                crt.push(pixel.to_string());

                x += *add_x;
                (x, crt)
            },
        )
        .1
        .join("");
    println!("{}", screen);
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
