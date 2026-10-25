pub struct Solution;

fn dfs(i: i32, j: i32, grid: &mut [Vec<i32>]) {
    let m = grid.len();
    let n = grid[0].len();
    grid[i as usize][j as usize] = 1;
    for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
        if i1 >= 0
            && j1 >= 0
            && i1 < m as i32
            && j1 < n as i32
            && grid[i1 as usize][j1 as usize] != 1
        {
            dfs(i1, j1, grid);
        }
    }
}

impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        for i in [0, m - 1] {
            for j in 0..n {
                if grid[i][j] == 0 {
                    dfs(i as i32, j as i32, &mut grid);
                }
            }
        }

        for j in [0, n - 1] {
            for i in 1..m - 1 {
                if grid[i][j] == 0 {
                    dfs(i as i32, j as i32, &mut grid);
                }
            }
        }

        let mut ans = 0;
        for i in 1..m - 1 {
            for j in 1..n - 1 {
                if grid[i][j] == 0 {
                    ans += 1;
                    dfs(i as i32, j as i32, &mut grid);
                }
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
        let grid = [
            [1, 1, 1, 1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0, 1, 1, 0],
            [1, 0, 1, 0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0, 1, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 0],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(2, Solution::closed_island(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 0, 1, 0, 0], [0, 1, 0, 1, 0], [0, 1, 1, 1, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(1, Solution::closed_island(grid));
    }

    #[test]
    fn case3() {
        let grid = [
            [1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 0, 1],
            [1, 0, 1, 1, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(2, Solution::closed_island(grid));
    }
}
