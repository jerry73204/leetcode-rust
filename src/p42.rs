use crate::Solution;
use std::cmp::Reverse;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut iter = height.iter().enumerate().map(|(pos, &value)| Wall {
            height: value,
            position: pos,
        });
        let mut wall = vec![iter.next().unwrap()];
        let mut start = 0;
        let mut end = 1;

        for item in iter {
            let offset = binary_search(&wall[start..end], &Reverse(item.height), |wall| {
                Reverse(wall.height)
            });
            let offset = match offset {
                Ok(offset) => offset,
                Err(offset) => offset,
            };
            let insert_idx = start + offset;

            if insert_idx == wall.len() {
                wall.push(item);
                end = wall.len();
            } else if insert_idx == start {
                start += 1;

                if start == wall.len() {
                    wall.push(item);
                    end = wall.len();
                } else {
                    wall[start] = item;
                    end = start + 1;
                }
            } else {
                wall[insert_idx] = item;
                end = insert_idx + 1;
            }
        }

        let wall = &wall[0..end];
        // dbg!(wall);

        let ascend_volume: i32 = wall
            .iter()
            .zip(&wall[1..])
            .take(start)
            .map(|(lhs, rhs)| {
                // dbg!(lhs, rhs);
                let water_volume = lhs.height * (rhs.position - lhs.position - 1) as i32;
                let obstacle_volume: i32 = height[(lhs.position + 1)..rhs.position]
                    .iter()
                    .copied()
                    .sum();
                water_volume - obstacle_volume
            })
            .sum();

        // dbg!(start);
        let descend_volume: i32 = wall[start..]
            .iter()
            .zip(&wall[(start + 1)..])
            .map(|(lhs, rhs)| {
                // dbg!(lhs, rhs);
                let water_volume = rhs.height * (rhs.position - lhs.position - 1) as i32;
                let obstacle_volume: i32 = height[(lhs.position + 1)..rhs.position]
                    .iter()
                    .copied()
                    .sum();
                water_volume - obstacle_volume
            })
            .sum();

        ascend_volume + descend_volume
    }
}

#[derive(Debug)]
struct Wall {
    pub height: i32,
    pub position: usize,
}

fn binary_search<'a, T, B, F>(mut slice: &'a [T], target: &B, mut key_fn: F) -> Result<usize, usize>
where
    B: Ord,
    F: FnMut(&'a T) -> B + Clone,
{
    match slice.binary_search_by_key(target, key_fn.clone()) {
        Ok(idx) => {
            if idx == 0 || (key_fn)(&slice[idx - 1]) != *target {
                return Ok(idx);
            }

            slice = &slice[0..idx];

            loop {
                match binary_search(slice, target, key_fn.clone()) {
                    Ok(idx) => {
                        if idx == 0 || (key_fn)(&slice[idx - 1]) != *target {
                            return Ok(idx);
                        }

                        slice = &slice[0..idx];
                    }
                    Err(idx) => return Ok(idx + 1),
                }
            }
        }
        Err(idx) => Err(idx),
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p42_test() {
        check(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6);
        check(vec![4, 2, 0, 3, 2, 5], 9);
        check(vec![2, 0, 2], 2);
        check(vec![4, 2, 0, 3, 2, 4, 3, 4], 10);
        check(
            vec![
                1000, 999, 998, 997, 996, 995, 994, 993, 992, 991, 990, 989, 988, 987, 986, 985,
                984, 983, 982, 981, 980, 979, 978, 977, 966, 966, 966, 966, 966, 966, 966, 966,
                966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
                966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
                966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
                966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
                966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
                966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
                966, 966, 966, 966,
            ],
            0,
        );
    }

    fn check(height: Vec<i32>, expect: i32) {
        assert_eq!(Solution::trap(height), expect);
    }
}
