pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn sum_subseq_widths(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut result = 0;
        let mut x = nums[0] as i64;
        let mut y = 2;
        for &n in &nums[1..] {
            let n = n as i64;
            result = (result + n * (y - 1) - x) % MOD;
            x = (x * 2 + n) % MOD;
            y = (y * 2) % MOD;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::sum_subseq_widths(vec![2, 1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::sum_subseq_widths(vec![2]));
    }
}
