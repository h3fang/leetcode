pub struct Solution;

fn dfs(grid: &mut [Vec<i32>], i: i32, j: i32) -> i64 {
    let mut ans = grid[i as usize][j as usize] as i64;
    grid[i as usize][j as usize] = 0;

    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    for (i1, j1) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
        if i1 < 0 || j1 < 0 || i1 == m || j1 == n || grid[i1 as usize][j1 as usize] == 0 {
            continue;
        }
        ans += dfs(grid, i1, j1);
    }

    ans
}

impl Solution {
    pub fn count_islands(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as i64;
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != 0 {
                    let v = dfs(&mut grid, i as i32, j as i32);
                    if v % k == 0 {
                        ans += 1;
                    }
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
            [0, 2, 1, 0, 0],
            [0, 5, 0, 0, 5],
            [0, 0, 1, 0, 0],
            [0, 1, 4, 7, 0],
            [0, 2, 0, 0, 8],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(2, Solution::count_islands(grid, 5));
    }

    #[test]
    fn case2() {
        let grid = [[3, 0, 3, 0], [0, 3, 0, 3], [3, 0, 3, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(6, Solution::count_islands(grid, 3));
    }
}
