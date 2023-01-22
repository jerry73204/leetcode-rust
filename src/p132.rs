use crate::Solution;
use std::{cmp, iter, ops::Range};

impl Solution {
    pub fn min_cut(input: String) -> i32 {
        let chars: Vec<char> = input.chars().collect();
        let mut cache = vec![None; chars.len() + 1];
        solve_recursive(&chars, chars.len() - 1, &mut cache).unwrap() as i32
    }
}

fn solve_recursive(
    input: &[char],
    mut cut_limit: usize,
    cache: &mut Vec<Option<usize>>,
) -> Option<usize> {
    // If input string is empty or has a singel character, return zero
    // cut.
    if input.len() <= 1 {
        return Some(0);
    }

    // Return cached answer if it exists
    if let Some(n_cuts) = cache[input.len()] {
        return (n_cuts <= cut_limit).then(|| n_cuts);
    }

    // If cut limit is zero, return zero if the input itself is a
    // palindrome. Otherwise, return None.
    if cut_limit == 0 {
        let len = input.len();
        let half_len = len / 2;
        let lhs = &input[..half_len];
        let rhs = &input[(len - half_len)..len];
        let is_palindrome = lhs.iter().zip(rhs.iter().rev()).all(|(&lc, &rc)| lc == rc);

        if is_palindrome {
            cache[input.len()] = Some(0);
            return Some(0);
        } else {
            return None;
        }
    }

    // Match the input with its reversed suffix.
    let matcher = Matcher::new(input.iter().rev().copied());
    let mut matches = matcher
        .search_all_partial(input.iter().copied())
        .filter(|range| {
            let half_len = (input.len() - range.start + 1) / 2;
            range.start + half_len <= range.end
        });

    // Specifically check the 1st match. If the 1st match starts from
    // 0th char, the input itself is a palindrome. Then, return zero
    // directly.
    let first_range = matches.next().unwrap();
    if first_range.start == 0 {
        cache[input.len()] = Some(0);
        return Some(0);
    }

    // Find all possible trailing palindromes. For every trailing
    // palindrome, strip the trailing palindrome and compute min cuts
    // on remaing prefix.
    let mut min_cuts = None;

    for range in iter::once(first_range).chain(matches) {
        // Strip trailing palindromes.
        let remain = &input[0..range.start];

        // Compute min cuts on remaining prefix. If the recursive call
        // returns None, it reached to the cut limit and it is
        // ignored.
        let remain_cuts = match solve_recursive(remain, cut_limit - 1, cache) {
            Some(cuts) => cuts,
            None => continue,
        };
        let n_cuts = remain_cuts + 1;

        // Update cut limit
        cut_limit = cmp::min(cut_limit, n_cuts);

        // Update min cuts so far
        match &mut min_cuts {
            Some(min_cuts) => {
                *min_cuts = cmp::min(*min_cuts, n_cuts);
            }
            None => min_cuts = Some(n_cuts),
        }
    }

    if let Some(min_cuts) = min_cuts {
        cache[input.len()] = Some(min_cuts);
    }

    min_cuts
}

pub struct Matcher {
    states: Vec<State>,
    last_fallback: usize,
}

impl Matcher {
    pub fn new<I>(chars: I) -> Self
    where
        I: IntoIterator<Item = char>,
    {
        let mut chars = chars.into_iter();
        let mut states = vec![];
        let mut fallback = 0;

        if let Some(symbol) = chars.next() {
            states.push(State { symbol, fallback });
        } else {
            return Self {
                states,
                last_fallback: fallback,
            };
        }

        for symbol in chars {
            states.push(State { symbol, fallback });

            fallback = loop {
                let fallback_state = &states[fallback];
                if fallback_state.symbol == symbol {
                    break fallback + 1;
                } else if fallback != 0 {
                    fallback = fallback_state.fallback;
                } else {
                    break 0;
                }
            };
        }

        Self {
            states,
            last_fallback: fallback,
        }
    }

    pub fn search<I>(&self, chars: I) -> Option<Range<usize>>
    where
        I: IntoIterator<Item = char>,
    {
        if self.states.is_empty() {
            return Some(0..0);
        }

        let chars = chars.into_iter();
        let mut offset = 0;
        let mut len = 0;

        for symbol in chars {
            len += 1;

            offset = loop {
                let state = &self.states[offset];

                if state.symbol == symbol {
                    break offset + 1;
                } else if offset != 0 {
                    offset = state.fallback;
                } else {
                    break 0;
                }
            };

            if offset == self.states.len() {
                return Some((len - offset)..len);
            }
        }

        None
    }

    pub fn search_all<I>(&self, chars: I) -> SearchAll<'_, impl Iterator<Item = char>>
    where
        I: IntoIterator<Item = char>,
    {
        SearchAll {
            len: 0,
            offset: 0,
            chars: Some(chars.into_iter()),
            matcher: self,
        }
    }

    pub fn search_all_partial<I>(
        &self,
        chars: I,
    ) -> SearchAllPartial<'_, impl Iterator<Item = char>>
    where
        I: IntoIterator<Item = char>,
    {
        SearchAllPartial {
            len: 0,
            offset: 0,
            chars: Some(chars.into_iter()),
            matcher: self,
        }
    }
}

struct State {
    symbol: char,
    fallback: usize,
}

pub struct SearchAll<'a, I>
where
    I: Iterator<Item = char>,
{
    len: usize,
    offset: usize,
    chars: Option<I>,
    matcher: &'a Matcher,
}

impl<'a, I> Iterator for SearchAll<'a, I>
where
    I: Iterator<Item = char>,
{
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let chars = match &mut self.chars {
            Some(chars) => chars,
            None => return None,
        };

        if self.matcher.states.is_empty() {
            let range = self.len..self.len;

            if chars.next().is_some() {
                self.len += 1;
            } else {
                self.chars = None;
            }
            return Some(range);
        }

        let states = &self.matcher.states;

        for symbol in chars {
            self.len += 1;

            self.offset = loop {
                let state = &states[self.offset];

                if state.symbol == symbol {
                    break self.offset + 1;
                } else if self.offset != 0 {
                    self.offset = state.fallback;
                } else {
                    break 0;
                }
            };

            if self.offset == states.len() {
                let range = (self.len - self.offset)..self.len;
                self.offset = self.matcher.last_fallback;
                return Some(range);
            }
        }

        self.chars = None;
        None
    }
}

pub struct SearchAllPartial<'a, I>
where
    I: Iterator<Item = char>,
{
    len: usize,
    offset: usize,
    chars: Option<I>,
    matcher: &'a Matcher,
}

impl<'a, I> Iterator for SearchAllPartial<'a, I>
where
    I: Iterator<Item = char>,
{
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let states = &self.matcher.states;

        let chars = match &mut self.chars {
            Some(chars) => chars,
            None => {
                let range = if self.offset != 0 {
                    let range = (self.len - self.offset)..self.len;
                    self.offset = states[self.offset].fallback;
                    Some(range)
                } else {
                    None
                };
                return range;
            }
        };

        if self.matcher.states.is_empty() {
            let range = self.len..self.len;

            if chars.next().is_some() {
                self.len += 1;
            } else {
                self.chars = None;
            }
            return Some(range);
        }

        for symbol in chars {
            self.len += 1;

            {
                let state = &states[self.offset];

                if state.symbol == symbol {
                    self.offset += 1;
                } else {
                    let range = (self.len - 1 - self.offset)..(self.len - 1);
                    self.offset = state.fallback;

                    self.offset = loop {
                        let state = &states[self.offset];

                        if state.symbol == symbol {
                            break self.offset + 1;
                        } else if self.offset != 0 {
                            self.offset = state.fallback;
                        } else {
                            break 0;
                        }
                    };

                    return Some(range);
                }
            }

            if self.offset == states.len() {
                let range = (self.len - self.offset)..self.len;
                self.offset = self.matcher.last_fallback;
                return Some(range);
            }
        }

        self.chars = None;

        if self.offset != 0 {
            let range = (self.len - self.offset)..self.len;
            self.offset = states[self.offset].fallback;
            Some(range)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Matcher;
    use crate::Solution;

    #[test]
    fn p132_test() {
        check("aaa", 0);
        check("aab", 1);
        check("a", 0);
        check("ab", 1);
        check("abc", 2);
        check("cacbab", 1);
        check("apjesgpsxoeiokmqmfgvjslcjukbqxpsobyhjpbgdfruqdkeiszrlmtwgfxyfostpqczidfljwfbbrflkgdvtytbgqalguewnhvvmcgxboycffopmtmhtfizxkmeftcucxpobxmelmjtuzigsxnncxpaibgpuijwhankxbplpyejxmrrjgeoevqozwdtgospohznkoyzocjlracchjqnggbfeebmuvbicbvmpuleywrpzwsihivnrwtxcukwplgtobhgxukwrdlszfaiqxwjvrgxnsveedxseeyeykarqnjrtlaliyudpacctzizcftjlunlgnfwcqqxcqikocqffsjyurzwysfjmswvhbrmshjuzsgpwyubtfbnwajuvrfhlccvfwhxfqthkcwhatktymgxostjlztwdxritygbrbibdgkezvzajizxasjnrcjwzdfvdnwwqeyumkamhzoqhnqjfzwzbixclcxqrtniznemxeahfozp", 452);
    }

    fn check(text: &str, expect: i32) {
        assert_eq!(Solution::min_cut(text.to_string()), expect);
    }

    #[test]
    fn p132_matcher_test() {
        // search()
        {
            let matcher = Matcher::new("".chars());
            assert_eq!(matcher.search("a".chars()), Some(0..0));
        }

        {
            let matcher = Matcher::new("aaa".chars());
            assert_eq!(matcher.search("baaa".chars()), Some(1..4));
        }

        // search_all()
        {
            let matcher = Matcher::new("a".chars());
            let mut search = matcher.search_all("aaa".chars());
            assert_eq!(search.next(), Some(0..1));
            assert_eq!(search.next(), Some(1..2));
            assert_eq!(search.next(), Some(2..3));
            assert_eq!(search.next(), None);
        }

        {
            let matcher = Matcher::new("aa".chars());
            let mut search = matcher.search_all("aaa".chars());
            assert_eq!(search.next(), Some(0..2));
            assert_eq!(search.next(), Some(1..3));
            assert_eq!(search.next(), None);
        }

        {
            let matcher = Matcher::new("abc".chars());
            let mut search = matcher.search_all("ababccabca".chars());
            assert_eq!(search.next(), Some(2..5));
            assert_eq!(search.next(), Some(6..9));
            assert_eq!(search.next(), None);
        }

        {
            let matcher = Matcher::new("".chars());
            let mut search = matcher.search_all("".chars());
            assert_eq!(search.next(), Some(0..0));
            assert_eq!(search.next(), None);
        }

        {
            let matcher = Matcher::new("".chars());
            let mut search = matcher.search_all("abc".chars());
            assert_eq!(search.next(), Some(0..0));
            assert_eq!(search.next(), Some(1..1));
            assert_eq!(search.next(), Some(2..2));
            assert_eq!(search.next(), Some(3..3));
            assert_eq!(search.next(), None);
        }

        // search_all_partial()
        {
            let matcher = Matcher::new("a".chars());
            let mut search = matcher.search_all_partial("aaa".chars());
            assert_eq!(search.next(), Some(0..1));
            assert_eq!(search.next(), Some(1..2));
            assert_eq!(search.next(), Some(2..3));
            assert_eq!(search.next(), None);
        }

        {
            let matcher = Matcher::new("aa".chars());
            let mut search = matcher.search_all_partial("aaa".chars());
            assert_eq!(search.next(), Some(0..2));
            assert_eq!(search.next(), Some(1..3));
            assert_eq!(search.next(), Some(2..3));
            assert_eq!(search.next(), None);
        }

        {
            let matcher = Matcher::new("abc".chars());
            let mut search = matcher.search_all_partial("ababab".chars());
            assert_eq!(search.next(), Some(0..2));
            assert_eq!(search.next(), Some(2..4));
            assert_eq!(search.next(), Some(4..6));
            assert_eq!(search.next(), None);
        }

        {
            let matcher = Matcher::new("aaa".chars());
            let mut search = matcher.search_all_partial("aaa".chars());
            assert_eq!(search.next(), Some(0..3));
            assert_eq!(search.next(), Some(1..3));
            assert_eq!(search.next(), Some(2..3));
            assert_eq!(search.next(), None);
        }

        {
            let matcher = Matcher::new("abc".chars());
            let mut search = matcher.search_all_partial("a".chars());
            assert_eq!(search.next(), Some(0..1));
            assert_eq!(search.next(), None);
        }

        {
            let matcher = Matcher::new("aaaa".chars());
            let mut search = matcher.search_all_partial("aa".chars());
            assert_eq!(search.next(), Some(0..2));
            assert_eq!(search.next(), Some(1..2));
            assert_eq!(search.next(), None);
        }

        {
            let matcher = Matcher::new("".chars());
            let mut search = matcher.search_all_partial("".chars());
            assert_eq!(search.next(), Some(0..0));
            assert_eq!(search.next(), None);
        }

        {
            let matcher = Matcher::new("".chars());
            let mut search = matcher.search_all_partial("abc".chars());
            assert_eq!(search.next(), Some(0..0));
            assert_eq!(search.next(), Some(1..1));
            assert_eq!(search.next(), Some(2..2));
            assert_eq!(search.next(), Some(3..3));
            assert_eq!(search.next(), None);
        }
    }
}
