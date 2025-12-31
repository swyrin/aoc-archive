/// Mandatory trait for every type of input.
pub trait InputLike {
    /// Read input from `input/day_[xx].txt` file.
    ///
    /// If you use [`aoc_macro::aoc_run`] macro in `main()`, file existence is handled for you, so you
    /// only need to implement input parsing.
    fn from_day_number(number: u8) -> Self;

    /// Read input from a raw `&str`.
    ///
    /// Normally, you should implement this in conjunction with [`InputLike::from_day_number`],
    /// like making [`InputLike::from_day_number`] to read the content and then pass the processing
    /// to [`InputLike::from_str`].
    ///
    /// But you do you, who am I to judge?
    fn from_str(content: &str) -> Self;
}
