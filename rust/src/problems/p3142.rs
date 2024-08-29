pub struct Solution;

impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        (0..grid[0].len()).all(|j| grid.iter().all(|r| r[j] == grid[0][j]))
            && grid.iter().all(|r| r.windows(2).all(|w| w[0] != w[1]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 0, 2], [1, 0, 2]].iter().map(|r| r.to_vec()).collect();
        assert!(Solution::satisfies_conditions(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 1, 1], [0, 0, 0]].iter().map(|r| r.to_vec()).collect();
        assert!(!Solution::satisfies_conditions(grid));
    }

    #[test]
    fn case3() {
        let grid = [[1], [2], [3]].iter().map(|r| r.to_vec()).collect();
        assert!(!Solution::satisfies_conditions(grid));
    }
}
