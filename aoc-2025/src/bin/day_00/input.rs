use aoc_lib::input::InputLike;

/// The structured input.
///
/// Mind you, this represents an ENTIRE INPUT FILE.
pub struct Input {
    pub number: u32,
}

impl InputLike for Input {
    fn from_day_number(number: u8) -> Self {
        let _swallow = number;

        Self { number: 0 }
    }

    fn from_str(content: &str) -> Self {
        let num: u32 = content.parse().unwrap();

        Self { number: num }
    }
}
