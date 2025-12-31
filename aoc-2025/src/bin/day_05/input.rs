use aoc_lib::input::InputLike;
use std::collections::HashSet;

/// The structured input.
///
/// Mind you, this represents an ENTIRE INPUT FILE.
#[derive(Debug)]
pub struct Input {
    pub ranges: HashSet<(i64, i64)>,
    pub numbers: Vec<i64>,
}

impl InputLike for Input {
    fn from_day_number(number: u8) -> Self {
        let _swallow = number;
        Self {
            ranges: HashSet::new(),
            numbers: vec![],
        }
    }

    fn from_str(content: &str) -> Self {
        let mut reading_actual_input = false;
        let mut ranges: HashSet<(i64, i64)> = HashSet::new();
        let mut numbers = vec![];

        for line in content.lines() {
            if line.is_empty() {
                reading_actual_input = true;
                continue;
            }

            if !reading_actual_input {
                let heads: Vec<&str> = line.split('-').collect();
                let head = heads[0].parse::<i64>().unwrap();
                let tail = heads[1].parse::<i64>().unwrap();
                ranges.insert((head, tail));
            } else {
                let number = line.parse::<i64>().unwrap();
                numbers.push(number);
            }
        }

        Self { ranges, numbers }
    }
}
