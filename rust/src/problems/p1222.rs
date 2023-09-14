pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let queens = queens
            .into_iter()
            .map(|q| (q[0], q[1]))
            .collect::<HashSet<_>>();
        let mut result = Vec::with_capacity(queens.len());
        for (di, dj) in [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ] {
            let (mut i, mut j) = (king[0], king[1]);
            loop {
                i += di;
                j += dj;
                if i < 0 || j < 0 || i >= 8 || j >= 8 {
                    break;
                }
                if queens.contains(&(i, j)) {
                    result.push(vec![i, j]);
                    break;
                }
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
        let queens = [[0, 1], [1, 0], [4, 0], [0, 4], [3, 3], [2, 4]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        let king = vec![0, 0];
        let mut result = Solution::queens_attackthe_king(queens, king);
        result.sort_unstable();
        let mut expected = [[0, 1], [1, 0], [3, 3]]
            .iter()
            .map(|q| q.to_vec())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let queens = [[0, 0], [1, 1], [2, 2], [3, 4], [3, 5], [4, 4], [4, 5]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        let king = vec![3, 3];
        let mut result = Solution::queens_attackthe_king(queens, king);
        result.sort_unstable();
        let mut expected = [[2, 2], [3, 4], [4, 4]]
            .iter()
            .map(|q| q.to_vec())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case3() {
        let queens = [
            [5, 6],
            [7, 7],
            [2, 1],
            [0, 7],
            [1, 6],
            [5, 1],
            [3, 7],
            [0, 3],
            [4, 0],
            [1, 2],
            [6, 3],
            [5, 0],
            [0, 4],
            [2, 2],
            [1, 1],
            [6, 4],
            [5, 4],
            [0, 0],
            [2, 6],
            [4, 5],
            [5, 2],
            [1, 4],
            [7, 5],
            [2, 3],
            [0, 5],
            [4, 2],
            [1, 0],
            [2, 7],
            [0, 1],
            [4, 6],
            [6, 1],
            [0, 6],
            [4, 3],
            [1, 7],
        ]
        .iter()
        .map(|q| q.to_vec())
        .collect();
        let king = vec![3, 4];
        let mut result = Solution::queens_attackthe_king(queens, king);
        result.sort_unstable();
        let mut expected = [[2, 3], [1, 4], [1, 6], [3, 7], [4, 3], [5, 4], [4, 5]]
            .iter()
            .map(|q| q.to_vec())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
