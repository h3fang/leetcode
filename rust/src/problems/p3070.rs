pub struct Solution;

impl Solution {
    pub fn count_submatrices(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ans = 0;
        for i in 0..grid.len() {
            let mut sum = 0;
            for j in 0..grid[0].len() {
                grid[i][j] += if i > 0 { grid[i - 1][j] } else { 0 };
                sum += grid[i][j];
                if sum <= k {
                    ans += 1;
                } else {
                    break;
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
        let grid = [[7, 6, 3], [6, 6, 1]].iter().map(|r| r.to_vec()).collect();
        let k = 18;
        assert_eq!(4, Solution::count_submatrices(grid, k));
    }

    #[test]
    fn case2() {
        let grid = [[7, 2, 9], [1, 5, 0], [2, 6, 6]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let k = 20;
        assert_eq!(6, Solution::count_submatrices(grid, k));
    }
}
