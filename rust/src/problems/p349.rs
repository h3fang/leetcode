pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let s1: HashSet<i32> = HashSet::from_iter(nums1);
        let s2 = HashSet::from_iter(nums2);
        s1.intersection(&s2).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut result = Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]);
        result.sort_unstable();
        assert_eq!(vec![2], result);
    }

    #[test]
    fn case2() {
        let mut result = Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        result.sort_unstable();
        assert_eq!(vec![4, 9], result);
    }
}
