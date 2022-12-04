/* const TESTDATA: &str = "
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
"; */
fn main() {
    let file_input: String = read_input_file();
    // print!("{}", TESTDATA);
    get_test_data_debug(&file_input);
}

fn read_input_file() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
}

fn get_test_data_debug(input: &str) {
    let mut elves = Vec::new();
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            elves.push(sum);
            sum = 0;
            // dbg!(line);
        } else {
            sum += line.parse::<usize>().unwrap();
        }
    }
    print!("max elves calories = {}", elves.iter().max().unwrap());
}
