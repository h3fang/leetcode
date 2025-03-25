pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut q = Vec::with_capacity(m + n);
        for j in 0..n {
            q.push((0, j));
        }
        for i in 0..m {
            q.push((i, 0));
        }
        let mut ans = vec![vec![0; n]; m];
        let mut s = HashSet::with_capacity(m + n);
        for (mut i, mut j) in q {
            let mut top_left = Vec::with_capacity(m + n);
            s.clear();
            while i < m && j < n {
                top_left.push(s.len() as i32);
                s.insert(grid[i][j]);
                i += 1;
                j += 1;
            }
            s.clear();
            i -= 1;
            j -= 1;
            while let Some(v) = top_left.pop() {
                ans[i][j] = v.abs_diff(s.len() as i32) as i32;
                s.insert(grid[i][j]);
                i = i.saturating_sub(1);
                j = j.saturating_sub(1);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 2, 3], [3, 1, 5], [3, 2, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let ans = Solution::difference_of_distinct_values(grid);
        let expected = [[1, 1, 0], [1, 0, 1], [0, 1, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, ans);
    }

    #[test]
    fn case2() {
        let grid = [[1]].iter().map(|r| r.to_vec()).collect();
        let ans = Solution::difference_of_distinct_values(grid);
        let expected = [[0]].iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, ans);
    }
}
