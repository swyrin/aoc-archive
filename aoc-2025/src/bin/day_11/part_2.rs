use crate::input::Input;
use aoc_lib::solution::Umi;
use aoc_macro::test_should_output;
use std::collections::{HashMap, HashSet};

fn find_paths(
    start: &str,
    connections: &HashMap<String, HashSet<String>>,
    cache: &mut HashMap<String, (usize, usize, usize, usize)>,
) -> (usize, usize, usize, usize) {
    if start == "out" {
        return (0, 0, 0, 1);
    }

    if let Some(result) = cache.get(start) {
        return *result;
    }

    let (mut dac_paths, mut fft_paths, mut dac_and_fft_paths, mut total_paths) = (0, 0, 0, 0);

    if let Some(connected_devices) = connections.get(start) {
        for connected_device in connected_devices {
            let (child_dac_paths, child_fft_paths, child_dac_and_fft_paths, child_total_paths) =
                find_paths(connected_device, connections, cache);

            dac_paths += if start == "dac" {
                child_total_paths
            } else {
                child_dac_paths
            };

            fft_paths += if start == "fft" {
                child_total_paths
            } else {
                child_fft_paths
            };

            // ("in any order" I laugh)
            dac_and_fft_paths += match start {
                "dac" => child_fft_paths,
                "fft" => child_dac_paths,
                _ => child_dac_and_fft_paths,
            };

            total_paths += child_total_paths;
        }
    }

    cache.insert(
        start.to_owned(),
        (dac_paths, fft_paths, dac_and_fft_paths, total_paths),
    );

    (dac_paths, fft_paths, dac_and_fft_paths, total_paths)
}

#[forbid(unsafe_code)]
#[test_should_output(
    input_type = Input,
    sample = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
    expected_out = 2
)]
pub fn part_2(input: Input, _is_sample: bool) -> Umi {
    let mut adj = HashMap::new();

    for entry in input.entries {
        adj.insert(entry.from, HashSet::from_iter(entry.neighbors));
    }

    Umi::from_number(find_paths("svr", &adj, &mut HashMap::new()).2 as u128)
}
