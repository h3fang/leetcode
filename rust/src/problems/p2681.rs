pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn sum_of_power(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut pre = 0;
        let mut result = 0;
        for n in nums {
            let n = n as i64;
            let dp = (n + pre) % MOD;
            pre = (pre + dp) % MOD;
            result = (result + (dp * n) % MOD * n) % MOD;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(141, Solution::sum_of_power(vec![2, 1, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::sum_of_power(vec![1, 1, 1]));
    }
}
