use std::cmp::Ordering;
use std::str::FromStr;
use std::string::ParseError;
const TESTDATA: &str = "
A Y
B X
C Z
";
#[derive(PartialEq)]
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

fn player_move(our: PlayerMove, their: PlayerMove) -> (i32, i32) {
    let (o, t) = if our == their {
        (3, 3)
    } else if our < their {
        (0, 6)
    } else {
        (6, 0)
    };
    (o + our.player_move(), t + their.player_move())
}
fn main() {
    /*     let file_input: String = read_input_file();
     */
    println!("{}", TESTDATA);
}
/* fn read_input_file() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
} */
