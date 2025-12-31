use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
    expected_out = 50
)]
pub fn part_1(input: Input, _is_sample: bool) -> Umi {
    let points = input.points;
    let n = points.len();

    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            let diff_x = (x2).abs_diff(x1) as i128 + 1_i128;
            let diff_y = (y2).abs_diff(y1) as i128 + 1_i128;

            max_area = max_area.max(diff_x * diff_y);
        }
    }

    Umi::from_number(max_area as u128)
}
