mod input;
mod part_1;
mod part_2;
use aoc_macro::aoc_run;
use input::Input;
use part_1::part_1;
use part_2::part_2;
fn main() {
    aoc_run!(
        day = 4,
        input_type = Input,
        run_part1 = true,
        run_part2 = true
    );
}
