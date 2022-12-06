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
enum MatchOutcomePrediction {
    Win,
    Draw,
    Lose,
}

impl FromStr for MatchOutcomePrediction {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MatchOutcomePrediction::Lose),
            "Y" => Ok(MatchOutcomePrediction::Draw),
            "Z" => Ok(MatchOutcomePrediction::Win),
            _ => panic!("Faild to match {s}")
            
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
        let round_score = game_score(our_move, opponent_move);
        game_total_score += round_score.0;
    }
    game_total_score
}

fn part_two(input: &str) -> i32{
    let mut match_outcome_prediction_score = 0;
    for s in input.lines() {
        let split_line: Vec<_> = s.split_ascii_whitespace().collect();
        let opponent_move = PlayerMove::from_str(split_line[0]).unwrap();
        let match_outcome_prediction = MatchOutcomePrediction::from_str(split_line[1]).unwrap();
        let our_move = match(opponent_move, match_outcome_prediction)  {
            (t, MatchOutcomePrediction::Win) => t.greater(),
            (t, MatchOutcomePrediction::Draw) => t,
            (t, MatchOutcomePrediction::Lose) => t.less()
        };
        let round_score = game_score(our_move, opponent_move);
        match_outcome_prediction_score += round_score.0;
    }
    match_outcome_prediction_score
}
fn main() {
    let file_input: String = read_input_file();
    let part_one_scores = part_one(&file_input);
    let part_two_scores = part_two(&file_input);
    /* assert_eq!(part_one_scores, 15);
    assert_eq!(part_two_scores, 12); */
    println!("Your total score of the game is: {} points.  Part two scores {}", part_one_scores, part_two_scores);
}
fn read_input_file() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
}
