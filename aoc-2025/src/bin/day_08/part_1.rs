use crate::input::Input;
use aoc_lib::ext::petgraph;
use aoc_lib::ext::petgraph::graph::UnGraph;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;
use std::collections::BTreeMap;

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
    expected_out = 40
)]
pub fn part_1(input: Input, is_sample: bool) -> Umi {
    let points = input.points;
    let mut edges = BTreeMap::<isize, (u32, u32)>::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let a = &points[i];
            let b = &points[j];
            let d = a.distance_from(b);
            edges.insert(d, (i as u32, j as u32));
        }
    }

    let graph =
        UnGraph::<u32, ()>::from_edges(edges.values().take(if is_sample { 10 } else { 1000 }));

    let mut sccs: Vec<usize> = petgraph::algo::kosaraju_scc(&graph)
        .iter()
        .map(|x| x.len())
        .collect();

    sccs.sort_unstable();

    Umi::from_number(sccs.iter().rev().take(3).product::<usize>() as u128)
}
