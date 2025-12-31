use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ",
    expected_out = 4277556
)]
pub fn part_1(input: Input, _is_sample: bool) -> Umi {
    let content = input.content;
    let mut total = 0u128;

    let mut lines: Vec<Vec<&str>> = vec![];

    for line in content.lines() {
        let line: Vec<&str> = line.split(" ").collect();

        let filtered_empty: Vec<&str> = line.into_iter().filter(|x| !x.is_empty()).collect();

        lines.push(filtered_empty);
    }

    let lc = lines.len();

    let operands = lines.last().expect("No last element?");
    let numbers: Vec<&Vec<&str>> = lines.iter().take(lc - 1).collect();

    let width = lines[0].len();
    let height = lc - 1;

    for w in 0..width {
        let mut val = if operands[w] == "*" { 1 } else { 0 };
        let is_mul = operands[w] == "*";

        for h in 0..height {
            let num = numbers[h][w].parse::<i32>().expect("NaN");

            if is_mul {
                val *= num;
            } else {
                val += num;
            }
        }

        total += val as u128;
    }

    Umi::from_number(total)
}
