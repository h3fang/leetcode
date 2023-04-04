pub struct Solution;

impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = stones.len();
        if (n - 1) % (k - 1) != 0 {
            return -1;
        }
        let mut dp = vec![vec![i32::MAX / 2; n]; n];
        let mut sum = vec![0; n];
        let mut curr = 0;
        for (i, &s) in stones.iter().enumerate() {
            dp[i][i] = 0;
            curr += s;
            sum[i] = curr;
        }
        for c in 2..=n {
            for l in 0..n + 1 - c {
                let r = l + c - 1;
                for p in (l..r).step_by(k - 1) {
                    dp[l][r] = dp[l][r].min(dp[l][p] + dp[p + 1][r]);
                }
                if (r - l) % (k - 1) == 0 {
                    dp[l][r] += sum[r] - if l == 0 { 0 } else { sum[l - 1] };
                }
            }
        }
        dp[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(20, Solution::merge_stones(vec![3, 2, 4, 1], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::merge_stones(vec![3, 2, 4, 1], 3));
    }

    #[test]
    fn case3() {
        assert_eq!(25, Solution::merge_stones(vec![3, 5, 1, 2, 6], 3));
    }
}
