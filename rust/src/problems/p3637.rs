pub struct Solution;

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        if nums[0] >= nums[1] {
            return false;
        }
        let mut count = 1;
        for (i, w) in nums.windows(2).enumerate().skip(1) {
            if w[0] == w[1] {
                return false;
            }
            if (nums[i - 1] < w[0]) != (w[0] < w[1]) {
                count += 1;
            }
        }
        count == 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_trionic(vec![1, 3, 5, 4, 2, 6]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_trionic(vec![2, 1, 3]));
    }
}
