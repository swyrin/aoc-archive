use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;
use std::collections::{HashSet, VecDeque};

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    expected_out = 7
)]
pub fn part_1(input: Input, _is_sample: bool) -> Umi {
    let content = input.content;
    let mut total = 0;

    for line in content.lines() {
        let components: Vec<&str> = line.split(" ").collect();
        let component_count = components.len();

        let configuration = components[0];

        let buttons: Vec<&str> = components
            .clone()
            .into_iter()
            .skip(1)
            .take(component_count - 2)
            .collect();

        let _what = components.last().expect("There isn't any?");

        let switches: Vec<String> = buttons
            .into_iter()
            .map(|x| x.replace(['(', ')'], "").split(",").collect())
            .collect();

        let mut toggles: Vec<Vec<u32>> = vec![];

        for state in &switches {
            let numbers: Vec<&str> = state.split(",").collect();
            let numbers = numbers[0].chars();
            let numbers: Vec<u32> = numbers.map(|x| x.to_digit(10).unwrap()).collect();

            toggles.push(numbers);
        }

        let target: Vec<char> = configuration
            .chars()
            .skip(1)
            .take(configuration.len() - 2)
            .collect();

        let target: Vec<bool> = target.iter().map(|x| *x == '#').collect();
        let start: Vec<bool> = vec![false; target.len()];

        if start == target {
            continue;
        }

        // (configuration, press count)
        let mut deq = VecDeque::new();
        deq.push_back((start.clone(), 0));

        let mut seen_state = HashSet::new();
        seen_state.insert(start);

        while let Some((config, count)) = deq.pop_front() {
            if config == target {
                total += count;
                break;
            }

            for toggle in &toggles {
                let mut new_config = config.clone();

                for selected in toggle {
                    let index = *selected as usize;
                    new_config[index] = !new_config[index];
                }

                if seen_state.insert(new_config.clone()) {
                    deq.push_back((new_config, count + 1));
                }
            }
        }
    }

    Umi::from_number(total as u128)
}
