use std::collections::BinaryHeap;

fn get_n_most_calories(calories_lines: std::str::Lines, n: usize) -> u64 {
    let mut calories_per_elf = BinaryHeap::new();
    let mut calories_in_this_elf = 0u64;

    for calories in calories_lines {
        if calories.is_empty() {
            calories_per_elf.push(calories_in_this_elf);
            calories_in_this_elf = 0;
            continue;
        }

        calories_in_this_elf += calories.parse::<u64>().unwrap();
    }

    calories_per_elf.push(calories_in_this_elf);

    let mut accumulate: u64 = 0;
    for _ in 0..n {
        accumulate += calories_per_elf.pop().unwrap();
    }

    accumulate
}

fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();

    println!("{}", get_n_most_calories(file.lines(), 3));
}

#[cfg(test)]
mod tests {
    use crate::{get_n_most_calories};

    const INPUT: &'static str = r"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn get_most_calories_works() {
        assert_eq!(get_n_most_calories(INPUT.lines(), 1), 24_000);
    }

    #[test]
    fn get_3_most_calories_works() {
        assert_eq!(get_n_most_calories(INPUT.lines(), 3), 45_000);
    }
}