use crate::Solution;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut order = vec![usize::MAX; 1001];

        for (rank, val) in arr2.into_iter().enumerate() {
            order[val as usize] = rank;
        }

        arr1.sort_unstable_by_key(|&val| (order[val as usize], val));

        arr1
    }
}
