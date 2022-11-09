pub struct Solution;

impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![[1; 4]; n]; n];
        for m in mines {
            dp[m[0] as usize][m[1] as usize] = [0; 4];
        }
        for i in 0..n {
            for j in 0..n {
                if i > 0 && dp[i][j][0] != 0 {
                    dp[i][j][0] = dp[i - 1][j][0] + 1;
                }
                if j > 0 && dp[i][j][1] != 0 {
                    dp[i][j][1] = dp[i][j - 1][1] + 1;
                }
            }
        }
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if i + 1 < n && dp[i][j][2] != 0 {
                    dp[i][j][2] = dp[i + 1][j][2] + 1;
                }
                if j + 1 < n && dp[i][j][3] != 0 {
                    dp[i][j][3] = dp[i][j + 1][3] + 1;
                }
            }
        }
        let mut result = 0;
        for r in dp {
            for c in r {
                let d = *c.iter().min().unwrap();
                result = result.max(d);
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
        let n = 5;
        let mines = [[4, 2]].iter().map(|m| m.to_vec()).collect();
        assert_eq!(2, Solution::order_of_largest_plus_sign(n, mines));
    }

    #[test]
    fn case2() {
        let n = 1;
        let mines = [[0, 0]].iter().map(|m| m.to_vec()).collect();
        assert_eq!(0, Solution::order_of_largest_plus_sign(n, mines));
    }
}
