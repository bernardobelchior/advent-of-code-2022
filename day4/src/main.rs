use std::ops::{RangeInclusive};

fn nth_string_to_number(line: &str, n: usize) -> u32 {
    line.split("-").nth(n).unwrap().parse::<u32>().unwrap()
}

fn parse_line(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let split: Vec<&str> = line.split(",").take(2).collect();
    let first_elf = split[0];
    let second_elf = split[1];

    (
        RangeInclusive::new(nth_string_to_number(first_elf, 0), nth_string_to_number(first_elf, 1)),
        RangeInclusive::new(nth_string_to_number(second_elf, 0), nth_string_to_number(second_elf, 1)),
    )
}

fn get_score_part1(lines: std::str::Lines) -> u64 {
    let mut score = 0u64;

    for line in lines {
        let (first_elf, second_elf) = parse_line(line);

        if first_elf.contains(&second_elf.start()) && first_elf.contains(&second_elf.end()) {
            score += 1;
        } else if second_elf.contains(&first_elf.start()) && second_elf.contains(&first_elf.end()) {
            score += 1;
        }
    }

    score
}

fn get_score_part2(lines: std::str::Lines) -> u64 {
    let mut score = 0u64;

    for line in lines {
        let (first_elf, second_elf) = parse_line(line);

        if first_elf.contains(&second_elf.start()) || first_elf.contains(&second_elf.end()) {
            score += 1;
        } else if second_elf.contains(&first_elf.start()) || second_elf.contains(&first_elf.end()) {
            score += 1;
        }
    }

    score
}

fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();

    println!("{}", get_score_part2(file.lines()));
}

#[cfg(test)]
mod tests {
    use crate::{get_score_part2, get_score_part1};

    const INPUT: &'static str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        assert_eq!(get_score_part1(INPUT.lines()), 2);
    }


    #[test]
    fn part2_works() {
        assert_eq!(get_score_part2(INPUT.lines()), 4);
    }
}
