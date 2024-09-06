pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut reach = 1;
        for (i, a) in nums.iter().enumerate() {
            if reach <= i {
                return false;
            }
            reach = reach.max(i + 1 + *a as usize);
            if reach >= n {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert!(Solution::can_jump(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![3, 2, 1, 0, 4];
        assert!(!Solution::can_jump(nums));
    }
}
