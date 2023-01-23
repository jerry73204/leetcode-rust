use crate::Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut people = vec![Person::default(); n as usize];

        trust.iter().for_each(|edge| {
            let (truster, trustee) = match edge.as_slice() {
                &[a, b] => ((a - 1) as usize, (b - 1) as usize),
                _ => unreachable!(),
            };

            people[truster].is_truster = true;
            people[trustee].num_trusts += 1;
        });

        people
            .iter()
            .enumerate()
            .find(|(_, person)| {
                let Person {
                    is_truster,
                    num_trusts,
                } = **person;
                !is_truster && num_trusts == (n - 1) as usize
            })
            .map(|(id, _)| (id + 1) as i32)
            .unwrap_or(-1)
    }
}

#[derive(Clone, Default)]
struct Person {
    pub is_truster: bool,
    pub num_trusts: usize,
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p997_test() {
        check(2, vec![vec![1, 2]], 2);
        check(3, vec![vec![1, 3], vec![2, 3]], 3);
        check(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]], -1);
    }

    fn check(n: i32, trust: Vec<Vec<i32>>, expect: i32) {
        assert_eq!(Solution::find_judge(n, trust), expect);
    }
}
