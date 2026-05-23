pub struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut valid = nums[0] >= nums[nums.len() - 1];
        for w in nums.windows(2) {
            if w[0] > w[1] {
                if !valid {
                    return false;
                }
                valid = false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check(vec![3, 4, 5, 1, 2]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check(vec![2, 1, 3, 4]));
    }

    #[test]
    fn case3() {
        assert!(Solution::check(vec![1, 2, 3]));
    }
}
