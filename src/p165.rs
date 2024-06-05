use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let ord_to_i32 = |ord| match ord {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        let is_zero_version = |ver: &[u8]| ver.iter().all(|c| [b'0', b'.'].contains(c));

        let mut v1 = version1.as_bytes();
        let mut v2 = version2.as_bytes();

        let ord = loop {
            let (comp1, remain1) = split_at_dot(v1);
            let (comp2, remain2) = split_at_dot(v2);

            let ord = compare_component(comp1, comp2);
            if ord != Ordering::Equal {
                break ord;
            }

            match (remain1, remain2) {
                (None, None) => break Ordering::Equal,
                (None, Some(remain2)) => {
                    let is_zero = is_zero_version(remain2);
                    break if is_zero {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    };
                }
                (Some(remain1), None) => {
                    let is_zero = is_zero_version(remain1);
                    break if is_zero {
                        Ordering::Equal
                    } else {
                        Ordering::Greater
                    };
                }
                (Some(remain1), Some(remain2)) => {
                    assert!(v1.len() > remain1.len());
                    v1 = remain1;
                    v2 = remain2;
                }
            }
        };

        ord_to_i32(ord)
    }
}

fn split_at_dot(s: &[u8]) -> (&[u8], Option<&[u8]>) {
    match s.iter().enumerate().find(|(_ix, c)| **c == b'.') {
        Some((ix, _)) => {
            let (p, n) = s.split_at(ix);
            (p, Some(&n[1..]))
        }
        None => (s, None),
    }
}

fn compare_component(c1: &[u8], c2: &[u8]) -> Ordering {
    let len1 = c1.len();
    let len2 = c2.len();

    if let Some(diff) = len1.checked_sub(len2) {
        let (p1, n1) = c1.split_at(diff);

        if p1.iter().any(|&c| c != b'0') {
            return Ordering::Greater;
        }

        n1.cmp(c2)
    } else {
        let diff = c2.len() - c1.len();
        let (p2, n2) = c2.split_at(diff);

        if p2.iter().any(|&c| c != b'0') {
            return Ordering::Less;
        }

        c1.cmp(n2)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p165_test() {
        check("1.2", "1.10", -1);
        check("1.01", "1.001", 0);
        check("1.0", "1.0.0.0", 0);
    }

    fn check(v1: &str, v2: &str, expect: i32) {
        assert_eq!(
            Solution::compare_version(v1.to_string(), v2.to_string()),
            expect
        );
    }
}
