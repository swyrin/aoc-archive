use aoc_lib::input::InputLike;

#[derive(Debug)]
pub struct Adjacent {
    pub from: String,
    pub neighbors: Vec<String>,
}

/// The structured input.
///
/// Mind you, this represents an ENTIRE INPUT FILE.
#[derive(Debug)]
pub struct Input {
    pub entries: Vec<Adjacent>,
}

impl InputLike for Input {
    fn from_day_number(number: u8) -> Self {
        let _swallow = number;
        Self { entries: vec![] }
    }

    fn from_str(content: &str) -> Self {
        let mut entries = vec![];

        for line in content.lines() {
            let components: Vec<&str> = line.split(' ').collect();

            let from = components[0].replace(':', "");
            let neighbors: Vec<String> = components
                .into_iter()
                .skip(1)
                .map(|x| x.to_string())
                .collect();

            entries.push(Adjacent { from, neighbors })
        }

        Self { entries }
    }
}
