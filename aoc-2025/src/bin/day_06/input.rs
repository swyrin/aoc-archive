use aoc_lib::input::InputLike;

/// The structured input.
///
/// Mind you, this represents an ENTIRE INPUT FILE.
#[derive(Debug)]
pub struct Input {
    pub content: String,
}

impl InputLike for Input {
    fn from_day_number(number: u8) -> Self {
        let _swallow = number;
        Self {
            content: String::new(),
        }
    }

    fn from_str(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}
