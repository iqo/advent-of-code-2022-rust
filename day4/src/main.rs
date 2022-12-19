use std::{str::FromStr, string::ParseError};

const TESTDATA: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
#[derive(Debug)]
struct SectionAssignmentRange {
    start: i32,
    end: i32,
}
impl SectionAssignmentRange {
    fn fully_contains ( &self, other: &Self) -> bool {
/*         dbg!(self);
        dbg!(other); */
        self.start <= other.start && self.end >= other.end
    }
     fn overlap_at_all(&self, other: &Self) -> bool {
        !(self.start > other.end || self.end < other.start)
     }
}
impl FromStr for SectionAssignmentRange {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair: Vec<_> = s
            .split('-')
            .flat_map(i32::from_str)
            .collect();
        let [start, end] = &pair[..] else { unreachable!() };
        Ok(Self {
            start: *start,
            end: *end,
        })
    }
}
fn part_one_and_two(input: &str, struct_function: fn(&SectionAssignmentRange, &SectionAssignmentRange) -> bool) -> i32 {
    input.lines().map(|line|{
        let comma_split = line.split(',').flat_map(SectionAssignmentRange::from_str);
        let [a, b] = &comma_split.collect::<Vec<_>>()[..] else { unreachable!()};
        (struct_function(a,b) || struct_function(b,a)) as i32
    }).sum()
}
fn main() {
    let data = read_input_file();
    let part_one_real_data = part_one_and_two(&data, SectionAssignmentRange::fully_contains);
    let part_one_test_data = part_one_and_two(TESTDATA, SectionAssignmentRange::fully_contains);
    let part_two_test_data = part_one_and_two(TESTDATA, SectionAssignmentRange::overlap_at_all);
    let part_two_real_data = part_one_and_two(&data, SectionAssignmentRange::overlap_at_all);


    assert_eq!(part_one_test_data, 2);
    assert_eq!(part_two_test_data, 4);

    println!("Part one {}: Part two {}", part_one_real_data, part_two_real_data);
}
fn read_input_file() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
}