use crate::Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut states: [Option<Result<usize, ()>>; 26] = [None; 26];

        for (idx, ch) in s.bytes().enumerate() {
            let code = (ch - b'a') as usize;

            match &mut states[code] {
                state @ None => {
                    *state = Some(Ok(idx));
                }
                state @ Some(Ok(_)) => {
                    *state = Some(Err(()));
                }
                Some(Err(())) => {}
            }
        }

        let min_idx = IntoIterator::into_iter(states)
            .into_iter()
            .filter_map(|state| state?.ok())
            .min();
        match min_idx {
            Some(idx) => idx as i32,
            None => -1,
        }
    }
}
