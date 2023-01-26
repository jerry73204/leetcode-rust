use crate::Solution;
use std::{cmp, iter, mem};

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n_cities = n as usize;
        let src = src as usize;
        let dst = dst as usize;

        // Build adjacent list
        let cities: Vec<City> = iter::repeat_with(|| City { neighbors: vec![] })
            .take(n_cities)
            .collect();

        let cities = flights.iter().fold(cities, |mut cities, flight| {
            let (from, to, price) = match flight.as_slice() {
                &[from, to, price] => (from as usize, to as usize, price),
                _ => unreachable!(),
            };

            cities[to].neighbors.push(Flight { from, price });
            cities
        });

        let mut lcosts = vec![i32::MAX; n_cities];
        lcosts[src] = 0;
        let mut rcosts = vec![0; n_cities];

        for _ in 0..(k + 1) {
            for curr in 0..n_cities {
                let new_cost = cities[curr]
                    .neighbors
                    .iter()
                    .map(|neighbor| lcosts[neighbor.from].saturating_add(neighbor.price))
                    .min();

                rcosts[curr] = match new_cost {
                    Some(new_cost) => cmp::min(lcosts[curr], new_cost),
                    None => lcosts[curr],
                };
            }
            mem::swap(&mut lcosts, &mut rcosts);
        }

        let cost = lcosts[dst];
        if cost == i32::MAX {
            -1
        } else {
            cost
        }
    }
}

#[derive(Debug)]
struct City {
    pub neighbors: Vec<Flight>,
}

#[derive(Debug)]
struct Flight {
    pub from: usize,
    pub price: i32,
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p787_test() {
        check(
            4,
            vec![
                vec![0, 1, 100],
                vec![1, 2, 100],
                vec![2, 0, 100],
                vec![1, 3, 600],
                vec![2, 3, 200],
            ],
            0,
            3,
            1,
            700,
        );
        check(
            3,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            1,
            200,
        );
        check(
            3,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            0,
            500,
        );
    }

    fn check(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32, expect: i32) {
        assert_eq!(
            Solution::find_cheapest_price(n, flights, src, dst, k),
            expect
        );
    }
}
