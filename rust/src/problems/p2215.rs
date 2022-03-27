use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let s1 = nums1.iter().cloned().collect::<HashSet<_>>();
        let s2 = nums2.iter().cloned().collect::<HashSet<_>>();
        let c1 = nums1
            .iter()
            .filter(|&n| !s2.contains(n))
            .cloned()
            .collect::<HashSet<_>>();
        let c2 = nums2
            .iter()
            .filter(|&n| !s1.contains(n))
            .cloned()
            .collect::<HashSet<_>>();
        vec![c1.iter().cloned().collect(), c2.iter().cloned().collect()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];
        let mut expected = vec![vec![1, 3], vec![4, 6]];
        for e in expected.iter_mut() {
            e.sort_unstable();
        }
        let mut result = Solution::find_difference(nums1, nums2);
        for e in result.iter_mut() {
            e.sort_unstable();
        }
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];
        let mut expected = vec![vec![3], vec![]];
        for e in expected.iter_mut() {
            e.sort_unstable();
        }
        let mut result = Solution::find_difference(nums1, nums2);
        for e in result.iter_mut() {
            e.sort_unstable();
        }
        assert_eq!(expected, result);
    }
}
