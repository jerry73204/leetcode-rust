use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut in_degrees = vec![0; n];
        let mut out_edges = vec![vec![]; n];
        let mut ancestors = vec![vec![]; n];

        for edge in edges {
            let [src, dst] = match edge.as_slice() {
                &[src, dst] => [src, dst],
                _ => unreachable!(),
            };

            in_degrees[dst as usize] += 1;
            out_edges[src as usize].push(dst);
        }

        let mut frontiers: Vec<_> = in_degrees
            .iter()
            .enumerate()
            .filter(|(_, deg)| **deg == 0)
            .map(|(idx, _)| idx as i32)
            .collect();

        while let Some(curr) = frontiers.pop() {
            for &next in &out_edges[curr as usize] {
                let deg = &mut in_degrees[next as usize];
                *deg -= 1;
                let new_ancestors =
                    merge(&ancestors[curr as usize], &ancestors[next as usize], curr);
                ancestors[next as usize] = new_ancestors;

                if *deg == 0 {
                    frontiers.push(next);
                }
            }
        }

        ancestors
    }
}

fn merge(lslice: &[i32], rslice: &[i32], val: i32) -> Vec<i32> {
    let mut output = Vec::with_capacity(lslice.len() + rslice.len());

    let (lhead, ltail) = split(lslice, val);
    let (rhead, rtail) = split(rslice, val);

    merge_sorted_slices(lhead, rhead, &mut output);
    output.push(val);
    merge_sorted_slices(ltail, rtail, &mut output);

    output
}

fn merge_sorted_slices(mut lslice: &[i32], mut rslice: &[i32], output: &mut Vec<i32>) {
    loop {
        let lsplit = lslice.split_first();
        let rsplit = rslice.split_first();

        match (lsplit, rsplit) {
            (Some((&lhead, ltail)), Some((&rhead, rtail))) => match lhead.cmp(&rhead) {
                Ordering::Less => {
                    output.push(lhead);
                    lslice = ltail;
                }
                Ordering::Equal => {
                    output.push(lhead);
                    lslice = ltail;
                    rslice = rtail;
                }
                Ordering::Greater => {
                    output.push(rhead);
                    rslice = rtail;
                }
            },
            (Some(_), None) => {
                output.extend(lslice);
                break;
            }
            (None, Some(_)) => {
                output.extend(rslice);
                break;
            }
            (None, None) => break,
        }
    }
}

fn split(slice: &[i32], pivot: i32) -> (&[i32], &[i32]) {
    let ix = match slice.binary_search(&pivot) {
        Ok(ix) => ix,
        Err(ix) => ix,
    };
    let (head, tail) = slice.split_at(ix);

    let tail = match tail.iter().enumerate().find(|(_, val)| **val != pivot) {
        Some((ix, _)) => &tail[ix..],
        None => &[],
    };

    (head, tail)
}
