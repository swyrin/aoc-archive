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
    expected_out = 6
)]
pub fn part_2(input: Input, _is_sample: bool) -> Umi {
    let mut pos: i64 = 50;
    let mut count: i64 = 0;

    for (direction, amount) in input.moves {
        let amount = amount as i64;
        count += amount / 100;
        let amount = amount % 100;

        match direction {
            'L' => {
                if pos != 0 && pos - amount <= 0 {
                    count += 1;
                }

                pos = (pos - amount).rem_euclid(100);
            }
            'R' => {
                if pos != 0 && pos + amount >= 100 {
                    count += 1;
                }

                pos = (pos + amount).rem_euclid(100);
            }
            _ => {
                panic!("Not a valid direction.")
            }
        }
    }

    Umi::from_number(count as u128)
}
