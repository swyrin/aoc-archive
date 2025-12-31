use std::{cmp, ops::RangeInclusive};

/// Merge all ranges into a minimal cover.
///
/// Due to a quirk in this implementation:
/// The function requires all ranges to be bounded-inclusive,
/// i.e. `(3..=6)` will work, `(3..)` and `(..=5)` might not.
///
/// ```
/// use aoc_lib::ext::range;
///
/// assert_eq!(range::minimal_cover(vec![(3..=5), (4..=9)]), [(3..=9)]);
/// assert_eq!(range::minimal_cover(vec![(3..=5), (3..=12), (4..=9)]), [(3..=12)]);
/// ```
pub fn minimal_cover(ranges: Vec<RangeInclusive<isize>>) -> Vec<RangeInclusive<isize>> {
    let mut range_copied = ranges.clone();
    range_copied.sort_by(|x, y| x.start().cmp(y.start()));

    let mut combined = vec![range_copied[0].clone()];

    for range in range_copied.iter().skip(1) {
        let current: RangeInclusive<isize> = range.clone();
        let j: usize = combined.len() - 1;

        // let's just assume that the best one always at the end.
        // until it isn't.
        let (current_start, current_end) = (*current.start(), *current.end());
        let (biggest_start, biggest_end) = (*combined[j].start(), *combined[j].end());

        // basically, if one's start is between the last one, extend if possible.
        // so like we have: [3, 7] and we want to merge [4, 9]
        // 4 is inside 3, so only need to check whether 9 extends the right bound.
        if biggest_start <= current_start && current_start <= biggest_end {
            combined[j] = biggest_start..=cmp::max(current_end, biggest_end);
        } else {
            combined.push(current);
        }
    }

    combined
}
