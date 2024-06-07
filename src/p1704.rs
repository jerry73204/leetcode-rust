use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let vowels: HashSet<_> =
            IntoIterator::into_iter([b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'])
                .collect();

        let s = s.as_bytes();
        let len = s.len();
        let (left, right) = s.split_at(len / 2);

        left.iter().filter(|ch| vowels.contains(ch)).count()
            == right.iter().filter(|ch| vowels.contains(ch)).count()
    }
}
