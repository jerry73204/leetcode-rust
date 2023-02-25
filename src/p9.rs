use crate::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        if x <= 9 {
            return true;
        }

        let text = x.to_string();
        let chars = text.as_bytes();

        let len = chars.len();
        let half = len / 2;

        let lchunk = &chars[0..half];
        let rchunk = &chars[(len - half)..len];

        let liter = lchunk.iter();
        let riter = rchunk.iter().rev();

        liter.zip(riter).all(|(lchar, rchar)| lchar == rchar)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p9_test() {
        check(121, true);
        check(-121, false);
        check(10, false);
        check(101, true);
        check(101, true);
        check(0, true);
        check(1, true);
    }

    fn check(input: i32, expect: bool) {
        assert_eq!(Solution::is_palindrome(input), expect);
    }
}
