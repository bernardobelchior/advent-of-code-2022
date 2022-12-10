type Stack<T> = Vec<T>;

struct Move {
    count: u8,
    from: usize,
    to: usize,
}

fn parse_crates(mut lines: Vec<&str>) -> Vec<Stack<char>> {
    let num_stacks = lines.pop().unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();

    let mut stacks: Vec<Stack<char>> = vec![Stack::new(); num_stacks];

    lines.reverse();
    for crates in lines.iter() {
        for i in 0..num_stacks {
            let crate_char = crates.chars().nth(1 + i * 4);

            match crate_char {
                None => break,
                Some(crate_char) => {
                    if !crate_char.is_whitespace() {
                        stacks[i].push(crate_char);
                    }
                }
            }
        }
    }

    stacks
}

fn parse_movements(lines: Vec<&str>) -> Vec<Move> {
    lines.iter().map(|line| {
        let mut split = line.split_whitespace();

        let count = split.nth(1).unwrap().parse::<u8>().unwrap();
        let from = split.nth(1).unwrap().parse::<usize>().unwrap();
        let to = split.nth(1).unwrap().parse::<usize>().unwrap();

        Move { from, to, count }
    }).collect()
}

fn get_top_crates_part1(lines: std::str::Lines) -> String {
    let crates = lines.clone().take_while(|s| !s.is_empty()).collect::<Vec<&str>>();

    let mut crates = parse_crates(crates);
    let movements = parse_movements(lines.skip_while(|s| !s.is_empty()).skip(1).collect());

    for movement in movements {
        for _ in 0..movement.count {
            let krate = crates[movement.from - 1].pop().unwrap();
            crates[movement.to - 1].push(krate);
        }
    }

    crates.iter().fold("".to_owned(), |mut acc, stack| {
        acc.push(*stack.last().unwrap());
        acc
    })
}

fn get_top_crates_part2(lines: std::str::Lines) -> String {
    let crates = lines.clone().take_while(|s| !s.is_empty()).collect::<Vec<&str>>();

    let mut crates = parse_crates(crates);
    let movements = parse_movements(lines.skip_while(|s| !s.is_empty()).skip(1).collect());

    for movement in movements {
        let mut moved_crates = vec![];

        for _ in 0..movement.count {
            moved_crates.insert(0, crates[movement.from - 1].pop().unwrap());
        }

        crates[movement.to - 1].append(&mut moved_crates);
    }

    crates.iter().fold("".to_owned(), |mut acc, stack| {
        acc.push(*stack.last().unwrap());
        acc
    })
}

fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();

    println!("{}", get_top_crates_part2(file.lines()));
}

#[cfg(test)]
mod tests {
    use crate::{get_top_crates_part2, get_top_crates_part1};

    const INPUT: &'static str =
        r"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        assert_eq!(get_top_crates_part1(INPUT.lines()), "CMZ");
    }


    #[test]
    fn part2_works() {
        assert_eq!(get_top_crates_part2(INPUT.lines()), "MCD");
    }
}
