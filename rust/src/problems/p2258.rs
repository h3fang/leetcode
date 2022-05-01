use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        fn spread_fire(grid: &[Vec<i32>]) -> Vec<i32> {
            let m = grid.len();
            let n = grid[0].len();

            let mut fire = vec![i32::MAX; m * n];
            let mut q = VecDeque::new();
            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 1 {
                        fire[i * n + j] = 0;
                        q.push_back((i, j));
                    }
                }
            }
            let mut k = 1;
            while !q.is_empty() {
                let len = q.len();
                for _ in 0..len {
                    let (i, j) = q.pop_front().unwrap();
                    let i = i as i32;
                    let j = j as i32;
                    for (i1, j1) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                        if i1 < 0
                            || j1 < 0
                            || i1 == m as i32
                            || j1 == n as i32
                            || grid[i1 as usize][j1 as usize] == 2
                        {
                            continue;
                        }
                        let i1 = i1 as usize;
                        let j1 = j1 as usize;
                        let f = fire[i1 * n + j1];
                        if f > k {
                            fire[i1 * n + j1] = k;
                            q.push_back((i1, j1));
                        }
                    }
                }
                k += 1;
            }
            fire
        }

        fn can_escape(grid: &[Vec<i32>], fire: &[i32], mut k: i32) -> bool {
            let m = grid.len();
            let n = grid[0].len();
            if fire[0] <= k {
                return false;
            }
            let mut vis = vec![false; m * n];
            let mut q = VecDeque::new();
            q.push_back((0, 0));
            vis[0] = true;
            while !q.is_empty() {
                let len = q.len();
                for _ in 0..len {
                    let (i, j) = q.pop_front().unwrap();
                    if fire[i * n + j] <= k {
                        continue;
                    }
                    let i = i as i32;
                    let j = j as i32;
                    for (i1, j1) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                        if i1 < 0 || j1 < 0 || i1 == m as i32 || j1 == n as i32 {
                            continue;
                        }
                        let i1 = i1 as usize;
                        let j1 = j1 as usize;
                        if grid[i1][j1] == 2 || vis[i1 * n + j1] || fire[i1 * n + j1] <= k {
                            continue;
                        }
                        if i1 == m - 1 && j1 == n - 1 {
                            return true;
                        }
                        vis[i1 * n + j1] = true;
                        q.push_back((i1, j1));
                    }
                }
                k += 1;
            }
            false
        }

        let m = grid.len();
        let n = grid[0].len();

        let fire = spread_fire(&grid);

        let mut left = 0;
        let mut right = (m * n) as i32;
        let mut result = -1;
        while left <= right {
            let mid = (left + right) / 2;
            if can_escape(&grid, &fire, mid) {
                left = mid + 1;
                result = mid;
            } else {
                right = mid - 1;
            }
        }
        if result == (m * n) as i32 {
            10_0000_0000
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [
            [0, 2, 0, 0, 0, 0, 0],
            [0, 0, 0, 2, 2, 1, 0],
            [0, 2, 0, 0, 1, 2, 0],
            [0, 0, 2, 2, 2, 0, 2],
            [0, 0, 0, 0, 0, 0, 0],
        ];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(3, Solution::maximum_minutes(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 0, 0, 0], [0, 1, 2, 0], [0, 2, 0, 0]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(-1, Solution::maximum_minutes(grid));
    }

    #[test]
    fn case3() {
        let grid = [[0, 0, 0], [2, 2, 0], [1, 2, 0]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(1000000000, Solution::maximum_minutes(grid));
    }

    #[test]
    fn case4() {
        let grid = [
            [0, 2, 0, 0, 1],
            [0, 2, 0, 2, 2],
            [0, 2, 0, 0, 0],
            [0, 0, 2, 2, 0],
            [0, 0, 0, 0, 0],
        ];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(0, Solution::maximum_minutes(grid));
    }
}
