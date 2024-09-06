use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        if grid[0][0] != '(' || grid[m - 1][n - 1] != ')' {
            return false;
        }
        let mut q = VecDeque::new();
        q.push_back((0, 0, 1));
        let mut vis = vec![vec![vec![false; 200]; 100]; 100];
        vis[0][0][1] = true;
        while let Some((i, j, k)) = q.pop_front() {
            if i == m - 1 && j == n - 1 && k == 0 {
                return true;
            }
            let dist = (m - 1 + n - 1 - i - j) as i32;
            if dist < k {
                continue;
            }
            for (i1, j1) in [(i + 1, j), (i, j + 1)] {
                if i1 == m || j1 == n {
                    continue;
                }
                let k1 = match grid[i1][j1] {
                    '(' => k + 1,
                    _ => k - 1,
                };
                if k1 < 0 {
                    continue;
                }
                let s = (i1, j1, k1);
                if !vis[i1][j1][k1 as usize] {
                    q.push_back(s);
                    vis[i1][j1][k1 as usize] = true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [
            ["(", "(", "("],
            [")", "(", ")"],
            ["(", "(", ")"],
            ["(", "(", ")"],
        ];
        let grid = grid
            .iter()
            .map(|row| row.iter().map(|c| c.chars().next().unwrap()).collect())
            .collect();
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn case2() {
        let grid = [[")", ")"], ["(", "("]];
        let grid = grid
            .iter()
            .map(|row| row.iter().map(|c| c.chars().next().unwrap()).collect())
            .collect();
        assert!(!Solution::has_valid_path(grid));
    }
}
