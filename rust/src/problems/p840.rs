pub struct Solution;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        if m < 3 || n < 3 {
            return 0;
        }
        let mut result = 0;
        for (i, r) in grid.iter().enumerate().skip(1).take(m - 2) {
            for (j, &c) in r.iter().enumerate().skip(1).take(n - 2) {
                if c != 5 {
                    continue;
                }
                let mut f = [0; 16];
                for i in i - 1..=i + 1 {
                    for j in j - 1..=j + 1 {
                        f[grid[i][j] as usize] += 1;
                    }
                }
                if !f[1..=9].iter().all(|&x| x == 1) {
                    continue;
                }
                if c + grid[i][j - 1] + grid[i][j + 1] == 15
                    && grid[i - 1][j] + grid[i - 1][j - 1] + grid[i - 1][j + 1] == 15
                    && grid[i + 1][j] + grid[i + 1][j - 1] + grid[i + 1][j + 1] == 15
                    && grid[i][j] + grid[i + 1][j] + grid[i - 1][j] == 15
                    && grid[i][j - 1] + grid[i + 1][j - 1] + grid[i - 1][j - 1] == 15
                    && grid[i][j + 1] + grid[i + 1][j + 1] + grid[i - 1][j + 1] == 15
                    && grid[i - 1][j - 1] + grid[i][j] + grid[i + 1][j + 1] == 15
                    && grid[i - 1][j + 1] + grid[i][j] + grid[i + 1][j - 1] == 15
                {
                    result += 1;
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
        let grid = [[4, 3, 8, 4], [9, 5, 1, 9], [2, 7, 6, 2]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(1, Solution::num_magic_squares_inside(grid));
    }

    #[test]
    fn case2() {
        let grid = [[8]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(0, Solution::num_magic_squares_inside(grid));
    }
}
