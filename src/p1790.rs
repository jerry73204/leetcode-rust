use crate::Solution;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let iter1 = s1.as_bytes().iter();
        let iter2 = s2.as_bytes().iter();
        let mut zip = iter1.zip(iter2);

        let (c1, c2) = loop {
            let Some((a, b)) = zip.next() else {
                return true;
            };

            if a != b {
                break (a, b);
            }
        };

        loop {
            let Some((a, b)) = zip.next() else {
                return false;
            };

            if a != b {
                if a == c2 && b == c1 {
                    break;
                } else {
                    return false;
                }
            }
        }

        for (a, b) in zip {
            if a != b {
                return false;
            }
        }

        true
    }
}
