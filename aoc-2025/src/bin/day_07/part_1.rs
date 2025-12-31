use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;
use std::collections::HashSet;

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
    expected_out = 21
)]
pub fn part_1(input: Input, _is_sample: bool) -> Umi {
    let content = input.content;
    let mut require_x = HashSet::new();

    let lines = content.lines();

    for line in lines.clone().take(1) {
        for (x, chr) in line.chars().enumerate() {
            if chr == 'S' {
                require_x.insert(x);
                break;
            }
        }
    }

    let mut total = 0_u128;

    for line in lines.skip(2).step_by(2) {
        let mut processed_x = HashSet::new();
        let mut next_x_require = HashSet::new();

        for (x, chr) in line.chars().enumerate() {
            if chr == '^' && require_x.contains(&x) {
                processed_x.insert(x);
                total += 1;

                next_x_require.insert(x - 1);
                next_x_require.insert(x + 1);
            }
        }

        for v in processed_x {
            require_x.remove(&v);
        }

        for v in next_x_require {
            require_x.insert(v);
        }
    }

    Umi::from_number(total)
}
