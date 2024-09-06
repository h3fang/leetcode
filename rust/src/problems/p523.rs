pub struct Solution;

use std::collections::{hash_map::Entry, HashMap};

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut m = HashMap::new();
        m.insert(0, 0);
        let mut sum = 0;
        for (i, n) in nums.iter().enumerate() {
            sum += n;
            let r = sum % k;
            match m.entry(r) {
                Entry::Occupied(e) => {
                    if *e.get() < i {
                        return true;
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(i + 1);
                }
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
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
    }

    #[test]
    fn case2() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
    }

    #[test]
    fn case3() {
        assert!(!Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13));
    }
}
