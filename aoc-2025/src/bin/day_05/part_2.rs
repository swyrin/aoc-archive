use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;
use std::cmp;

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    expected_out = 14
)]
pub fn part_2(input: Input, _is_sample: bool) -> Umi {
    let mut ranges: Vec<Vec<i64>> = input.ranges.into_iter().map(|(a, b)| vec![a, b]).collect();

    // It's been years since I last seen a range combinator
    ranges.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut combined: Vec<Vec<i64>> = vec![];
    combined.push(ranges[0].clone());

    for i in 1..ranges.len() {
        let current: Vec<i64> = ranges[i].clone();
        let j: usize = combined.len() - 1;

        if combined[j][0] <= current[0] && current[0] <= combined[j][1] {
            combined[j][1] = cmp::max(current[1], combined[j][1]);
        } else {
            combined.push(current);
        }
    }

    let mut total = 0;

    for r in combined {
        let a = r[0];
        let b = r[1];

        total += (b - a) + 1;
    }

    Umi::from_number(total as u128)
}
