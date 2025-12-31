use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;

/// NSFW warning?
#[derive(Debug, Copy, Clone)]
struct Umipai {
    digit: u32,
}

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "987654321111111
811111111111119
234234234234278
818181911112111",
    expected_out = 3121910778619
)]
pub fn part_2(input: Input, _is_sample: bool) -> Umi {
    // can't believe we jumped from 2 to 12, smh.
    let mut total: i128 = 0;

    // basically, try to form the longest number chain possible,
    // like 9999..9, 999..8, ... and so on.
    // https://en.wikipedia.org/wiki/Tournament_sort
    // (sans the min-heap part).
    for line in input.lines {
        let numbers: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        let mut arr: Vec<Umipai> = vec![];

        for (i, v) in numbers.iter().enumerate() {
            let num = *v;
            let obj = Umipai { digit: num };

            loop {
                // for some reasons, leave this code above the `loop` doesn't work.
                if arr.is_empty() {
                    arr.push(obj);
                    break;
                    // continue;
                }

                let unused_number_count = line.len() - i;
                let slot_count = 12 - arr.len();
                let last = *arr.last().unwrap();

                // basically, clean up small numbers so that
                // the larger number (champion) will join the inner bracket.
                // to create a chain of 9's, then 8's, ...
                if last.digit < num && unused_number_count > slot_count {
                    arr.pop();
                    continue;
                }

                // then the champion joins the bracket.
                if slot_count > 0 {
                    arr.push(Umipai { digit: num });
                    break;
                } else {
                    break;
                }
            }
        }

        if arr.is_empty() {
            panic!("Why are you panic?")
        }

        let number: i128 = arr
            .iter()
            .fold(0, |umi, meow| umi * 10 + meow.digit as i128);

        total += number;
    }

    Umi::from_number(total as u128)
}
