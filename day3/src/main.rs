use std::collections::{hash_map::RandomState, HashMap, HashSet};

const TESTDATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
fn backpack_priority_table(b: u8) -> i32 {
    (97..=122) // assci a till z
        .chain(65..=90) //assci A till Z
        .zip(1..=52)
        .collect::<HashMap<_, _>>()[&b]
}
fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let backpack_index = a.find(&b.chars().collect::<Vec<_>>()[..]).unwrap();
            backpack_priority_table(a.as_bytes()[backpack_index]) // get value index on split string
        })
        .sum()
}
fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|line| HashSet::from_iter(line.bytes()))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|f| {
            let [first, second, third] = f else { unreachable!()};
            let first_and_second: HashSet<u8, RandomState> = HashSet::from_iter(first.intersection(second).cloned());
            backpack_priority_table(*first_and_second.intersection(third).next().unwrap())
        })
        .sum()
}
fn main() {
    let file_input: String = read_input_file();
    let part_one_backpacks_test_data = part_one(TESTDATA);
    let part_one_backpacks = part_one(&file_input);
    let part_two_backpacks = part_two(&file_input);
    let part_two_backpacks_test_data = part_two(TESTDATA);
    assert_eq!(part_one_backpacks_test_data, 157);
    assert_eq!(part_two_backpacks_test_data, 70);
    println!("part one: {}, part two: {}", part_one_backpacks, part_two_backpacks);
}
fn read_input_file() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
}
