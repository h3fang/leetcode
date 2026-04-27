pub struct Solution;

fn next(dir: i32, cell: i32) -> i32 {
    match (dir, cell) {
        (0, 1) => 0,
        (2, 1) => 2,
        (1, 2) => 1,
        (3, 2) => 3,
        (0, 3) => 1,
        (3, 3) => 2,
        (2, 4) => 1,
        (3, 4) => 0,
        (0, 5) => 3,
        (1, 5) => 2,
        (2, 6) => 3,
        (1, 6) => 0,
        _ => -1,
    }
}

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        let mut vis = vec![false; (m * n) as usize];
        for mut d in 0..4 {
            if d > 0 {
                vis.fill(false);
            }

            let (mut i, mut j) = (0, 0);

            loop {
                if vis[(i * n + j) as usize] {
                    break;
                }
                vis[(i * n + j) as usize] = true;
                let cell = grid[i as usize][j as usize];
                d = next(d, cell);
                if d == -1 {
                    break;
                }

                if (i, j) == (m - 1, n - 1) {
                    return true;
                }

                let (di, dj) = DIRS[d as usize];
                i += di;
                j += dj;
                if i < 0 || j < 0 || i == m || j == n {
                    break;
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
        let grid = [[2, 4, 3], [6, 5, 2]].iter().map(|r| r.to_vec()).collect();
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 2, 1], [1, 2, 1]].iter().map(|r| r.to_vec()).collect();
        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn case3() {
        let grid = [[1, 1, 2]].iter().map(|r| r.to_vec()).collect();
        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn case4() {
        let grid = [[1, 1, 1, 1, 1, 1, 3]].iter().map(|r| r.to_vec()).collect();
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn case5() {
        let grid = [[2], [2], [2], [2], [2], [2], [6]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert!(Solution::has_valid_path(grid));
    }
}
