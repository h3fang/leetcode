pub struct Solution;

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn max(grid: &[Vec<i32>], i: usize, j: usize) -> i32 {
            let mut max = 0;
            for r in &grid[i - 1..=i + 1] {
                for &c in &r[j - 1..=j + 1] {
                    max = max.max(c);
                }
            }
            max
        }
        let n = grid.len();
        let mut result = vec![vec![0; n - 2]; n - 2];
        for i in 1..n - 1 {
            for j in 1..n - 1 {
                result[i - 1][j - 1] = max(&grid, i, j);
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
        let grid = [[9, 9, 8, 1], [5, 6, 2, 6], [8, 2, 6, 4], [6, 2, 2, 2]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        let expected = [[9, 9], [8, 6]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::largest_local(grid));
    }
}
