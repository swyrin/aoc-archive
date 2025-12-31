use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "123",
    expected_out = 246
)]
pub fn part_2(input: Input, _is_sample: bool) -> Umi {
    Umi::from_number((input.number * 2) as u128)
}
