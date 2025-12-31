use aoc_lib::input::InputLike;

/// The structured input.
///
/// Mind you, this represents an ENTIRE INPUT FILE.
#[derive(Debug)]
pub struct Input {
    pub grid: Vec<Vec<char>>,
}

impl InputLike for Input {
    fn from_day_number(number: u8) -> Self {
        let _swallow = number;
        Self { grid: vec![] }
    }

    fn from_str(content: &str) -> Self {
        let grid = content.lines().map(|line| line.chars().collect()).collect();

        Self { grid }
    }
}
