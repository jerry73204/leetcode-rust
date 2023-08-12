use crate::Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();

        let nrow = board.len();
        let ncol = board[0].len();
        let mut visited = vec![vec![false; ncol]; nrow];

        for row in 0..nrow {
            for col in 0..ncol {
                let found = dfs(&board, &mut visited, (row, col), 0, &chars, nrow, ncol);
                if found {
                    return true;
                }
            }
        }

        false
    }
}

fn dfs(
    board: &[Vec<char>],
    visited: &mut [Vec<bool>],
    (row, col): (usize, usize),
    char_idx: usize,
    chars: &[char],
    nrow: usize,
    ncol: usize,
) -> bool {
    if char_idx == chars.len() {
        return true;
    }

    let vis = &mut visited[row][col];

    if *vis || board[row][col] != chars[char_idx] {
        return false;
    }

    if char_idx + 1 == chars.len() {
        return true;
    }

    *vis = true;

    for (nb_row, nb_col) in get_neighbors((row, col), nrow, ncol) {
        let found = dfs(
            board,
            visited,
            (nb_row, nb_col),
            char_idx + 1,
            chars,
            nrow,
            ncol,
        );

        if found {
            return true;
        }
    }

    visited[row][col] = false;
    false
}

fn get_neighbors(
    (row, col): (usize, usize),
    nrow: usize,
    ncol: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let p1 = row.checked_sub(1).map(|r| (r, col));
    let p2 = col.checked_sub(1).map(|c| (row, c));
    let p3 = (row + 1 < nrow).then(|| (row + 1, col));
    let p4 = (col + 1 < ncol).then(|| (row, col + 1));
    p1.into_iter().chain(p2).chain(p3).chain(p4)
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p79_test() {
        // check(
        //     vec![
        //         vec!['A', 'B', 'C', 'E'],
        //         vec!['S', 'F', 'C', 'S'],
        //         vec!['A', 'D', 'E', 'E'],
        //     ],
        //     "ABCCED",
        //     true,
        // );

        // check(
        //     vec![
        //         vec!['A', 'B', 'C', 'E'],
        //         vec!['S', 'F', 'C', 'S'],
        //         vec!['A', 'D', 'E', 'E'],
        //     ],
        //     "SEE",
        //     true,
        // );

        // check(
        //     vec![
        //         vec!['A', 'B', 'C', 'E'],
        //         vec!['S', 'F', 'C', 'S'],
        //         vec!['A', 'D', 'E', 'E'],
        //     ],
        //     "ABCB",
        //     false,
        // );

        check(vec![vec!['a']], "a", true);
    }

    fn check(board: Vec<Vec<char>>, word: &str, expect: bool) {
        assert_eq!(Solution::exist(board, word.to_string()), expect);
    }
}
