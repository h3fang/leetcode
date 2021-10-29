pub struct Solution;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        fn step(grid: &mut Vec<Vec<i32>>) -> bool {
            let old = grid.clone();
            let m = old.len();
            let n = old[0].len();
            let mut changed = false;
            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 1 {
                        if i > 0 && old[i - 1][j] == 2 {
                            grid[i][j] = 2;
                            changed = true;
                            continue;
                        }
                        if j > 0 && old[i][j - 1] == 2 {
                            grid[i][j] = 2;
                            changed = true;
                            continue;
                        }
                        if i < m - 1 && old[i + 1][j] == 2 {
                            grid[i][j] = 2;
                            changed = true;
                            continue;
                        }
                        if j < n - 1 && old[i][j + 1] == 2 {
                            grid[i][j] = 2;
                            changed = true;
                        }
                    }
                }
            }

            changed
        }

        let mut i = 0;
        while step(&mut grid) {
            i += 1;
        }
        if grid.iter().flatten().all(|c| *c != 1) {
            i
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[2, 1, 1], [1, 1, 0], [0, 1, 1]];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(4, Solution::oranges_rotting(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 2]];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(0, Solution::oranges_rotting(grid));
    }
}
