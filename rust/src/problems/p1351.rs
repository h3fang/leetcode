pub struct Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let i = grid.partition_point(|r| r[0] >= 0);
        let r = (0..i)
            .map(|i| n - grid[i].partition_point(|&c| c >= 0))
            .sum::<usize>();
        (r + (m - i) * n) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [
            [4, 3, 2, -1],
            [3, 2, 1, -1],
            [1, 1, -1, -2],
            [-1, -1, -2, -3],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(8, Solution::count_negatives(grid));
    }

    #[test]
    fn case2() {
        let grid = [[3, 2], [1, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(0, Solution::count_negatives(grid));
    }
}
