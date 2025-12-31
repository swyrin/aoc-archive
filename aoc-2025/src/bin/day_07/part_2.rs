use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;
use std::collections::HashMap;

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
    expected_out = 40
)]
pub fn part_2(input: Input, _is_sample: bool) -> Umi {
    let content = input.content;
    // how many rays to reach x
    let mut rays: HashMap<usize, u128> = HashMap::new();

    let lines = content.lines();

    for line in lines.clone().take(1) {
        for (x, chr) in line.chars().enumerate() {
            if chr == 'S' {
                rays.insert(x, 1_u128);
                break;
            }
        }
    }

    for line in lines.skip(2).step_by(2) {
        let mut next_rays = HashMap::new();

        for (x, chr) in line.chars().enumerate() {
            if chr == '^' {
                for (k, v) in &rays {
                    // branch if we see a spiltter.
                    if *k == x {
                        next_rays
                            .entry(x + 1)
                            .and_modify(|umeow| *umeow += v)
                            .or_insert(v.clone());

                        next_rays
                            .entry(x - 1)
                            .and_modify(|umeow| *umeow += v)
                            .or_insert(v.clone());
                    }
                }
            } else {
                for (k, v) in &rays {
                    if *k == x {
                        // or else, we go down.
                        next_rays
                            .entry(x)
                            .and_modify(|umeow| *umeow += v)
                            .or_insert(v.clone());
                    }
                }
            }
        }

        rays = next_rays;
    }

    Umi::from_number(rays.values().sum())
}
