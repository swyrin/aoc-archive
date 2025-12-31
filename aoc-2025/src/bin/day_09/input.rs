use aoc_lib::input::InputLike;

/// The structured input.
///
/// Mind you, this represents an ENTIRE INPUT FILE.
#[derive(Debug)]
pub struct Input {
    pub points: Vec<(i128, i128)>,
}

impl InputLike for Input {
    fn from_day_number(number: u8) -> Self {
        let _swallow = number;
        Self { points: vec![] }
    }

    fn from_str(content: &str) -> Self {
        let points = content
            .lines()
            .map(|line| {
                let items: Vec<&str> = line.split(",").collect();
                let x: i128 = items[0].parse().unwrap();
                let y: i128 = items[1].parse().unwrap();
                (x, y)
            })
            .collect();

        Self { points }
    }
}
