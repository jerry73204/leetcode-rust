use crate::Solution;

impl Solution {
    pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
        let group_size = group_size as usize;

        let mut uniq: Vec<(i32, u32)> = {
            hand.sort_unstable();

            let mut uniq = Vec::with_capacity(hand.len());

            let mut iter = hand.into_iter();
            let first = iter.next().unwrap();
            uniq.push((first, 1));

            let mut last = uniq.last_mut().unwrap();

            for curr in iter {
                if curr == last.0 {
                    last.1 += 1;
                } else {
                    uniq.push((curr, 1));
                    last = uniq.last_mut().unwrap();
                }
            }

            uniq
        };

        if uniq.len() < group_size {
            return false;
        }

        for i1 in 0..=(uniq.len() - group_size) {
            let (first_val, first_cnt) = &mut uniq[i1];

            let mut dec = 0;
            std::mem::swap(&mut dec, first_cnt);
            if dec == 0 {
                continue;
            }

            let expect_values = (*first_val + 1)..(*first_val + group_size as i32);
            let tail = &mut uniq[(i1 + 1)..(i1 + group_size)];

            for (expect, (val, count)) in expect_values.zip(tail) {
                if *val != expect {
                    return false;
                }

                match count.checked_sub(dec) {
                    Some(reduced_count) => {
                        *count = reduced_count;
                    }
                    None => return false,
                }
            }
        }

        uniq.iter()
            .rev()
            .take(group_size - 1)
            .all(|(_, cnt)| *cnt == 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p846_test() {
        check(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3, true);
        check(vec![1, 2, 3, 4, 5], 4, false);
        check(vec![8, 10, 12], 3, false);
        check(vec![1, 2, 2, 3, 3, 3, 4, 4, 5], 4, false);
    }

    fn check(hand: Vec<i32>, group_size: i32, expect: bool) {
        assert_eq!(Solution::is_n_straight_hand(hand, group_size), expect);
    }
}
