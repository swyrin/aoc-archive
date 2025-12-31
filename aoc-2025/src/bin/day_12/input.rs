use aoc_lib::input::InputLike;

/// The structured input.
///
/// Mind you, this represents an ENTIRE INPUT FILE.
#[derive(Debug)]
pub struct Input {
    pub raw_input: String,
}

impl InputLike for Input {
    fn from_day_number(number: u8) -> Self {
        let _swallow = number;
        Self {
            raw_input: String::new(),
        }
    }

    fn from_str(content: &str) -> Self {
        Self {
            raw_input: content.to_string(),
        }
    }
}
