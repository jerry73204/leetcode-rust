use crate::Solution;
use std::ops::Range;

impl Solution {
    pub fn shortest_palindrome(input: String) -> String {
        if input.is_empty() {
            return "".to_string();
        }

        let input = input.as_bytes();
        let matcher = Matcher::new(input.iter().copied());

        let subpld_range = matcher
            .search_partial(input.iter().rev().copied())
            .map(|rev_range| {
                let len = input.len();
                (len - rev_range.end)..(len - rev_range.start)
            })
            .find(|range| range.start == 0)
            .unwrap();

        let (subpld, suffix) = input.split_at(subpld_range.end);
        suffix
            .iter()
            .rev()
            .chain(subpld)
            .chain(suffix)
            .map(|&byte| byte as char)
            .collect()
    }
}

struct Matcher {
    states: Vec<State>,
    last_fallback: usize,
}

impl Matcher {
    pub fn new<I>(chars: I) -> Self
    where
        I: IntoIterator<Item = u8>,
    {
        let mut chars = chars.into_iter();
        let mut states = vec![];
        let mut fallback = 0;

        match chars.next() {
            Some(symbol) => states.push(State { symbol, fallback }),
            None => {
                return Self {
                    states,
                    last_fallback: 0,
                }
            }
        };

        for symbol in chars {
            states.push(State { symbol, fallback });

            fallback = loop {
                let state = &states[fallback];

                if state.symbol == symbol {
                    break fallback + 1;
                } else if fallback != 0 {
                    fallback = state.fallback;
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

    pub fn search_partial<I>(&self, chars: I) -> SearchPartial<'_, impl Iterator<Item = u8>>
    where
        I: IntoIterator<Item = u8>,
    {
        SearchPartial {
            offset: 0,
            len: 0,
            chars: Some(chars.into_iter()),
            matcher: self,
        }
    }
}

struct State {
    symbol: u8,
    fallback: usize,
}

struct SearchPartial<'a, I>
where
    I: Iterator<Item = u8>,
{
    matcher: &'a Matcher,
    offset: usize,
    len: usize,
    chars: Option<I>,
}

impl<'a, I> Iterator for SearchPartial<'a, I>
where
    I: Iterator<Item = u8>,
{
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let Matcher {
            ref states,
            last_fallback,
        } = *self.matcher;
        let Self {
            offset, len, chars, ..
        } = self;

        let chars = match chars {
            Some(chars) => chars,
            None => {
                return if *offset != 0 {
                    let range = (*len - *offset)..(*len);
                    *offset = states[*offset].fallback;
                    Some(range)
                } else {
                    None
                };
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

        let range = (*len - *offset)..(*len);
        *offset = states[*offset].fallback;
        self.chars = None;

        Some(range)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p214_test() {
        check("aacecaaa", "aaacecaaa");
        check("abcd", "dcbabcd");
        check("a", "a");
        check("aa", "aa");
        check("abcba", "abcba");
        check("", "");
    }

    fn check(input: &str, expect: &str) {
        assert_eq!(Solution::shortest_palindrome(input.to_string()), expect);
    }
}
