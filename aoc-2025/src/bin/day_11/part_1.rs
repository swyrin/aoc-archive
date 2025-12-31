use crate::input::Input;
use aoc_lib::ext::petgraph::algo::all_simple_paths;
use aoc_lib::ext::petgraph::graph::DiGraph;
use aoc_lib::ext::petgraph::graph::NodeIndex;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;
use std::collections::HashMap;
use std::hash::RandomState;
#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
    expected_out = 5
)]
pub fn part_1(input: Input, _is_sample: bool) -> Umi {
    let mut g = DiGraph::<String, ()>::new();
    let mut node_index: HashMap<String, NodeIndex> = HashMap::new();
    for entry in &input.entries {
        let from = entry.from.clone();
        if !node_index.contains_key(&from) {
            let index = g.add_node(from.clone());
            node_index.insert(from.clone(), index);
        }
        let parent_index = *node_index.get(&from).unwrap();
        for neighbor in entry.neighbors.clone() {
            if !node_index.contains_key(&neighbor) {
                let index = g.add_node(neighbor.clone());
                node_index.insert(neighbor.clone(), index);
            }
            let child_index = *node_index.get(&neighbor).unwrap();
            g.add_edge(parent_index, child_index, ());
        }
    }
    let you = *node_index.get("you").unwrap();
    let out = *node_index.get("out").unwrap();
    let paths =
        all_simple_paths::<Vec<_>, _, RandomState>(&g, you, out, 0, None).collect::<Vec<_>>();
    Umi::from_number(paths.len() as u128)
}
