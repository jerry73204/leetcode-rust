use crate::Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut output: Vec<i32> = vec![];

        // Initialize left, right, top and bottom boundaries.
        let mut bt = 0;
        let mut bb = matrix.len();
        let mut bl = 0;
        let mut br = matrix[0].len();

        loop {
            match (bb - bt, br - bl) {
                (1, 1) => {
                    output.push(matrix[bt][bl]);
                    break;
                }
                (1, _) => {
                    output.extend(&matrix[bt][bl..br]);
                    break;
                }
                (_, 1) => {
                    let iter = (bt..bb).map(|row| matrix[row][bl]);
                    output.extend(iter);
                    break;
                }
                (_, _) => {
                    let iter1 = matrix[bt][bl..br].iter().cloned();
                    let iter2 = ((bt + 1)..bb).map(|row| matrix[row][br - 1]);
                    let iter3 = matrix[bb - 1][bl..(br - 1)].iter().rev().cloned();
                    let iter4 = ((bt + 1)..(bb - 1)).rev().map(|row| matrix[row][bl]);
                    let iter = iter1.chain(iter2).chain(iter3).chain(iter4);
                    output.extend(iter);

                    // Narrow down boundary
                    bt += 1;
                    bl += 1;
                    bb -= 1;
                    br -= 1;

                    if !(bt < bb && bl < br) {
                        break;
                    }
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p54_test() {
        check(
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
        );
        check(
            vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]],
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
        );
    }

    fn check(matrix: Vec<Vec<i32>>, expect: Vec<i32>) {
        assert_eq!(Solution::spiral_order(matrix), expect);
    }
}
