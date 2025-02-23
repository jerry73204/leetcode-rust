//! # 1352. Product of the Last K Numbers
//!
//! [Submission](https://leetcode.com/problems/product-of-the-last-k-numbers/submissions/1553096387)

#[allow(unused)]
struct ProductOfNumbers {
    prefix_products: Vec<i32>,
}

impl ProductOfNumbers {
    #[allow(unused)]
    fn new() -> Self {
        let mut prefix_products = Vec::with_capacity(40001);
        prefix_products.push(1);

        Self { prefix_products }
    }

    #[allow(unused)]
    fn add(&mut self, num: i32) {
        let Self { prefix_products } = self;

        if num == 0 {
            prefix_products.clear();
            prefix_products.push(1);
        } else {
            let prod = prefix_products.last().unwrap() * num;
            prefix_products.push(prod);
        }
    }

    #[allow(unused)]
    fn get_product(&self, k: i32) -> i32 {
        let Self {
            ref prefix_products,
        } = *self;

        let len = prefix_products.len() - 1;
        let Some(prefix_len) = len.checked_sub(k as usize) else {
            return 0;
        };
        prefix_products[len] / prefix_products[prefix_len]
    }
}
