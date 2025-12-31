use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "987654321111111
811111111111119
234234234234278
818181911112111",
    expected_out = 357
)]
pub fn part_1(input: Input, _is_sample: bool) -> Umi {
    let mut total: usize = 0;

    for line in input.lines {
        let numbers: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();

        // basically, a linear function mx + b, at its maxima when m and b reaches max.

        let index_max_1: usize = numbers
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .expect("Hmmmm.");

        let max_1 = numbers[index_max_1];

        let mut max_left = u32::MIN;
        let mut max_right = u32::MIN;

        for left_i in numbers.iter().take(index_max_1) {
            max_left = max_left.max(*left_i);
        }

        for right_i in numbers.iter().skip(index_max_1 + 1) {
            max_right = max_right.max(*right_i);
        }

        let mut result1 = max_left * 10 + max_1;
        let mut result2 = max_1 * 10 + max_right;

        if max_left == 0 {
            result1 = 0;
        }

        if max_right == 0 {
            result2 = 0;
        }

        let volt = u32::max(result1, result2);

        total += volt as usize;
    }

    Umi::from_number(total as u128)
}
