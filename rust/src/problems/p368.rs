pub struct Solution;

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n < 2 {
            return nums;
        }
        nums.sort_unstable();
        let mut dp = vec![(1, -1); n];
        let mut max_len = 1;
        let mut last_idx = 0;
        for (i, n) in nums.iter().enumerate().skip(1) {
            for j in 0..i {
                if n % nums[j] == 0 && dp[j].0 + 1 > dp[i].0 {
                    dp[i] = (dp[j].0 + 1, j as i32);
                }
            }
            if dp[i].0 > max_len {
                max_len = dp[i].0;
                last_idx = i as i32;
            }
        }
        let mut result = Vec::with_capacity(max_len as usize);
        while last_idx >= 0 {
            result.push(nums[last_idx as usize]);
            last_idx = dp[last_idx as usize].1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut result = Solution::largest_divisible_subset(vec![1, 2, 3]);
        result.sort_unstable();
        let expected = [vec![1, 2], vec![1, 3]];
        assert!(expected.contains(&result));
    }

    #[test]
    fn case2() {
        let mut result = Solution::largest_divisible_subset(vec![1, 2, 4, 8]);
        result.sort_unstable();
        let expected = [vec![1, 2, 4, 8]];
        assert!(expected.contains(&result));
    }
}
