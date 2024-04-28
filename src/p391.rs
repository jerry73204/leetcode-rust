use crate::Solution;
use std::{
    collections::{BTreeMap, VecDeque},
    convert::TryInto,
    iter::FromIterator,
};

impl Solution {
    pub fn is_rectangle_cover(mut rectangles: Vec<Vec<i32>>) -> bool {
        let mut rectangles: Vec<_> = rectangles
            .iter()
            .map(|rect| -> &[i32; 4] {
                let array: &[i32; 4] = TryInto::try_into(rect.as_slice()).unwrap();
                array
            })
            .collect();

        rectangles.sort_by_cached_key(|[xl, yb, _, _]| (xl, yb));

        let [x_min, _, _, _] = **rectangles.first().unwrap();
        let [_, _, x_max, _] = **rectangles.last().unwrap();
        let y_min = rectangles.iter().map(|&&[_, yb, _, _]| yb).min().unwrap();
        let y_max = rectangles.iter().map(|&&[_, _, _, yt]| yt).max().unwrap();

        let mut rect_iter = rectangles.into_iter().peekable();

        let mut fronts = {
            let [_, yb, xr, yt] = *rect_iter.next_if(|&&[xl, _, _, _]| xl == x_min).unwrap();
            let mut fronts: VecDeque<_> =
                FromIterator::from_iter([(i32::MIN..yb, x_min), (yb..yt, xr)]);
            let mut next_yb = yt;

            while let Some(&[_, yb, xr, yt]) = rect_iter.next_if(|&&[xl, _, _, _]| xl == x_min) {
                if yb != next_yb {
                    return false;
                }

                fronts.push_back((yb..yt, xr));
                next_yb = yt;
            }

            if next_yb != y_max {
                return false;
            }

            fronts.push_back((y_max..i32::MAX, x_min));
            fronts
        };

        while let Some(&&[x_curr, _, _, _]) = rect_iter.peek() {
            let mut new_fronts = VecDeque::new();

            while let Some(&[_, yb, xr, yt]) = rect_iter.next_if(|&&[xl, _, _, _]| xl == x_curr) {
                while let Some((y_range, x_front)) = fronts.pop_front() {
                    if y_range.end <= yb {
                        new_fronts.push_back((y_range, x_front));
                    } else {
                        if x_front != x_curr {
                            return false;
                        }
                        if y_range.start < yb {
                            new_fronts.push_back((y_range.start..yb, x_curr));
                        }
                        break;
                    }
                }

                match new_fronts.back_mut() {
                    Some((y_range, x_prev)) => {
                        if *x_prev == xr {
                            y_range.end = yt;
                        } else {
                            new_fronts.push_back((yb..yt, xr));
                        }
                    }
                    None => {
                        new_fronts.push_back((yb..yt, xr));
                    }
                }

                while let Some((y_range, x_front)) = fronts.pop_front() {
                    if y_range.start < yt {
                        if x_front != x_curr {
                            return false;
                        }
                    } else {
                        fronts.push_front((y_range.clone(), x_front));
                        if y_range.start > yt {
                            fronts.push_front((yt..y_range.start, x_curr));
                        }
                        break;
                    }
                }
            }

            new_fronts.extend(fronts);
            fronts = new_fronts;
        }

        match fronts.pop_front() {
            Some((y_range, x_curr)) => {
                if !(y_range == (i32::MIN..y_min) && x_curr == x_min) {
                    return false;
                }
            }
            None => return false,
        }

        match fronts.pop_back() {
            Some((y_range, x_curr)) => {
                if !(y_range == (y_max..i32::MAX) && x_curr == x_min) {
                    return false;
                }
            }
            None => return false,
        }

        if !fronts.into_iter().all(|(_, x_curr)| x_curr == x_max) {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p391_test() {
        check(
            &[
                [1, 1, 3, 3],
                [3, 1, 4, 2],
                [3, 2, 4, 4],
                [1, 3, 2, 4],
                [2, 3, 3, 4],
            ],
            true,
        );
        check(
            &[[1, 1, 2, 3], [1, 3, 2, 4], [3, 1, 4, 2], [3, 2, 4, 4]],
            false,
        );
        check(
            &[[1, 1, 3, 3], [3, 1, 4, 2], [1, 3, 2, 4], [2, 2, 4, 4]],
            false,
        );
        check(&[[1, 1, 3, 3]], true);
        check(&[[1, 1, 3, 3], [3, 1, 4, 3]], true);
        check(&[[1, 1, 3, 3], [1, 3, 2, 4], [2, 3, 3, 4]], true);
        check(
            &[
                [0, 0, 4, 1],
                [7, 0, 8, 2],
                [6, 2, 8, 3],
                [5, 1, 6, 3],
                [4, 0, 5, 1],
                [6, 0, 7, 2],
                [4, 2, 5, 3],
                [2, 1, 4, 3],
                [0, 1, 2, 2],
                [0, 2, 2, 3],
                [4, 1, 5, 2],
                [5, 0, 6, 1],
            ],
            true,
        );
    }

    fn check(rectangles: &[[i32; 4]], expect: bool) {
        let rectangles: Vec<_> = rectangles.iter().map(|r| r.to_vec()).collect();
        assert_eq!(Solution::is_rectangle_cover(rectangles), expect);
    }
}
