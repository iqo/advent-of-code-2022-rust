use std::cmp::Ordering;
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
}
fn main() {
    /*     let file_input: String = read_input_file();
     */
    println!("{}", TESTDATA);
}
/* fn read_input_file() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
} */
