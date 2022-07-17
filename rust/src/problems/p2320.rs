pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn count_house_placements(n: i32) -> i32 {
        let mut dp = [1i64; 4];
        for _ in 1..n {
            let d0 = dp.iter().sum::<i64>() % MOD;
            let d1 = (dp[0] + dp[2]) % MOD;
            let d2 = (dp[0] + dp[1]) % MOD;
            let d3 = dp[0];
            dp = [d0, d1, d2, d3];
        }
        let m = dp.iter().sum::<i64>();
        (m % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::count_house_placements(1));
    }

    #[test]
    fn case2() {
        assert_eq!(9, Solution::count_house_placements(2));
    }

    #[test]
    fn case3() {
        assert_eq!(25, Solution::count_house_placements(3));
    }

    #[test]
    fn case4() {
        assert_eq!(500478595, Solution::count_house_placements(1000));
    }
}
