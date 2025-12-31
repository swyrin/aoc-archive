use aoc_lib::input::InputLike;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Point3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point3 {
    pub fn distance_from(&self, other: &Self) -> isize {
        (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)
    }

    pub fn from_str(line: &str) -> Self {
        let numbers: Vec<&str> = line.trim().split(",").take(3).collect();

        Self {
            x: numbers[0].parse().unwrap(),
            y: numbers[1].parse().unwrap(),
            z: numbers[2].parse().unwrap(),
        }
    }
}

/// The structured input.
///
/// Mind you, this represents an ENTIRE INPUT FILE.
#[derive(Debug)]
pub struct Input {
    pub points: Vec<Point3>,
}

impl InputLike for Input {
    fn from_day_number(number: u8) -> Self {
        let _swallow = number;
        Self { points: vec![] }
    }

    fn from_str(content: &str) -> Self {
        let points = content.lines().map(Point3::from_str).collect();
        Self { points }
    }
}
