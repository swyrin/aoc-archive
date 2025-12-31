use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;
use std::collections::HashSet;

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
    expected_out = 43
)]
pub fn part_2(input: Input, _is_sample: bool) -> Umi {
    let grid = input.grid;
    let mut ignore_list: HashSet<(i32, i32)> = HashSet::new();
    let mut destroy_count = 0;

    loop {
        let mut paper_rolls = 0;

        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;

        let is_a_paper_roll = |v: &Vec<Vec<char>>, i: &HashSet<(i32, i32)>, r: i32, c: i32| {
            0 <= r
                && r < rows
                && 0 <= c
                && c < cols
                && !i.contains(&(r, c))
                && v[r as usize][c as usize] == '@'
        };

        for i in 0..rows {
            for j in 0..cols {
                let mut count = 0;

                if ignore_list.contains(&(i, j)) {
                    continue;
                }

                if grid[i as usize][j as usize] == '.' {
                    ignore_list.insert((i, j));
                    continue;
                }

                // 12h
                if is_a_paper_roll(&grid, &ignore_list, i - 1, j) {
                    count += 1;
                }

                // 1h30
                if is_a_paper_roll(&grid, &ignore_list, i - 1, j + 1) {
                    count += 1;
                }

                // 3h
                if is_a_paper_roll(&grid, &ignore_list, i, j + 1) {
                    count += 1;
                }

                // 4h30
                if is_a_paper_roll(&grid, &ignore_list, i + 1, j + 1) {
                    count += 1;
                }

                // 6h
                if is_a_paper_roll(&grid, &ignore_list, i + 1, j) {
                    count += 1;
                }

                // 7h30
                if is_a_paper_roll(&grid, &ignore_list, i + 1, j - 1) {
                    count += 1;
                }

                // 9h
                if is_a_paper_roll(&grid, &ignore_list, i, j - 1) {
                    count += 1;
                }

                // 10h30
                if is_a_paper_roll(&grid, &ignore_list, i - 1, j - 1) {
                    count += 1;
                }

                if count < 4 {
                    paper_rolls += 1;
                    ignore_list.insert((i, j));
                }
            }
        }

        if paper_rolls == 0 {
            break;
        } else {
            destroy_count += paper_rolls;
        }
    }

    Umi::from_number(destroy_count)
}
