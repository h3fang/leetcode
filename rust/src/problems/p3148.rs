pub struct Solution;

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut f = vec![i32::MAX; n];
        let mut result = i32::MIN;
        for r in grid.iter() {
            let mut pre_min = i32::MAX;
            for (j, &c) in r.iter().enumerate() {
                result = result.max(c - pre_min.min(f[j]));
                f[j] = f[j].min(c);
                pre_min = pre_min.min(f[j]);
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
        let grid = [[9, 5, 7, 3], [8, 9, 6, 1], [6, 7, 14, 3], [2, 5, 3, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(9, Solution::max_score(grid));
    }

    #[test]
    fn case2() {
        let grid = [[4, 3, 2], [3, 2, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(-1, Solution::max_score(grid));
    }
}
