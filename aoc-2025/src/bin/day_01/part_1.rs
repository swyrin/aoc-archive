use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
    expected_out = 3
)]
pub fn part_1(input: Input, _is_sample: bool) -> Umi {
    let mut pos = 50;
    let mut count = 0;

    for (direction, amount) in input.moves {
        match direction {
            'L' => {
                pos = (pos - amount) % 100;
            }
            'R' => {
                pos = (pos + amount) % 100;
            }
            _ => {
                panic!("Not a valid direction.")
            }
        }

        if pos == 0 {
            count += 1;
        }
    }

    Umi::from_number(count)
}
