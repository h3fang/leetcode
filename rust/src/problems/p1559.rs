pub struct Solution;

fn dfs(grid: &[Vec<char>], vis: &mut [bool], x: i32, y: i32, px: i32, py: i32) -> bool {
    let (m, n) = (grid.len(), grid[0].len());
    vis[x as usize * n + y as usize] = true;

    for (x1, y1) in [(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)] {
        if x1 < 0
            || y1 < 0
            || x1 == m as i32
            || y1 == n as i32
            || (x1, y1) == (px, py)
            || grid[x1 as usize][y1 as usize] != grid[x as usize][y as usize]
        {
            continue;
        }

        if vis[x1 as usize * n + y1 as usize] || dfs(grid, vis, x1, y1, x, y) {
            return true;
        }
    }

    false
}

impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut vis = vec![false; m * n];

        for i in 0..m {
            for j in 0..n {
                if !vis[i * n + j] && dfs(&grid, &mut vis, i as i32, j as i32, -1, -1) {
                    return true;
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
            ["a", "a", "a", "a"],
            ["a", "b", "b", "a"],
            ["a", "b", "b", "a"],
            ["a", "a", "a", "a"],
        ]
        .iter()
        .map(|r| r.iter().map(|s| s.as_bytes()[0] as char).collect())
        .collect();
        assert!(Solution::contains_cycle(grid));
    }

    #[test]
    fn case2() {
        let grid = [
            ["c", "c", "c", "a"],
            ["c", "d", "c", "c"],
            ["c", "c", "e", "c"],
            ["f", "c", "c", "c"],
        ]
        .iter()
        .map(|r| r.iter().map(|s| s.as_bytes()[0] as char).collect())
        .collect();
        assert!(Solution::contains_cycle(grid));
    }

    #[test]
    fn case3() {
        let grid = [["a", "b", "b"], ["b", "z", "b"], ["b", "b", "a"]]
            .iter()
            .map(|r| r.iter().map(|s| s.as_bytes()[0] as char).collect())
            .collect();
        assert!(!Solution::contains_cycle(grid));
    }
}
