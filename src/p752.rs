use crate::Solution;
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let deadends: HashSet<_> = deadends.into_iter().map(|s| text_to_code(&s)).collect();
        let target = text_to_code(&target);
        let init = [0u8; 4];

        let mut visited = vec![false; 10000];
        let mut queue = VecDeque::new();
        queue.push_back((init, 0));

        while let Some((cur_code, dist)) = queue.pop_front() {
            if cur_code == target {
                return dist;
            }

            if deadends.contains(&cur_code) {
                continue;
            }

            let cur_ix = code_to_index(cur_code);
            let visited_entry = &mut visited[cur_ix];
            if *visited_entry {
                continue;
            }
            *visited_entry = true;

            for neighbor_code in neighbors(cur_code) {
                queue.push_back((neighbor_code, dist + 1));
            }
        }

        -1
    }
}

fn text_to_code(text: &str) -> [u8; 4] {
    match text.as_bytes() {
        &[a, b, c, d] => [a - b'0', b - b'0', c - b'0', d - b'0'],
        _ => unreachable!(),
    }
}

fn code_to_index(code: [u8; 4]) -> usize {
    let [a, b, c, d] = code;
    (((a as usize * 10 + b as usize) * 10 + c as usize) * 10) + d as usize
}

fn neighbors(code: [u8; 4]) -> impl Iterator<Item = [u8; 4]> {
    (0..4).flat_map(move |ix| {
        let n1 = {
            let mut code = code;
            code[ix] = (code[ix] + 1) % 10;
            code
        };
        let n2 = {
            let mut code = code;
            code[ix] = (code[ix] + 9) % 10;
            code
        };
        [n1, n2]
    })
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p752_test() {
        check(&["8888"], "0009", 1);
        check(
            &[
                "8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888",
            ],
            "8888",
            -1,
        );
        check(&[], "0000", 0);
        check(&[], "0010", 1);
        check(&[], "0900", 1);
    }

    fn check(deadends: &[&str], target: &str, expect: i32) {
        let deadends: Vec<String> = deadends.iter().map(|s| s.to_string()).collect();
        let target = target.to_string();
        assert_eq!(Solution::open_lock(deadends, target), expect);
    }
}
