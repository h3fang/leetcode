pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let k = k as usize;
        let m = pizza.len();
        let n = pizza[0].len();
        let mut sub = vec![vec![0; n + 1]; m + 1];
        let mut dp = vec![vec![vec![0; k]; n]; m];
        for (i, row) in pizza.iter().enumerate().rev() {
            for (j, &p) in row.as_bytes().iter().enumerate().rev() {
                sub[i][j] =
                    sub[i + 1][j] + sub[i][j + 1] - sub[i + 1][j + 1] + i32::from(p == b'A');
                dp[i][j][0] = i32::from(sub[i][j] > 0);
            }
        }
        for t in 1..k {
            for i in (0..m - 1).rev() {
                for j in (0..n - 1).rev() {
                    for i1 in i + 1..m {
                        if sub[i][j] - sub[i1][j] > 0 {
                            dp[i][j][t] = (dp[i][j][t] + dp[i1][j][t - 1]) % MOD;
                        }
                    }
                    for j1 in j + 1..n {
                        if sub[i][j] - sub[i][j1] > 0 {
                            dp[i][j][t] = (dp[i][j][t] + dp[i][j1][t - 1]) % MOD;
                        }
                    }
                }
            }
        }
        dp[0][0][k - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let pizza = ["A..", "AAA", "..."]
            .iter()
            .map(|r| r.to_string())
            .collect();
        assert_eq!(3, Solution::ways(pizza, 3));
    }

    #[test]
    fn case2() {
        let pizza = ["A..", "AA.", "..."]
            .iter()
            .map(|r| r.to_string())
            .collect();
        assert_eq!(1, Solution::ways(pizza, 3));
    }

    #[test]
    fn case3() {
        let pizza = ["A..", "A..", "..."]
            .iter()
            .map(|r| r.to_string())
            .collect();
        assert_eq!(1, Solution::ways(pizza, 1));
    }
}
