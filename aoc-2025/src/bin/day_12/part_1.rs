use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;

/// Eric put a troll problem.
#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2",
    expected_out = 3
)]
pub fn part_1(input: Input, is_sample: bool) -> Umi {
    let bricks = if is_sample {
        [7, 7, 7, 7, 7, 7]
    } else {
        [6, 7, 5, 7, 7, 7]
    };

    let content = input.raw_input;
    let mut result_count = 0;

    for line in content.lines() {
        if line.contains(": ") {
            let (board, count) = line.split_once(": ").unwrap();

            let (x, y) = board.split_once("x").unwrap();

            let board_size = x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap();

            let piece_count: Vec<u32> = count
                .split(" ")
                .map(|x| (*x).parse::<u32>().unwrap())
                .collect();

            let mut sum: u32 = 0;

            for (piece_slot, available) in piece_count.iter().zip(bricks) {
                sum += (piece_slot) * available;
            }
            if board_size > sum {
                result_count += 1;
            }
        }
    }

    Umi::from_number(result_count)
}
