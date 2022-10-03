pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let mut dp = vec![vec![0; target as usize + 1]; n as usize + 1];
        dp[1][1..=k.min(target) as usize]
            .iter_mut()
            .for_each(|e| *e = 1);
        for i in 2..=n {
            for j in 1..=target {
                dp[i as usize][j as usize] = dp[i as usize - 1]
                    [(j - k).max(1) as usize..j as usize]
                    .iter()
                    .fold(0, |acc, e| (acc + e) % MOD);
            }
        }
        dp[n as usize][target as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::num_rolls_to_target(1, 6, 3));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::num_rolls_to_target(2, 6, 7));
    }

    #[test]
    fn case3() {
        assert_eq!(222616187, Solution::num_rolls_to_target(30, 30, 500));
    }
}
