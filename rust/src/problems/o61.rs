pub struct Solution;

impl Solution {
    pub fn is_straight(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        let mut zeros = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                zeros += 1;
            } else if i > 0 && nums[i] == nums[i - 1] {
                return false;
            }
        }
        nums[4] - nums[zeros] < 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_straight(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn case2() {
        assert!(Solution::is_straight(vec![0, 0, 1, 2, 5]));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_straight(vec![0, 0, 2, 2, 5]));
    }

    #[test]
    fn case4() {
        assert!(Solution::is_straight(vec![0, 0, 8, 5, 4]));
    }
}
