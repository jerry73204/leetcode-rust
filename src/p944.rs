use crate::Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut row_vec: Vec<_> = strs.iter().map(|row| row.chars()).collect();
        let mut deleted_column_count = 0;

        loop {
            let mut column = row_vec.iter_mut().map(|chars| chars.next());
            let first_ch = match column.next().unwrap() {
                Some(ch) => ch,
                None => break,
            };

            let mut check_iter = column.scan(first_ch, |prev_ch, curr_ch| {
                let curr_ch = curr_ch.unwrap();

                Some(if *prev_ch <= curr_ch {
                    *prev_ch = curr_ch;
                    true
                } else {
                    false
                })
            });
            let is_lex_order = check_iter.all(|is_lex_order| is_lex_order);

            // Consume remaining chars
            check_iter.for_each(|_| {});

            if !is_lex_order {
                deleted_column_count += 1;
            }
        }

        deleted_column_count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p944_test() {
        check(&["cba", "daf", "ghi"], 1);
        check(&["a", "b"], 0);
        check(&["zyx", "wvu", "tsr"], 3);
        check(&["egguij", "gjsnnk", "lstgon", "ztzrqv"], 1);
    }

    fn check(strs: &[&str], expect: i32) {
        let strs: Vec<_> = strs.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::min_deletion_size(strs), expect);
    }
}
