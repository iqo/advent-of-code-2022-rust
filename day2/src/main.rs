use std::cmp::Ordering;
use std::str::FromStr;
use std::string::ParseError;
const TESTDATA: &str = "
A Y
B X
C Z
";
// #[derive(Eq)]
enum PlayerMove {
    Rock,
    Paper,
    Scissor,
}
enum MatchOutcome {
    Win,
    Draw,
    Lose,
}

impl PlayerMove {
    fn player_move(&self) -> i32 {
        match self {
            PlayerMove::Rock => 1,
            PlayerMove::Paper => 2,
            PlayerMove::Scissor => 3
        }
    }
    // Greater than self
    fn greater(&self) -> Self{
        match self {
            PlayerMove::Rock => Self::Paper,
            PlayerMove::Paper => Self::Scissor,
            PlayerMove::Scissor => Self::Rock,
        }
    }
    // Less then self
    fn less(&self) -> Self{
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
            _ => panic!("failed to match {s}")
        }
    }
}
fn main() {
    /*     let file_input: String = read_input_file();
     */
    println!("{}", TESTDATA);
}
/* fn read_input_file() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
} */
