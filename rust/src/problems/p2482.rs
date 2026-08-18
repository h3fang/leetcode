pub struct Solution;

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut result = vec![vec![0; n]; m];
        let row = grid.iter().map(|r| r.iter().sum()).collect::<Vec<i32>>();
        let col = (0..n)
            .map(|j| grid.iter().map(|r| r[j]).sum())
            .collect::<Vec<i32>>();
        for (i, r) in result.iter_mut().enumerate() {
            for (j, e) in r.iter_mut().enumerate() {
                *e = 2 * (row[i] + col[j]) - (m + n) as i32;
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
        let grid = [[0, 1, 1], [1, 0, 1], [0, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let expected = [[0, 0, 4], [0, 0, 4], [-2, -2, 2]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::ones_minus_zeros(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 1, 1], [1, 1, 1]].iter().map(|r| r.to_vec()).collect();
        let expected = [[5, 5, 5], [5, 5, 5]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::ones_minus_zeros(grid));
    }
}
