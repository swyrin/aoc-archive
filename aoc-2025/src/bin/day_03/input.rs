use aoc_lib::input::InputLike;

/// The structured input.
///
/// Mind you, this represents an ENTIRE INPUT FILE.
#[derive(Debug)]
pub struct Input {
    pub lines: Vec<String>,
}

impl InputLike for Input {
    fn from_day_number(number: u8) -> Self {
        let _swallow = number;
        Self { lines: vec![] }
    }

    fn from_str(content: &str) -> Self {
        let lines = content.lines().map(|s| s.to_string()).collect();
        Self { lines }
    }
}
