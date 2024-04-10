use crate::Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut lslice = nums1.as_slice();
        let mut rslice = nums2.as_slice();

        let (mut nth, nth_rem) = divrem2(lslice.len() + rslice.len() - 1);
        let mut loffset = 0;
        let mut roffset = 0;

        loop {
            // println!(
            //     "l={:?} r={:?} nth={nth}.{}",
            //     loffset..(loffset + lslice.len()),
            //     roffset..(roffset + rslice.len()),
            //     if nth_rem { "5" } else { "0" }
            // );

            match (lslice.len(), rslice.len()) {
                (0, 0) => unreachable!(),
                (0, _) => {
                    if nth_rem {
                        if rslice.len() == nth + 1 {
                            let med2 = match (nums1.get(loffset), nums2.get(roffset + rslice.len()))
                            {
                                (None, None) => unreachable!(),
                                (Some(val), None) => val,
                                (None, Some(val)) => val,
                                (Some(lval), Some(rval)) => lval.min(rval),
                            };

                            break (rslice[nth] + med2) as f64 / 2.0;
                        } else {
                            break (rslice[nth] + rslice[nth + 1]) as f64 / 2.0;
                        }
                    } else {
                        break rslice[nth] as f64;
                    }
                }
                (_, 0) => {
                    if nth_rem {
                        if lslice.len() == nth + 1 {
                            let med2 = match (nums1.get(loffset + lslice.len()), nums2.get(roffset))
                            {
                                (None, None) => unreachable!(),
                                (Some(val), None) => val,
                                (None, Some(val)) => val,
                                (Some(lval), Some(rval)) => lval.min(rval),
                            };

                            break (lslice[nth] + med2) as f64 / 2.0;
                        } else {
                            break (lslice[nth] + lslice[nth + 1]) as f64 / 2.0;
                        }
                    } else {
                        break lslice[nth] as f64;
                    }
                }
                (1, 1) => {
                    let lval = lslice[0];
                    let rval = rslice[0];

                    let (lval, rval) = if lval <= rval {
                        (lval, rval)
                    } else {
                        (rval, lval)
                    };

                    match (nth, nth_rem) {
                        (0, false) => break lval as f64,
                        (0, true) => break (lval + rval) as f64 / 2.0,
                        (1, false) => break rval as f64,
                        (1, true) => {
                            let med2 = match (nums1.get(loffset + 1), nums2.get(roffset + 1)) {
                                (None, None) => unreachable!(),
                                (Some(val), None) => val,
                                (None, Some(val)) => val,
                                (Some(lval), Some(rval)) => lval.min(rval),
                            };
                            break (rval + med2) as f64 / 2.0;
                        }
                        _ => unreachable!(),
                    }
                }
                (_, _) => {
                    let (lix1, rix1) = find_median_pair(lslice, rslice);
                    let (rix2, lix2) = find_median_pair(rslice, lslice);

                    let (lix1, lix2) = sort_pair(lix1, lix2);
                    let (rix1, rix2) = sort_pair(rix1, rix2);

                    // println!("=> lmed={:?} rmed={:?}", lix1..lix2, rix1..rix2);

                    let low_count = lix1 + rix1;
                    let med_count = lix2 + rix2;

                    if nth < low_count {
                        lslice = &lslice[0..lix1];
                        rslice = &rslice[0..rix1];
                    } else if nth < med_count {
                        lslice = &lslice[lix1..lix2];
                        rslice = &rslice[rix1..rix2];
                        nth -= low_count;
                        loffset += lix1;
                        roffset += rix1;
                    } else {
                        lslice = &lslice[lix2..];
                        rslice = &rslice[rix2..];
                        nth -= med_count;
                        loffset += lix2;
                        roffset += rix2;
                    }
                }
            }
        }
    }
}

fn find_median_pair(lslice: &[i32], rslice: &[i32]) -> (usize, usize) {
    let lix = lslice.len() / 2;
    let lval = lslice[lix];

    let rix = match rslice.binary_search(&lval) {
        Ok(ix) => ix,
        Err(ix) => ix,
    };

    (lix, rix)
}

fn sort_pair(lix: usize, rix: usize) -> (usize, usize) {
    if lix <= rix {
        (lix, rix)
    } else {
        (rix, lix)
    }
}

fn divrem2(val: usize) -> (usize, bool) {
    let quot = val / 2;
    let rem = val % 2 == 1;
    (quot, rem)
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p4_test() {
        check(vec![1, 3], vec![2], 2.0);
        check(vec![1, 2], vec![3, 4], 2.5);
        check(vec![1, 1], vec![1, 1], 1.0);
        check(vec![1], vec![1], 1.0);
        check(vec![], vec![1], 1.0);
        check(vec![1], vec![], 1.0);
        check(vec![2], vec![1], 1.5);
        check(vec![1, 1, 1], vec![3, 3, 3], 2.0);
        check(vec![1, 3, 5], vec![2, 4], 3.0);
        check(vec![1, 3, 5], vec![2, 4, 6], 3.5);
    }

    fn check(nums1: Vec<i32>, nums2: Vec<i32>, expect: f64) {
        assert_eq!(
            Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone()),
            expect
        );
        assert_eq!(Solution::find_median_sorted_arrays(nums2, nums1), expect);
    }
}
