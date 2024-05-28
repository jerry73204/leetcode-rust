use crate::Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n_subsets = 2usize.pow(nums.len() as u32);

        (0..n_subsets)
            .map(|choice| {
                let subset: Vec<_> = (0..nums.len())
                    .filter(|&nth| {
                        let bit = 1 << nth;
                        (choice & bit) != 0
                    })
                    .map(|nth| nums[nth])
                    .collect();
                subset
            })
            .collect()
    }
}
