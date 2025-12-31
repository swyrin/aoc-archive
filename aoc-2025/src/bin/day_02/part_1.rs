use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
    expected_out = 1227775554
)]
pub fn part_1(input: Input, _is_sample: bool) -> Umi {
    let mut result = 0;

    for (head, tail) in input.ranges {
        for i in head..=tail {
            let x = i.to_string();
            let l = x.len();

            if l % 2 != 0 {
                continue;
            }

            let first_half = &x[..(l / 2)];
            let second_half = &x[(l / 2)..];

            if first_half == second_half {
                result += i;
            }
        }
    }

    Umi::from_number(result as u128)
}
