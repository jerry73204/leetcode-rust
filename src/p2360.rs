use crate::Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let mut marks: Vec<Option<Mark>> = vec![None; edges.len()];

        (0..edges.len())
            .filter_map(|start| {
                let mut curr = start;
                let mut path_len = 0;

                loop {
                    let mark = &mut marks[curr];

                    if let Some(mark) = mark {
                        if mark.start == start {
                            let cycle_len = path_len - mark.path_len;
                            return Some(cycle_len);
                        } else {
                            break;
                        }
                    }

                    *mark = Some(Mark { start, path_len });

                    let next = edges[curr];
                    if next == -1 {
                        break;
                    }

                    curr = next as usize;
                    path_len += 1;
                }

                None
            })
            .max()
            .map(|max_len| max_len as i32)
            .unwrap_or(-1)
    }
}

#[derive(Debug, Clone)]
struct Mark {
    pub start: usize,
    pub path_len: usize,
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p2360_test() {
        check(vec![3, 3, 4, 2, 3], 3);
        check(vec![2, -1, 3, 1], -1);
        check(vec![1, 2, 3, 1, 5, 6, 7, 8, 5], 4);
    }

    fn check(edges: Vec<i32>, expect: i32) {
        assert_eq!(Solution::longest_cycle(edges), expect);
    }
}
