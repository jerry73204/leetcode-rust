use crate::Solution;

impl Solution {
    pub fn check_valid_string(input: String) -> bool {
        let input = input.as_bytes();
        let len = input.len();

        if len == 1 {
            return input[0] == b'*';
        }

        struct Tab {
            len: usize,
            tab: Vec<bool>,
        }

        impl Tab {
            fn new(len: usize) -> Self {
                let tab = vec![false; (len + 1).pow(2)];
                Self { len, tab }
            }

            fn get(&self, start: usize, end: usize) -> bool {
                *self.tab.get(start * (self.len + 1) + end).unwrap()
            }

            fn get_mut(&mut self, start: usize, end: usize) -> &mut bool {
                self.tab.get_mut(start * (self.len + 1) + end).unwrap()
            }
        }

        let mut tab = Tab::new(len);

        (0..=len).for_each(|idx| {
            *tab.get_mut(idx, idx) = true;
        });

        input
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == b'*')
            .for_each(|(idx, _)| {
                *tab.get_mut(idx, idx + 1) = true;
            });

        for sub_len in 2..=len {
            'inner: for start in 0..=(len - sub_len) {
                let end = start + sub_len;

                match (input[start], input[end - 1]) {
                    (b')', _) | (_, b'(') => {
                        continue 'inner;
                    }
                    _ => {
                        // case 1
                        if tab.get(start + 1, end - 1) {
                            *tab.get_mut(start, end) = true;

                            continue 'inner;
                        }
                    }
                }

                // case 2
                for med in (start + 1)..end {
                    if tab.get(start, med) && tab.get(med, end) {
                        *tab.get_mut(start, end) = true;

                        continue 'inner;
                    }
                }
            }
        }

        tab.get(0, len)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p678_test() {
        check("()", true);
        check("(*)", true);
        check("(*))", true);
        check("**", true);
        check("***", true);
        check("***********************", true);
        check("*", true);
        check(")", false);
        check("(", false);
        check("*", true);
        check(")(", false);
        check("*(", false);
        check("*)", true);
        check(")*", false);
        check("(*", true);
        check("*(*", true);
        check("*)*", true);
        check("(())*(*)", true);
        check("((*)*(*)", true);
        check("(((((()*)(*)*))())())(()())())))((**)))))(()())()", false);
        check("((((()(()()()*()(((((*)()*(**(())))))(())()())(((())())())))))))(((((())*)))()))(()((*()*(*)))(*)()", true);
    }

    fn check(input: &str, expect: bool) {
        assert_eq!(Solution::check_valid_string(input.to_string()), expect);
    }
}
