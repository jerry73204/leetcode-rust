use crate::Solution;
use std::{cmp::max, ops::RangeInclusive};

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        const NONE_RANGE: Option<RangeInclusive<usize>> = None;
        let mut ranges: [Option<RangeInclusive<usize>>; 26] = [NONE_RANGE; 26];

        for (ix, &b) in s.as_bytes().iter().enumerate() {
            let nth_letter = (b - b'a') as usize;
            let range = &mut ranges[nth_letter];

            match range {
                Some(range) => *range = *range.start()..=ix,
                None => *range = Some(ix..=ix),
            }
        }

        let mut ranges: Vec<_> = ranges.into_iter().flatten().collect();
        ranges.sort_unstable_by_key(|range| (*range.start(), *range.end()));

        let mut lengths = Vec::with_capacity(ranges.len());
        let mut iter = ranges.into_iter();
        let mut curr = iter.next().unwrap();

        for range in iter {
            if curr.end() >= range.start() {
                curr = *curr.start()..=max(*curr.end(), *range.end());
            } else {
                lengths.push((curr.end() - curr.start() + 1) as i32);
                curr = range;
            }
        }

        lengths.push((curr.end() - curr.start() + 1) as i32);
        lengths
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p763_test() {
        check("ababcbacadefegdehijhklij", &[9, 7, 8]);
    }

    fn check(s: &str, expect: &[i32]) {
        assert_eq!(Solution::partition_labels(s.to_string()), expect);
    }
}
