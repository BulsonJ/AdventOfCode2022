pub fn run() {
    let input = include_str!("../input/day2.txt");

    println!("PART 1: {}", day2_part_one(input));
    println!("PART 2: {}", day2_part_two(input));
}

#[derive(Clone, Copy, PartialEq)]
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

    fn beats(&self, other_play: &Self) -> bool {
        matches!(
            (self, other_play),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn get_move_needed_for_result(
        &self,
        result_required: &RockPaperScissorsResult,
    ) -> RockPaperScissors {
        const ALL_MOVES: [RockPaperScissors; 3] = [
            RockPaperScissors::Rock,
            RockPaperScissors::Paper,
            RockPaperScissors::Scissors,
        ];

        match result_required {
            RockPaperScissorsResult::Loss => ALL_MOVES
                .iter()
                .copied()
                .find(|&m| self.beats(&m))
                .expect("at least one move should win"),
            RockPaperScissorsResult::Draw => ALL_MOVES
                .iter()
                .copied()
                .find(|&m| *self == m)
                .expect("at least one move draws"),
            RockPaperScissorsResult::Win => ALL_MOVES
                .iter()
                .copied()
                .find(|&m| m.beats(self))
                .expect("at least one move loses"),
        }
    }

    fn get_outcome(&self, other_play: &Self) -> RockPaperScissorsResult {
        if self.beats(other_play) {
            RockPaperScissorsResult::Win
        } else if other_play.beats(self) {
            RockPaperScissorsResult::Loss
        } else {
            RockPaperScissorsResult::Draw
        }
    }

    fn get_result(&self, other_play: &Self) -> i32 {
        self.get_move_score()
            + match self.get_outcome(other_play) {
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
