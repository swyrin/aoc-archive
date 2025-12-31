use crate::input::Input;
use aoc_lib::ext::good_lp::{
    Expression, IntoAffineExpression, Solution, SolverModel, Variable, microlp, variable, variables,
};
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    expected_out = 33
)]
pub fn part_2(input: Input, _is_sample: bool) -> Umi {
    let content = input.content;
    let mut total = 0_f64;

    for line in content.lines() {
        let components: Vec<&str> = line.split(" ").collect();
        let component_count = components.len();

        let buttons: Vec<&str> = components
            .clone()
            .into_iter()
            .skip(1)
            .take(component_count - 2)
            .collect();

        let jolts = components.last().expect("There isn't any?");
        let jolts = jolts.replace(['{', '}'], "");

        let jolts: Vec<u32> = jolts
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

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

        let mut vars = variables!();

        // basically, we do an [A | x] = B where A is the button press toggles, B is the required count
        // the result will be vector x which is the number of each button touches.
        let presses: Vec<Variable> = (0..toggles.len())
            .map(|_| vars.add(variable().min(0).integer()))
            .collect();

        let mut optimization = microlp(vars.minimise(presses.iter().sum::<Expression>()));
        let mut expressions = vec![0.into_expression(); jolts.len()];

        for i in 0..toggles.len() {
            for &x in &toggles[i] {
                expressions[x as usize] += presses[i];
            }
        }

        for (e, j) in expressions.into_iter().zip(jolts) {
            optimization.add_constraint(e.eq(j as f64));
        }

        let solution = optimization.solve().unwrap();

        total += presses.iter().map(|&v| solution.value(v)).sum::<f64>();
    }

    Umi::from_number(total as u128)
}
