use aoc_lib::input::InputLike;

/// The structured input.
///
/// Mind you, this represents an ENTIRE INPUT FILE.
#[derive(Debug)]
pub struct Input {
    pub ranges: Vec<(i64, i64)>,
}

impl InputLike for Input {
    fn from_day_number(number: u8) -> Self {
        let _swallow = number;
        Self { ranges: vec![] }
    }

    fn from_str(content: &str) -> Self {
        let ranges = content
            .trim()
            .split(',')
            .map(|range| {
                let parts: Vec<&str> = range.split('-').collect();
                let head = parts[0].parse::<i64>().unwrap();
                let tail = parts[1].parse::<i64>().unwrap();
                (head, tail)
            })
            .collect();

        Self { ranges }
    }
}
