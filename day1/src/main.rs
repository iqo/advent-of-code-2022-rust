const TESTDATA: &str = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
fn main() {
    // let file_input:String = read_input_file();
    // let input1 = include_str!("input.txt");
    // println!("{}", file_input);
    print!("{}", TESTDATA);
    get_test_data_debug();
}

fn read_input_file() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
}

fn get_test_data_debug() {
    for line in TESTDATA.lines() {
        if line.is_empty() {
           dbg!(line); 
        }
    }
}