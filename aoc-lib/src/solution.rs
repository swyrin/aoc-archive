use num_bigint::BigUint;
use std::fmt::{Display, Formatter};

/// The answer seems a little bit cute today?
///
/// Basically the wrapper for [`num_bigint::BigUint`]
#[derive(Debug, PartialEq)]
pub struct Umi {
    /// The underlying answer.
    pub answer: BigUint,
}

impl Display for Umi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Feeling like {} because I'm cute :3", self.answer)
    }
}

impl Umi {
    pub fn from_number(num: u128) -> Self {
        Self {
            answer: BigUint::from(num),
        }
    }
}
