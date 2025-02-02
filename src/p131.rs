use crate::Solution;
use std::{collections::HashSet, ops::Range};

impl Solution {
    pub fn partition(input: String) -> Vec<Vec<String>> {
        let chars: Vec<char> = input.chars().collect();

        // 1-sized palindromes
        let pld1: HashSet<Range<usize>> =
            (0..input.len()).map(|start| start..(start + 1)).collect();

        // 2-sized palindromes
        let pld2: HashSet<Range<usize>> = input
            .char_indices()
            .zip(input.chars().skip(1))
            .filter(|&((_start, lsymbol), rsymbol)| (lsymbol == rsymbol))
            .map(|((start, _lsymbol), _rsymbol)| start..(start + 2))
            .collect();

        // Generate a stack of palindromes indexed by palindrome length
        let pld_stack: Vec<HashSet<Range<usize>>> = {
            let mut pld_stack = vec![pld1, pld2];

            let extend_range = |range: &Range<usize>| -> Option<Range<usize>> {
                let start = range.start.checked_sub(1)?;
                let end = range.end + 1;
                let is_pld = chars[start] == *chars.get(end - 1)?;
                is_pld.then(|| start..end)
            };

            loop {
                let stack_len = pld_stack.len();
                let pld1 = &pld_stack[stack_len - 2];
                let pld2 = &pld_stack[stack_len - 1];

                let pld3: HashSet<Range<usize>> = pld1.iter().filter_map(extend_range).collect();
                let pld4: HashSet<Range<usize>> = pld2.iter().filter_map(extend_range).collect();

                if pld3.is_empty() && pld4.is_empty() {
                    break;
                }

                pld_stack.push(pld3);
                pld_stack.push(pld4);
            }

            pld_stack
        };

        // Build palindrome look-up table
        let pld_map: Vec<Vec<usize>> = {
            let mut pld_map = vec![vec![]; chars.len()];
            pld_stack.into_iter().flatten().for_each(|range| {
                pld_map[range.start].push(range.end);
            });
            pld_map
        };

        let mut substrs = vec![];
        let mut answers = vec![];
        solve_recursive(&chars, 0, &pld_map, &mut substrs, &mut answers);

        answers
    }
}

fn solve_recursive(
    chars: &[char],
    start: usize,
    pld_map: &[Vec<usize>],
    substrs: &mut Vec<String>,
    answers: &mut Vec<Vec<String>>,
) {
    if start == chars.len() {
        answers.push(substrs.clone());
        return;
    }

    for &end in &pld_map[start] {
        let substr: String = chars[start..end].iter().copied().collect();
        substrs.push(substr);
        solve_recursive(chars, end, pld_map, substrs, answers);
        substrs.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p131_test() {
        check("aab", &[&["a", "a", "b"], &["aa", "b"]]);
        check("a", &[&["a"]]);
    }

    fn check(input: &str, expect: &[&[&str]]) {
        assert_eq!(Solution::partition(input.to_string()), expect);
    }
}
