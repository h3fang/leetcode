pub struct Solution;

use std::collections::BTreeMap;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<u32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return;
        }
        match self.size[px].cmp(&self.size[py]) {
            std::cmp::Ordering::Less => {
                self.parent[px] = py;
                self.size[py] += self.size[px];
            }
            _ => {
                self.parent[py] = px;
                self.size[px] += self.size[py];
            }
        }
    }
}

impl Solution {
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut group: BTreeMap<i32, Vec<_>> = BTreeMap::new();
        for (i, row) in matrix.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                group.entry(c).or_default().push((i, j));
            }
        }

        let mut rank: BTreeMap<usize, i32> = BTreeMap::new();
        let mut row_max = vec![0; m];
        let mut col_max = vec![0; n];
        let mut result = vec![vec![0; n]; m];
        for g in group.values() {
            let mut uf = UnionFind::new(m + n);
            for &(i, j) in g {
                uf.union(i, j + m);
            }
            for &(i, j) in g {
                let e = rank.entry(uf.find(i)).or_default();
                *e = (*e).max(row_max[i].max(col_max[j]));
            }
            for &(i, j) in g {
                let r = 1 + rank.get(&uf.find(i)).unwrap();
                result[i][j] = r;
                row_max[i] = r;
                col_max[j] = r;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = [[1, 2], [3, 4]].iter().map(|r| r.to_vec()).collect();
        let expected = [[1, 2], [2, 3]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::matrix_rank_transform(matrix));
    }

    #[test]
    fn case2() {
        let matrix = [[7, 7], [7, 7]].iter().map(|r| r.to_vec()).collect();
        let expected = [[1, 1], [1, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::matrix_rank_transform(matrix));
    }

    #[test]
    fn case3() {
        let matrix = [[20, -21, 14], [-19, 4, 19], [22, -47, 24], [-19, 4, 19]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let expected = [[4, 2, 3], [1, 3, 4], [5, 1, 6], [1, 3, 4]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::matrix_rank_transform(matrix));
    }

    #[test]
    fn case4() {
        let matrix = [[7, 3, 6], [1, 4, 5], [9, 8, 2]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let expected = [[5, 1, 4], [1, 2, 3], [6, 3, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::matrix_rank_transform(matrix));
    }
}
