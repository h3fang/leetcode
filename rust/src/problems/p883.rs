pub struct Solution;

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = grid.iter().flatten().map(|&e| e.min(1)).sum::<i32>();
        result += grid.iter().map(|yz| yz.iter().max().unwrap()).sum::<i32>();
        result += (0..grid[0].len())
            .map(|j| (0..grid.len()).map(|i| grid[i][j]).max().unwrap())
            .sum::<i32>();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 2], [3, 4]];
        let grid = grid.iter().map(|e| e.to_vec()).collect();
        assert_eq!(17, Solution::projection_area(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 0], [0, 2]];
        let grid = grid.iter().map(|e| e.to_vec()).collect();
        assert_eq!(8, Solution::projection_area(grid));
    }
}
