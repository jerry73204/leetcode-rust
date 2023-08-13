use crate::Solution;
use std::cmp::Ordering::*;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        // The index of the number after the nearest zero.
        let mut product_after_zero: i32 = 1;
        let mut max_product = None;

        // The product of numbers after the nearest zero until the 1st
        // negative number.
        let mut product_until_1st_neg = None;

        // The product of numbers after the nearest zero after the
        // last negative number.
        let mut product_since_last_neg = None;

        let mut pos_count = 0;
        let mut neg_count = 0;

        for &val in &nums {
            // max_val = max_val.max(val);

            match val.cmp(&0) {
                Less => {
                    product_after_zero *= -val;
                    neg_count += 1;
                    product_since_last_neg = Some(-val);
                    if product_until_1st_neg.is_none() {
                        product_until_1st_neg = Some(product_after_zero);
                    }
                }
                Equal => {
                    // Update max product.
                    let candidate = match (pos_count, neg_count) {
                        (0, 0) => Some(0),
                        (0, 1) => Some(0),
                        _ => {
                            let candidate = if neg_count & 1 != 0 {
                                let case1 = product_after_zero / product_until_1st_neg.unwrap();
                                let case2 = product_after_zero / product_since_last_neg.unwrap();
                                case1.max(case2)
                            } else {
                                product_after_zero
                            };
                            Some(candidate)
                        }
                    };
                    max_product = max_product.max(candidate);

                    product_until_1st_neg = None;
                    product_since_last_neg = None;
                    product_after_zero = 1;
                    pos_count = 0;
                    neg_count = 0;
                }
                Greater => {
                    pos_count += 1;
                    product_after_zero *= val;
                    if let Some(product_since_last_neg) = &mut product_since_last_neg {
                        *product_since_last_neg *= val;
                    }
                }
            }
        }

        // Update max product
        let candidate = match (pos_count, neg_count) {
            (0, 0) => None, // no-op
            (0, 1) => Some(-product_after_zero),
            _ => {
                let candidate = if neg_count & 1 != 0 {
                    let case1 = product_after_zero / product_until_1st_neg.unwrap();
                    let case2 = product_after_zero / product_since_last_neg.unwrap();
                    case1.max(case2)
                } else {
                    product_after_zero
                };
                Some(candidate)
            }
        };
        max_product = max_product.max(candidate);

        max_product.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p152_test() {
        check(vec![2, 3, -2, 4], 6);
        check(vec![-2, 0, -1], 0);
    }

    fn check(nums: Vec<i32>, expect: i32) {
        assert_eq!(Solution::max_product(nums), expect);
    }
}
