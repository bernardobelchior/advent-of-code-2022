#![feature(iter_array_chunks)]

use std::collections::HashSet;

fn get_char_score(c: &char) -> u8 {
    return if c.is_ascii_lowercase() {
        1 + *c as u8 - 'a' as u8
    } else {
        27 + *c as u8 - 'A' as u8
    };
}

fn get_score_part1(lines: std::str::Lines) -> u64 {
    let mut score = 0u64;

    for line in lines {
        let len = line.len();
        let half_len = len / 2;

        let first_chars: HashSet<char> = HashSet::from_iter(line.chars().take(half_len));
        let second_chars: HashSet<char> = HashSet::from_iter(line.chars().skip(half_len));

        let mut common_chars = first_chars.intersection(&second_chars);

        let common_char = common_chars.next().unwrap();

        score += get_char_score(common_char) as u64;
    }

    score
}

fn get_score_part2(lines: std::str::Lines) -> u64 {
    let mut score = 0u64;

    for [line1, line2, line3] in lines.array_chunks::<3>() {
        let first_chars: HashSet<char> = HashSet::from_iter(line1.chars());
        let second_chars: HashSet<char> = HashSet::from_iter(line2.chars());
        let third_chars: HashSet<char> = HashSet::from_iter(line3.chars());

        let common_chars: HashSet<char> = first_chars.intersection(&second_chars).copied().collect();
        let mut common_chars = common_chars.intersection(&third_chars);

        let common_char = common_chars.next().unwrap();

        score += get_char_score(common_char) as u64;
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

    const INPUT: &'static str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        assert_eq!(get_score_part1(INPUT.lines()), 157);
    }

    #[test]
    fn part2_works() {
        assert_eq!(get_score_part2(INPUT.lines()), 70);
    }
}
