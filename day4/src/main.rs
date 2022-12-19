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
        dbg!(self);
        dbg!(other);
        self.start <= other.start && self.end >= other.end
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
fn part_one(input: &str, struct_data: fn(&SectionAssignmentRange, &SectionAssignmentRange) -> bool) -> i32 {
    input.lines().map(|line|{
        let comma_split = line.split(',').flat_map(SectionAssignmentRange::from_str);
        let [a, b] = &comma_split.collect::<Vec<_>>()[..] else { unreachable!()};
        (struct_data(a,b) || struct_data(b,a)) as i32
    }).sum()
}
fn main() {
    let part_one_test_data = part_one(TESTDATA, SectionAssignmentRange::fully_contains);
    assert_eq!(part_one_test_data, 2);
}
