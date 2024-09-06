pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = 0;
        let mut max = nums[0];
        for &n in &nums {
            sum += n;
            max = max.max(n);
        }
        if sum % 2 == 1 {
            return false;
        }
        let sum = (sum / 2) as usize;
        if max as usize > sum {
            return false;
        }
        let mut dp = vec![false; sum + 1];
        dp[0] = true;
        dp[nums[0] as usize] = true;
        for &n in &nums[1..] {
            if dp[sum] {
                return true;
            }
            let n = n as usize;
            for s in (n..=sum).rev() {
                dp[s] |= dp[s - n];
            }
        }

        dp[sum]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_partition(vec![1, 5, 11, 5]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
    }
}
