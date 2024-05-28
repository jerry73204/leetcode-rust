use crate::Solution;

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let cost: Vec<i32> = s
            .iter()
            .zip(t)
            .map(|(&sb, &tb)| (sb as i32 - tb as i32).abs())
            .collect();

        let (mut curr_cost, mut end) = cost
            .iter()
            .scan(0, |acc, c| {
                *acc += c;
                Some(*acc)
            })
            .take_while(|&acc| acc <= max_cost)
            .zip(1..)
            .last()
            .unwrap_or((0, 0));

        let mut max_len = end;

        if end > 0 {
            curr_cost -= cost[0];
        } else {
            end = 1;
        }

        for start in 1..s.len() {
            while let Some(&c) = cost.get(end) {
                let new_cost = curr_cost + c;
                if new_cost <= max_cost {
                    curr_cost = new_cost;
                    end += 1;
                } else {
                    break;
                }
            }

            max_len = max_len.max(end - start);

            if start < end {
                curr_cost -= cost[start];
            } else {
                end += 1;
            }
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1208_test() {
        check("abcd", "bcdf", 3, 3);
        check("abcd", "cdef", 3, 1);
        check("abcd", "acde", 0, 1);
    }

    fn check(s: &str, t: &str, max_cost: i32, expect: i32) {
        assert_eq!(
            Solution::equal_substring(s.to_string(), t.to_string(), max_cost),
            expect
        );
    }
}
