use crate::Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = tickets[k];

        let (prefix, suffix) = tickets.split_at(k + 1);

        let prefix_sum: i32 = prefix.iter().map(|&tk| tk.min(n)).sum();
        let suffix_sum: i32 = suffix.iter().map(|&tk| tk.min(n - 1)).sum();

        prefix_sum + suffix_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2073_test() {
        check(vec![2, 3, 2], 2, 6);
        check(vec![5, 1, 1, 1], 0, 8);
    }

    fn check(tickets: Vec<i32>, k: i32, expect: i32) {
        assert_eq!(Solution::time_required_to_buy(tickets, k), expect);
    }
}
