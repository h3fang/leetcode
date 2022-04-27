use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();
        let mut f = vec![vec![false; n]; m];

        let mut q = VecDeque::new();
        let mut vis = vec![vec![false; n]; m];
        for i in 0..n {
            q.push_back((0, i));
        }
        for j in 1..m {
            q.push_back((j, 0));
        }
        while !q.is_empty() {
            let (i, j) = q.pop_front().unwrap();
            vis[i][j] = true;
            f[i][j] = true;
            for (i1, j1) in [
                ((i + 1).min(m - 1), j),
                (i.saturating_sub(1), j),
                (i, (j + 1).min(n - 1)),
                (i, j.saturating_sub(1)),
            ] {
                if !vis[i1][j1] && heights[i][j] <= heights[i1][j1] {
                    q.push_back((i1, j1));
                }
            }
        }

        let mut result = vec![];

        let mut q = VecDeque::new();
        let mut vis = vec![vec![false; n]; m];
        for i in 0..n {
            q.push_back((m - 1, i));
        }
        for j in 0..m - 1 {
            q.push_back((j, n - 1));
        }
        while !q.is_empty() {
            let (i, j) = q.pop_front().unwrap();
            if vis[i][j] {
                continue;
            }
            vis[i][j] = true;
            if f[i][j] {
                result.push(vec![i as i32, j as i32]);
            }
            for (i1, j1) in [
                ((i + 1).min(m - 1), j),
                (i.saturating_sub(1), j),
                (i, (j + 1).min(n - 1)),
                (i, j.saturating_sub(1)),
            ] {
                if !vis[i1][j1] && heights[i][j] <= heights[i1][j1] {
                    q.push_back((i1, j1));
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
        let heights = [
            [1, 2, 2, 3, 5],
            [3, 2, 3, 4, 4],
            [2, 4, 5, 3, 1],
            [6, 7, 1, 4, 5],
            [5, 1, 1, 2, 4],
        ];
        let heights = heights.iter().map(|r| r.to_vec()).collect();
        let expected = [[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]];
        let mut expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::pacific_atlantic(heights);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
