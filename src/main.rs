use std::fmt;

fn main() {
    let input = include_str!("../input/day2.txt");

    println!("PART 1: {}", day2_part_one(input));
    println!("PART 2: {}", day2_part_two(input));
}

fn day1_part_one(input: &str) -> i32 {
    let mut current_elf = 0;
    let mut elves = Vec::new();
    for line in input.split('\n') {
        if line == "\r" || line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
            continue;
        }
        let string = line.trim_end();
        let number = string.parse::<i32>().unwrap();

        current_elf += number;
    }
    elves.sort();
    elves.iter().rev().take(1).sum()
}

fn day1_part_two(input: &str) -> i32 {
    let mut current_elf = 0;
    let mut elves = Vec::new();
    for line in input.split('\n') {
        if line == "\r" || line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
            continue;
        }
        let string = line.trim_end();
        let number = string.parse::<i32>().unwrap();

        current_elf += number;
    }
    elves.sort();
    elves.iter().rev().take(3).sum()
}

enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

enum RockPaperScissorsResult {
    Win,
    Draw,
    Loss,
}

impl RockPaperScissorsResult {
    fn from_string(input: &str) -> Option<Self> {
        match input {
            "X" => Some(RockPaperScissorsResult::Loss),
            "Y" => Some(RockPaperScissorsResult::Draw),
            "Z" => Some(RockPaperScissorsResult::Win),
            _ => None,
        }
    }
}

impl RockPaperScissors {
    fn from_string(input: &str) -> Option<Self> {
        match input {
            "A" | "X" => Some(RockPaperScissors::Rock),
            "B" | "Y" => Some(RockPaperScissors::Paper),
            "C" | "Z" => Some(RockPaperScissors::Scissors),
            _ => None,
        }
    }

    fn get_move_score(&self) -> i32 {
        match self {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3,
        }
    }

    fn beats(&self, other_play: &Self) -> RockPaperScissorsResult {
        match self {
            RockPaperScissors::Rock => match other_play {
                RockPaperScissors::Rock => RockPaperScissorsResult::Draw,
                RockPaperScissors::Paper => RockPaperScissorsResult::Loss,
                RockPaperScissors::Scissors => RockPaperScissorsResult::Win,
            },
            RockPaperScissors::Paper => match other_play {
                RockPaperScissors::Rock => RockPaperScissorsResult::Win,
                RockPaperScissors::Paper => RockPaperScissorsResult::Draw,
                RockPaperScissors::Scissors => RockPaperScissorsResult::Loss,
            },
            RockPaperScissors::Scissors => match other_play {
                RockPaperScissors::Rock => RockPaperScissorsResult::Loss,
                RockPaperScissors::Paper => RockPaperScissorsResult::Win,
                RockPaperScissors::Scissors => RockPaperScissorsResult::Draw,
            },
        }
    }

    fn get_move_needed_for_result(
        &self,
        result_required: &RockPaperScissorsResult,
    ) -> RockPaperScissors {
        match result_required {
            RockPaperScissorsResult::Win => match self {
                RockPaperScissors::Rock => RockPaperScissors::Paper,
                RockPaperScissors::Paper => RockPaperScissors::Scissors,
                RockPaperScissors::Scissors => RockPaperScissors::Rock,
            },
            RockPaperScissorsResult::Draw => match self {
                RockPaperScissors::Rock => RockPaperScissors::Rock,
                RockPaperScissors::Paper => RockPaperScissors::Paper,
                RockPaperScissors::Scissors => RockPaperScissors::Scissors,
            },
            RockPaperScissorsResult::Loss => match self {
                RockPaperScissors::Rock => RockPaperScissors::Scissors,
                RockPaperScissors::Paper => RockPaperScissors::Rock,
                RockPaperScissors::Scissors => RockPaperScissors::Paper,
            },
        }
    }

    fn get_result(&self, other_play: &Self) -> i32 {
        self.get_move_score()
            + match self.beats(other_play) {
                RockPaperScissorsResult::Win => 6,
                RockPaperScissorsResult::Draw => 3,
                RockPaperScissorsResult::Loss => 0,
            }
    }
}

fn day2_part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|battle| {
            let mut moves = battle.split(' ');

            let opponent_move =
                RockPaperScissors::from_string(moves.next().unwrap()).expect("Not a valid move");
            let player_move =
                RockPaperScissors::from_string(moves.next().unwrap()).expect("Not a valid move");

            player_move.get_result(&opponent_move)
        })
        .sum()
}

fn day2_part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|battle| {
            let mut moves = battle.split(' ');

            let opponent_move =
                RockPaperScissors::from_string(moves.next().unwrap()).expect("Not a valid move");
            let player_needed_result = RockPaperScissorsResult::from_string(moves.next().unwrap())
                .expect("Not a valid move");
            let required_move = opponent_move.get_move_needed_for_result(&player_needed_result);

            required_move.get_result(&opponent_move)
        })
        .sum()
}
