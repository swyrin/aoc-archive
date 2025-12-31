use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ",
    expected_out = 3263827
)]
pub fn part_2(input: Input, _is_sample: bool) -> Umi {
    let content = input.content;
    let mut total = 0u128;

    let lines: Vec<&str> = content.lines().collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut numbers: Vec<u32> = vec![];

    for w in (0..width).rev() {
        let number_count = height - 1;
        let mut parsed_num = 0;

        for h in 0..number_count {
            let character = lines[h].chars().nth(w).expect("Huh?");

            if character != ' ' {
                parsed_num = parsed_num * 10 + character.to_digit(10).expect("NaN");
            }
        }

        if parsed_num == 0 {
            numbers.clear();
            continue;
        }

        numbers.push(parsed_num);

        let operand = lines[height - 1]
            .chars()
            .nth(w)
            .expect("Did you turn off whitespace trim?");

        if operand != ' ' {
            let mut val = if operand == '*' { 1 } else { 0 };
            let is_mul = operand == '*';

            for number in numbers.iter() {
                if is_mul {
                    val *= *number;
                } else {
                    val += *number;
                }
            }

            total += val as u128;
        }
    }

    Umi::from_number(total)
}
