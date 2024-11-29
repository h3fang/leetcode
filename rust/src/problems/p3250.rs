pub struct Solution;

const MOD: i64 = 10_0000_0007;

fn qpow(a: i64, mut b: i64) -> i64 {
    let mut a = a % MOD;
    let mut result = 1;
    while b > 0 {
        if b & 1 == 1 {
            result = (result * a) % MOD;
        }
        a = (a * a) % MOD;
        b >>= 1;
    }
    result
}

impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = (nums[n - 1] - nums.windows(2).map(|w| (w[1] - w[0]).max(0)).sum::<i32>()) as i64;
        if m >= 0 {
            let n = m + n as i64;
            let numerator = (0..m).fold(1, |acc, i| acc * (n - i) % MOD);
            let denominator = (1..=m).fold(1, |acc, i| (acc * i) % MOD);
            ((numerator * qpow(denominator, MOD - 2)) % MOD) as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::count_of_pairs(vec![2, 3, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(126, Solution::count_of_pairs(vec![5, 5, 5, 5]));
    }
}
