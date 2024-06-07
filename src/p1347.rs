use crate::Solution;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let s_counts = count_letters(&s);
        let t_counts = count_letters(&t);

        s_counts
            .iter()
            .zip(t_counts)
            .filter_map(|(s_cnt, t_cnt)| {
                let diff = s_cnt - t_cnt;
                (diff > 0).then(|| diff)
            })
            .sum()
    }
}

fn count_letters(input: &str) -> [i32; 26] {
    let mut counts = [0; 26];

    for &ch in input.as_bytes() {
        let ix = (ch - b'a') as usize;
        counts[ix] += 1;
    }

    counts
}
