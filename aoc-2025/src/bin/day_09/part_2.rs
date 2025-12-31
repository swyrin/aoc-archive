use crate::input::Input;
use aoc_lib::ext::geo::{Contains, LineString, Point, Polygon, Rect, point};
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
    expected_out = 24
)]
pub fn part_2(input: Input, _is_sample: bool) -> Umi {
    let mut points: Vec<Point> = vec![];

    for (x, y) in input.points {
        points.push(point! { x: x as f64, y: y as f64 });
    }

    let poly = Polygon::new(LineString::from(points.clone()), vec![]);

    let n = points.len();

    let mut max_area: f64 = 0_f64;

    for i in 0..n {
        for j in (i + 1)..n {
            let rect = Rect::new(points[i], points[j]);

            let (x1, y1) = points[i].x_y();
            let (x2, y2) = points[j].x_y();

            let diff_x = (x2 - x1).abs() + 1_f64;
            let diff_y = (y2 - y1).abs() + 1_f64;

            let area = diff_x * diff_y;

            if area > max_area && poly.contains(&rect) {
                max_area = max_area.max(area);
            }
        }
    }

    Umi::from_number(max_area as u128)
}
