use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;

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
    expected_out = 3
)]
pub fn part_1(input: Input, _is_sample: bool) -> Umi {
    let mut count = 0;

    for number in input.numbers {
        let mut has_match = false;

        for range in input.ranges.clone().into_iter() {
            let a = range.0;
            let b = range.1;

            if a <= number && number <= b {
                has_match = true;
                break;
            }
        }

        if has_match {
            count += 1;
        }
    }

    Umi::from_number(count)
}
