#[derive(PartialEq, Copy, Clone)]
enum Object {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq)]
enum GameResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

fn parse_opponent_str(string: &char) -> Object {
    match string {
        'A' => Object::Rock,
        'B' => Object::Paper,
        'C' => Object::Scissors,
        _ => unreachable!()
    }
}

fn parse_my_str(string: &char) -> Object {
    match string {
        'X' => Object::Rock,
        'Y' => Object::Paper,
        'Z' => Object::Scissors,
        _ => unreachable!()
    }
}

fn parse_game_result(string: &char) -> GameResult {
    match string {
        'X' => GameResult::Loss,
        'Y' => GameResult::Draw,
        'Z' => GameResult::Win,
        _ => unreachable!()
    }
}

fn get_game_result(me: &Object, opponent: &Object) -> GameResult {
    if *me == *opponent {
        return GameResult::Draw;
    }

    match *me {
        Object::Rock => if *opponent == Object::Paper { GameResult::Loss } else { GameResult::Win }
        Object::Paper => if *opponent == Object::Scissors { GameResult::Loss } else { GameResult::Win }
        Object::Scissors => if *opponent == Object::Rock { GameResult::Loss } else { GameResult::Win }
    }
}

fn get_my_play_from_game_result(opponent: &Object, result: &GameResult) -> Object {
    if *result == GameResult::Draw {
        return *opponent;
    }

    match *opponent {
        Object::Rock => if *result == GameResult::Win { Object::Paper } else { Object::Scissors }
        Object::Paper => if *result == GameResult::Win { Object::Scissors } else { Object::Rock }
        Object::Scissors => if *result == GameResult::Win { Object::Rock } else { Object::Paper }
    }
}

fn get_score_part1(lines: std::str::Lines) -> u64 {
    let mut score = 0u64;

    for line in lines {
        let opponent = line.chars().nth(0).unwrap();
        let me = line.chars().nth(2).unwrap();

        let opponent = parse_opponent_str(&opponent);
        let me = parse_my_str(&me);

        let result = get_game_result(&me, &opponent);

        score += result as u64 + me as u64;
    }

    score
}

fn get_score_part2(lines: std::str::Lines) -> u64 {
    let mut score = 0u64;

    for line in lines {
        let opponent = line.chars().nth(0).unwrap();
        let result = line.chars().nth(2).unwrap();

        let opponent = parse_opponent_str(&opponent);
        let result = parse_game_result(&result);

        let my_play = get_my_play_from_game_result(&opponent, &result);

        score += result as u64 + my_play as u64;
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

    const INPUT: &'static str =
        r"A Y
B X
C Z";

    #[test]
    fn part1_works() {
        assert_eq!(get_score_part1(INPUT.lines()), 15);
    }


    #[test]
    fn part2_works() {
        assert_eq!(get_score_part2(INPUT.lines()), 12);
    }
}
