use crate::Solution;
use std::ops::Range;

impl Solution {
    pub fn check_partitioning(input: String) -> bool {
        let chars: Vec<char> = input.chars().collect();

        let back_matcher = Matcher::new(chars.iter().rev().copied());
        let front_matcher = Matcher::new(chars.iter().copied());

        let suffix_starts: Vec<usize> = back_matcher
            .search_partial(chars.iter().copied())
            .filter(|range| range.start >= 2)
            .filter(|range| {
                let suffix_len = chars.len() - range.start;
                let half_suffix_len = suffix_len / 2;

                range.len() >= half_suffix_len
            })
            .map(|range| range.start)
            .collect();

        let prefix_ends: Vec<usize> = front_matcher
            .search_partial(chars.iter().rev().copied())
            .filter(|rev_range| rev_range.start >= 2)
            .filter(|rev_range| {
                let prefix_len = chars.len() - rev_range.start;
                let half_prefix_len = prefix_len / 2;
                rev_range.len() >= half_prefix_len
            })
            .map(|rev_range| chars.len() - rev_range.start)
            .collect();

        prefix_ends
            .iter()
            .flat_map(|&prefix_end| {
                suffix_starts
                    .iter()
                    .map(move |&suffix_start| (prefix_end..suffix_start))
            })
            .filter(|range| !range.is_empty())
            .any(|range| {
                let substr = &chars[range];
                let half_len = substr.len() / 2;
                substr
                    .iter()
                    .zip(substr.iter().rev())
                    .take(half_len)
                    .all(|(lchar, rchar)| lchar == rchar)
            })
    }
}

struct Matcher {
    states: Vec<State>,
    last_fallback: usize,
}

impl Matcher {
    pub fn new<I>(chars: I) -> Self
    where
        I: IntoIterator<Item = char>,
    {
        let mut chars = chars.into_iter();
        let mut fallback = 0;
        let mut states = vec![];

        {
            let first_symbol = match chars.next() {
                Some(symbol) => symbol,
                None => {
                    return Self {
                        states,
                        last_fallback: 0,
                    }
                }
            };
            states.push(State {
                symbol: first_symbol,
                fallback: 0,
            });
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

    pub fn search_partial<I>(&self, chars: I) -> SearchPartial<'_, impl Iterator<Item = char>>
    where
        I: IntoIterator<Item = char>,
    {
        SearchPartial {
            matcher: self,
            chars: Some(chars.into_iter()),
            offset: 0,
            len: 0,
        }
    }
}

struct State {
    symbol: char,
    fallback: usize,
}

struct SearchPartial<'a, I>
where
    I: Iterator<Item = char>,
{
    matcher: &'a Matcher,
    chars: Option<I>,
    offset: usize,
    len: usize,
}

impl<'a, I> Iterator for SearchPartial<'a, I>
where
    I: Iterator<Item = char>,
{
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            chars, offset, len, ..
        } = self;
        let Matcher {
            ref states,
            last_fallback,
        } = *self.matcher;

        let chars = match chars {
            Some(chars) => chars,
            None => {
                let range = if *offset != 0 {
                    let state = &states[*offset];
                    let range = (*len - *offset)..(*len);
                    *offset = state.fallback;
                    Some(range)
                } else {
                    None
                };

                return range;
            }
        };

        for symbol in chars {
            *len += 1;

            let state = &states[*offset];

            if state.symbol == symbol {
                *offset += 1;

                if *offset == states.len() {
                    let range = (*len - *offset)..(*len);
                    *offset = last_fallback;
                    return Some(range);
                }
            } else if *offset != 0 {
                let range = (*len - 1 - *offset)..(*len - 1);

                *offset = state.fallback;
                *offset = loop {
                    let state = &states[*offset];

                    if state.symbol == symbol {
                        break *offset + 1;
                    } else if *offset != 0 {
                        *offset = state.fallback;
                    } else {
                        break 0;
                    }
                };

                return Some(range);
            }
        }

        self.chars = None;

        if *offset != 0 {
            let state = &states[*offset];
            let range = (*len - *offset)..(*len);
            *offset = state.fallback;
            Some(range)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1745_test() {
        check("abcbdd", true);
        check("abc", true);
        check("a", false);
        check("aa", false);
        check("aaa", true);
        check("abcd", false);
    }

    fn check(input: &str, expect: bool) {
        assert_eq!(Solution::check_partitioning(input.to_string()), expect);
    }
}
