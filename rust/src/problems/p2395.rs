pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for w in nums.windows(2) {
            let s = w[0] + w[1];
            if set.contains(&s) {
                return true;
            }
            set.insert(s);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::find_subarrays(vec![4, 2, 4]));
    }
}
