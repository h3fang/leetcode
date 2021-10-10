use std::{collections::HashSet, iter::FromIterator};

pub struct Solution;

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<i32> = HashSet::from_iter(nums1);
        let set2: HashSet<i32> = HashSet::from_iter(nums2);
        let set3: HashSet<i32> = HashSet::from_iter(nums3);
        let part1 = set1.intersection(&set2).cloned().collect::<HashSet<_>>();
        let set12 = set1.union(&set2).cloned().collect::<HashSet<_>>();
        let part2 = set12.intersection(&set3).cloned().collect::<HashSet<_>>();
        part1.union(&part2).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums1 = vec![1, 1, 3, 2];
        let nums2 = vec![2, 3];
        let nums3 = vec![3];
        let mut expected = vec![3, 2];
        expected.sort_unstable();
        let mut result = Solution::two_out_of_three(nums1, nums2, nums3);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let nums1 = vec![3, 1];
        let nums2 = vec![2, 3];
        let nums3 = vec![1, 2];
        let mut expected = vec![1, 3, 2];
        expected.sort_unstable();
        let mut result = Solution::two_out_of_three(nums1, nums2, nums3);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
