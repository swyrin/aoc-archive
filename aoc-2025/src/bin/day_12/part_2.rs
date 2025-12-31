use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "0",
    expected_out = 0
)]
pub fn part_2(_input: Input, _is_sample: bool) -> Umi {
    // Ho ho ho!
    // Part 2 is this.
    Umi::from_number(0)
}
