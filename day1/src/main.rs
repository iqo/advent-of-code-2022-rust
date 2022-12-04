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
    let mut elves = part_one_calculate_max_calories(&file_input);
    elves.sort();
    elves.reverse();
    print!(
        "Elf whit most calories: {} AND Sum of top three calories: {}",
        elves.iter().max().unwrap(),
        elves[..3].iter().sum::<usize>()
    );
}

fn read_input_file() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
}

fn part_one_calculate_max_calories(input: &str) -> Vec<usize> {
    let mut elves = Vec::new();
    let mut calories = 0;
    for line in input.lines() {
        if line.is_empty() {
            elves.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<usize>().unwrap();
        }
    }
    elves.push(calories);
    elves
}
