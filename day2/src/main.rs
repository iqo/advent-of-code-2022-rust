use std::cmp::Ordering;
use std::str::FromStr;
use std::string::ParseError;
/* const TESTDATA: &str = "A Y
B X
C Z
"; */
#[derive(PartialEq, Debug, Clone, Copy)]
enum PlayerMove {
    Rock,
    Paper,
    Scissor,
}
/* enum MatchOutcome {
    Win,
    Draw,
    Lose,
} */

impl PartialOrd for PlayerMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self == other {
            Ordering::Equal
        } else if self.greater() == *other {
            Ordering::Less
        } else {
            Ordering::Greater
        })
    }
}

impl PlayerMove {
    fn player_move(&self) -> i32 {
        match self {
            PlayerMove::Rock => 1,
            PlayerMove::Paper => 2,
            PlayerMove::Scissor => 3,
        }
    }
    // Greater than self
    fn greater(&self) -> Self {
        match self {
            PlayerMove::Rock => Self::Paper,
            PlayerMove::Paper => Self::Scissor,
            PlayerMove::Scissor => Self::Rock,
        }
    }
    // Less then self
    fn less(&self) -> Self {
        match self {
            PlayerMove::Rock => Self::Scissor,
            PlayerMove::Paper => Self::Rock,
            PlayerMove::Scissor => Self::Paper,
        }
    }
}

impl FromStr for PlayerMove {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissor),
            _ => panic!("failed to match {s}"),
        }
    }
}

fn game_score(our: PlayerMove, their: PlayerMove) -> (i32, i32) {

    let (o, t) = 
    if our == their {
        (3, 3)
    } else if our < their {
        (0, 6)
    } else {
        (6, 0)
    };
    (o + our.player_move(), t + their.player_move())
}

fn part_one(input: &str) -> i32 {
    let mut game_total_score = 0;
    for s in input.lines() {
        let split_line: Vec<_> = s.split_ascii_whitespace().collect();
        /*         
        println!("{:?}", split_line);
        */
        let opponent_move = PlayerMove::from_str(split_line[0]).unwrap();
        let our_move = PlayerMove::from_str(split_line[1]).unwrap();
        let round_score = game_score(opponent_move, our_move);
        game_total_score += round_score.0;
    }
    game_total_score
}
fn main() {
    let file_input: String = read_input_file();
    let part_one_scores = part_one(&file_input);
    println!("Your total score of the game is: {} points", part_one_scores);
}
fn read_input_file() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
}
