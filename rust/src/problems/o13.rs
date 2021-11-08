pub struct Solution;

impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        fn sum(mut i: usize) -> usize {
            let mut c = 0;
            while i > 0 {
                c += i % 10;
                i /= 10;
            }
            c
        }

        let k = k as usize;
        let m = m as usize;
        let n = n as usize;

        let mut dp = vec![false; n];
        dp[0] = true;
        let mut result = 0;
        for i in 0..m {
            for j in 0..n {
                if sum(i) + sum(j) <= k && (dp[j] || (j > 0 && dp[j - 1])) {
                    dp[j] = true;
                    result += 1;
                } else {
                    dp[j] = false;
                }
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
        assert_eq!(3, Solution::moving_count(2, 3, 1));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::moving_count(3, 1, 0));
    }
}
