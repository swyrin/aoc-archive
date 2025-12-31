use aoc_lib::input::InputLike;

/// The structured input.
///
/// Mind you, this represents an ENTIRE INPUT FILE.
#[derive(Debug)]
pub struct Input {
    pub moves: Vec<(char, i32)>,
}

impl InputLike for Input {
    fn from_day_number(number: u8) -> Self {
        let _swallow = number;
        Self { moves: vec![] }
    }

    fn from_str(content: &str) -> Self {
        let moves = content
            .lines()
            .map(|line| {
                let direction = line.chars().nth(0).unwrap();
                let amount = line[1..].parse::<i32>().unwrap();
                (direction, amount)
            })
            .collect();

        Self { moves }
    }
}
