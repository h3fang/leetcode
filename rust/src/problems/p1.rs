pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut s = HashMap::new();
        for (i, n) in nums.into_iter().enumerate() {
            if let Some(&j) = s.get(&(target - n)) {
                return vec![i as i32, j];
            }
            s.insert(n, i as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let r = Solution::two_sum(nums.to_vec(), target);
        assert!(r.len() == 2 && nums[r[0] as usize] + nums[r[1] as usize] == target);
    }

    #[test]
    fn case2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let r = Solution::two_sum(nums.to_vec(), target);
        assert!(r.len() == 2 && nums[r[0] as usize] + nums[r[1] as usize] == target);
    }

    #[test]
    fn case3() {
        let nums = vec![3, 3];
        let target = 6;
        let r = Solution::two_sum(nums.to_vec(), target);
        assert!(r.len() == 2 && nums[r[0] as usize] + nums[r[1] as usize] == target);
    }
}
