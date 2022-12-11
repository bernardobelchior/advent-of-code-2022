use std::collections::{HashMap, VecDeque};

fn get_first_marker_position(datastream: &str) -> u64 {
    let mut i = 0;
    let mut chars_deque = VecDeque::<char>::with_capacity(4);
    let mut chars_map = HashMap::<char, u8>::with_capacity(4);

    for character in datastream.chars() {
        chars_deque.push_back(character);
        *chars_map.entry(character).or_insert(0) += 1;
        i += 1;

        if chars_deque.len() < 4 {
            continue;
        }

        if chars_map.len() == 4 {
            return i;
        }

        let removed_char = chars_deque.pop_front().unwrap();
        if *chars_map.get(&removed_char).unwrap() == 1 {
            chars_map.remove(&removed_char);
        } else {
            chars_map.entry(removed_char).and_modify(|count| *count -= 1);
        }
    }

    i
}

fn get_message_marker_position(datastream: &str) -> u64 {
    let mut i = 0;
    let mut chars_deque = VecDeque::<char>::with_capacity(14);
    let mut chars_map = HashMap::<char, u8>::with_capacity(14);

    for character in datastream.chars() {
        chars_deque.push_back(character);
        *chars_map.entry(character).or_insert(0) += 1;
        i += 1;

        if chars_deque.len() < 14 {
            continue;
        }

        if chars_map.len() == 14 {
            return i;
        }

        let removed_char = chars_deque.pop_front().unwrap();
        if *chars_map.get(&removed_char).unwrap() == 1 {
            chars_map.remove(&removed_char);
        } else {
            chars_map.entry(removed_char).and_modify(|count| *count -= 1);
        }
    }

    i
}

fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();

    println!("{}", get_message_marker_position(&file));
}

#[cfg(test)]
mod tests {
    use crate::{get_first_marker_position, get_message_marker_position};

    #[test]
    fn part1_works() {
        assert_eq!(get_first_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(get_first_marker_position("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(get_first_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(get_first_marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }


    #[test]
    fn part2_works() {
        assert_eq!(get_message_marker_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(get_message_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(get_message_marker_position("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(get_message_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(get_message_marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
