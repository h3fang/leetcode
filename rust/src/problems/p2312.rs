pub struct Solution;

impl Solution {
    pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for p in prices {
            dp[p[0] as usize][p[1] as usize] = p[2] as i64;
        }
        for h in 1..=m {
            for w in 1..=n {
                for k in 1..=h / 2 {
                    dp[h][w] = dp[h][w].max(dp[k][w] + dp[h - k][w]);
                }
                for k in 1..=w / 2 {
                    dp[h][w] = dp[h][w].max(dp[h][k] + dp[h][w - k]);
                }
            }
        }
        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let m = 3;
        let n = 5;
        let prices = [[1, 4, 2], [2, 2, 7], [2, 1, 3]];
        let prices = prices.iter().map(|p| p.to_vec()).collect();
        assert_eq!(19, Solution::selling_wood(m, n, prices));
    }

    #[test]
    fn case2() {
        let m = 4;
        let n = 6;
        let prices = [[3, 2, 10], [1, 4, 2], [4, 1, 3]];
        let prices = prices.iter().map(|p| p.to_vec()).collect();
        assert_eq!(32, Solution::selling_wood(m, n, prices));
    }
}
