pub struct Solution;

impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut dp = vec![vec![-1; n]; n];
        dp[0][n - 1] = grid[0][0] + grid[0][n - 1];
        for row in &grid[1..] {
            let old = dp.clone();
            for (i, first) in old.iter().enumerate() {
                for (j, &curr) in first.iter().enumerate() {
                    if curr == -1 {
                        continue;
                    }
                    for i_next in i.saturating_sub(1)..=(i + 1).min(n - 1) {
                        for j_next in j.saturating_sub(1)..=(j + 1).min(n - 1) {
                            let next = curr
                                + if i_next == j_next {
                                    row[i_next]
                                } else {
                                    row[i_next] + row[j_next]
                                };
                            dp[i_next][j_next] = dp[i_next][j_next].max(next);
                        }
                    }
                }
            }
        }
        dp.into_iter().flatten().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[3, 1, 1], [2, 5, 1], [1, 5, 5], [2, 1, 1]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(24, Solution::cherry_pickup(grid));
    }

    #[test]
    fn case2() {
        let grid = [
            [1, 0, 0, 0, 0, 0, 1],
            [2, 0, 0, 0, 0, 3, 0],
            [2, 0, 9, 0, 0, 0, 0],
            [0, 3, 0, 5, 4, 0, 0],
            [1, 0, 2, 3, 0, 0, 6],
        ];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(28, Solution::cherry_pickup(grid));
    }
}
