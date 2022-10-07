pub struct Solution;

impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        for i in 0..grid.len() - 2 {
            let mut sum = grid[i..i + 3]
                .iter()
                .map(|r| r[0..3].iter().sum::<i32>())
                .sum::<i32>();
            result = result.max(sum - grid[i + 1][0] - grid[i + 1][2]);
            for j in 1..grid[0].len() - 2 {
                sum += grid[i..i + 3]
                    .iter()
                    .map(|r| r[j + 2] - r[j - 1])
                    .sum::<i32>();
                result = result.max(sum - grid[i + 1][j] - grid[i + 1][j + 2]);
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
        let grid = [[6, 2, 1, 3], [4, 2, 1, 5], [9, 2, 8, 7], [4, 1, 2, 9]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(30, Solution::max_sum(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(35, Solution::max_sum(grid));
    }
}
