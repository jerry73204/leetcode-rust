use crate::Solution;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let n = n as usize;

        let mut uf = UnionFind::new(n);

        for edge in edges {
            let [n1, n2] = match edge.as_slice() {
                &[n1, n2] => [n1 as usize, n2 as usize],
                _ => unreachable!(),
            };

            uf.union(n1, n2);
        }

        let (g_src, _) = uf.find(source as usize);
        let (g_dst, _) = uf.find(destination as usize);
        g_src == g_dst
    }
}

#[derive(Debug)]
struct UnionFind {
    groups: Vec<(usize, usize)>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            groups: (0..size).map(|ix| (ix, 0)).collect(),
        }
    }

    pub fn find(&self, ix: usize) -> (usize, usize) {
        let mut curr = ix;

        loop {
            let (next, height) = self.groups[curr];

            if curr == next {
                break (curr, height);
            } else {
                curr = next;
            }
        }
    }

    pub fn union(&mut self, n1: usize, n2: usize) {
        let (g1, h1) = self.find(n1);
        let (g2, h2) = self.find(n2);

        if g1 == g2 {
            return;
        }

        let (g_min, g_max, h_min, h_max) = if h1 <= h2 {
            (g1, g2, h1, h2)
        } else {
            (g2, g1, h2, h1)
        };

        let h_new = if h_min == h_max { h_max + 1 } else { h_max };

        self.groups[g_min] = (g_max, h_new);
        self.groups[g_max] = (g_max, h_new);
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn p1971_test() {
        check(3, &[[0, 1], [1, 2], [2, 0]], 0, 2, true);
        check(6, &[[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]], 0, 5, false);
        check(1, &[], 0, 0, true);
        check(3, &[], 0, 2, false);
        check(6, &[[0, 1], [0, 2], [3, 4], [3, 5], [0, 5]], 0, 5, true);
        check(
            10,
            &[
                [2, 6],
                [4, 7],
                [1, 2],
                [3, 5],
                [7, 9],
                [6, 4],
                [9, 8],
                [0, 1],
                [3, 0],
            ],
            3,
            5,
            true,
        );
    }

    fn check(n: i32, edges: &[[i32; 2]], source: i32, destination: i32, expect: bool) {
        let edges: Vec<_> = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(Solution::valid_path(n, edges, source, destination), expect);
    }
}
