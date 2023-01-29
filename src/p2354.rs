use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as u32;

        // Eliminate duplications
        let set: HashSet<i32> = nums.into_iter().collect();

        // Count bits for each number and sort them by the n. of bits
        let mut one_counts: Vec<u32> = set.into_iter().map(|num| num.count_ones()).collect();
        one_counts.sort_unstable();

        // dbg!(&one_counts);

        // Locate the indices where bit_count * 2 >= k and count >= k
        let half_k_index = {
            let thresh = (k + 1) / 2;
            binary_search(&one_counts, &thresh)
        };
        let k_index = binary_search(&one_counts, &k);

        // Count self-pairs
        let n_self_pairs = (one_counts.len() - half_k_index) as i64;

        // Count dinstinct bit_count pairs where each bit_count * 2 >= k.
        let n_great_pairs = {
            let n = one_counts.len() - half_k_index;
            match n.checked_sub(1) {
                Some(n_sub1) => n as i64 * n_sub1 as i64,
                None => 0,
            }
        };

        // Split the bit counts array at k/2 and k.
        let (non_large_counts, large_counts) = one_counts.split_at(k_index);
        let (small_counts, mid_counts) = non_large_counts.split_at(half_k_index);

        // Count dinstinct bit_count pairs (a, b) or (b, a) where a >=
        // k and b*2 < k.
        let n_mid_pairs = large_counts.len() as i64 * small_counts.len() as i64 * 2;

        // Count dinstinct bit_count pairs (a, b) or (b, a) where a*2
        // >= k, a < k and b*2 < k.
        let n_small_pairs = {
            let small_runs = uniq(small_counts);
            let mid_runs = uniq(mid_counts);
            let mut mid_runs = mid_runs.as_slice();
            let mut count = 0;

            // dbg!(&small_runs, mid_runs);

            for small_run in small_runs.into_iter().rev() {
                let thresh = k - small_run.value;
                let index = binary_search_by_key(mid_runs, &thresh, |run| run.value);

                let mid_run = match mid_runs.get(index) {
                    Some(mid_run) => mid_run,
                    None => break,
                };

                // dbg!(&small_run, &mid_run);

                count += small_run.count as i64 * mid_run.suffix_count as i64 * 2;
                mid_runs = &mid_runs[index..];
            }

            count
        };

        // dbg!(n_self_pairs, n_great_pairs, n_mid_pairs, n_small_pairs);

        n_self_pairs + n_great_pairs + n_mid_pairs + n_small_pairs
    }
}

fn uniq(mut slice: &[u32]) -> Vec<Run> {
    let mut output = vec![];

    while let Some((&first, non_first)) = slice.split_first() {
        let index = non_first
            .iter()
            .zip(1..)
            .find(|(&val, _)| val != first)
            .map(|(_, index)| index)
            .unwrap_or(slice.len());
        let (uniq_run, remain) = slice.split_at(index);
        output.push(Run {
            value: first,
            count: uniq_run.len(),
            suffix_count: slice.len(),
        });
        slice = remain;
    }

    output
}

fn binary_search<T>(slice: &[T], x: &T) -> usize
where
    T: Ord,
{
    let mut index = match slice.binary_search(x) {
        Ok(index) => index,
        Err(index) => return index,
    };

    loop {
        match index.checked_sub(1) {
            Some(sub1) if &slice[sub1] == x => index = sub1,
            _ => break index,
        }
    }
}

fn binary_search_by_key<T, B, F>(slice: &[T], x: &B, mut key_fn: F) -> usize
where
    B: Ord,
    F: FnMut(&T) -> B,
{
    let mut index = match slice.binary_search_by_key(x, &mut key_fn) {
        Ok(index) => index,
        Err(index) => return index,
    };

    loop {
        match index.checked_sub(1) {
            Some(sub1) if key_fn(&slice[sub1]) == *x => index = sub1,
            _ => break index,
        }
    }
}

#[derive(Debug)]
struct Run {
    pub value: u32,
    pub count: usize,
    pub suffix_count: usize,
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2354_test() {
        check(vec![1, 2, 3, 1], 3, 5);
        check(vec![5, 1, 1], 10, 0);
        check(vec![1, 2, 4, 3, 5, 6, 7, 23, 15, 31], 5, 58);
        check(
            vec![
                728529570, 148598210, 443495301, 242440979, 911286922, 30943499, 308153171,
                740783964, 650762393, 446086847, 930075805, 540854604, 860107615, 720236770,
                713877237, 439284081, 919006384, 14172738, 355796771, 702237697, 118921380,
                944192487, 925529720, 128027967, 184720050, 726373526, 930984918, 881444550,
                189058098, 747863603, 40918608, 855649500, 145087862, 607553557, 920028637,
                394198989, 881013232, 231822702, 593710676, 780673945, 416799460, 250417625,
                768553105, 490645288, 241330019, 929728537, 775780972, 989452167, 267495679,
                273474590, 359711155, 218140919, 777776619, 461348777, 122492682, 669280307,
                778184867, 92518531, 467007745, 315627725, 148580269, 708334776, 392495635,
                336694364, 318749933, 377772674, 350000586, 358006091, 536729773, 79165845,
                372926988, 287010541, 379641115, 152476089, 144269981, 701169540, 853361750,
                215995314, 826135104, 945533785, 617004814, 422394140, 302034694, 859579587,
            ],
            43,
            19,
        );
    }

    fn check(nums: Vec<i32>, k: i32, expect: i64) {
        assert_eq!(Solution::count_excellent_pairs(nums, k), expect);
    }
}
