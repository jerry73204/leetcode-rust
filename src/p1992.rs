use crate::Solution;

impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let width = land[0].len();
        let cap = (width + 1) / 2;
        let mut answer = vec![];
        let mut lands: Vec<[i32; 4]> = Vec::with_capacity(cap);

        for (rix, row) in (0i32..).zip(land) {
            let bit_iter = (0i32..).zip(row);
            let mut stride_iter = std::iter::repeat(())
                .scan(bit_iter, |bit_iter, ()| -> Option<_> {
                    let (start, _) = bit_iter.find(|&(_, bit)| bit == 1)?;
                    let end = match bit_iter.find(|&(_, bit)| bit == 0) {
                        Some((end, _)) => end,
                        None => width as i32,
                    } - 1;
                    Some([start, end])
                })
                .peekable();

            let mut new_lands = Vec::with_capacity(cap);

            for [r1, c1, r2, c2] in lands {
                while let Some([start, end]) = stride_iter.next_if(|&[start, _]| start < c1) {
                    new_lands.push([rix, start, rix, end]);
                }

                if let Some([_start, end]) = stride_iter.next_if(|&[start, _]| start == c1) {
                    new_lands.push([r1, c1, rix, end]);
                } else {
                    answer.push(vec![r1, c1, r2, c2]);
                }
            }

            new_lands.extend(stride_iter.map(|[start, end]| [rix, start, rix, end]));
            lands = new_lands;
        }

        answer.extend(lands.into_iter().map(|range| range.to_vec()));
        answer
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1992_test() {
        check(
            vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 1, 1]],
            vec![vec![0, 0, 0, 0], vec![1, 1, 2, 2]],
        );
        check(vec![vec![1, 1], vec![1, 1]], vec![vec![0, 0, 1, 1]]);
        check(vec![vec![0]], vec![]);
        check(vec![vec![1]], vec![vec![0, 0, 0, 0]]);
    }

    fn check(land: Vec<Vec<i32>>, expect: Vec<Vec<i32>>) {
        assert_eq!(Solution::find_farmland(land), expect);
    }
}
