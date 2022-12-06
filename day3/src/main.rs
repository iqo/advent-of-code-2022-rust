use std::collections::HashMap;

const TESTDATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
fn priority(b: u8) -> i32 {
    (97..=122) // assci a till z
        .chain(65..=90) //assci A till Z
        .zip(1..=52)
        .collect::<HashMap<_, _>>()[&b]
}
fn part_one(input: &str) -> i32{
input.lines().map(|f| {
    let (a, b) = f.split_at(f.len() / 2);
    let backpack_index = a.find(&b.chars().collect::<Vec<_>>()[..]).unwrap();
    priority(a.as_bytes()[backpack_index]) // get value index on split string
}).sum()
}
fn main() { 
let file_input: String = read_input_file();
let part_one_backpacks_test_data = part_one(TESTDATA);
let part_one_backpacks = part_one(&file_input);
    assert_eq!(part_one_backpacks_test_data, 157);
    println!("{}", part_one_backpacks);
}
fn read_input_file() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
}