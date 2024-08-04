pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let n = n as usize;
        let mut sums = Vec::with_capacity(n * (n + 1) / 2);
        for i in 0..n {
            let mut sum = 0;
            for &y in &nums[i..] {
                sum = (sum + y) % MOD;
                sums.push(sum);
            }
        }
        sums.sort_unstable();
        let mut result = 0;
        for &x in &sums[left as usize - 1..right as usize] {
            result = (result + x) % MOD;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(13, Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::range_sum(vec![1, 2, 3, 4], 4, 3, 4));
    }

    #[test]
    fn case3() {
        assert_eq!(50, Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 10));
    }
}
