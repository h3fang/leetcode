use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut q = nums1
            .iter()
            .enumerate()
            .take(k)
            .map(|(i, &n)| (Reverse(n + nums2[0]), i, 0))
            .collect::<BinaryHeap<_>>();
        let mut result = Vec::with_capacity(k);
        while let Some((_, i, j)) = q.pop() {
            if result.len() == k {
                break;
            }
            result.push(vec![nums1[i], nums2[j]]);
            if j + 1 < nums2.len() {
                q.push((Reverse(nums1[i] + nums2[j + 1]), i, j + 1));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let expected = [[1, 2], [1, 4], [1, 6]];
        let expected = expected.iter().map(|p| p.to_vec()).collect::<Vec<_>>();
        assert_eq!(
            expected,
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3)
        );
    }

    #[test]
    fn case2() {
        let expected = [[1, 3], [2, 3], [1, 5]];
        let expected = expected.iter().map(|p| p.to_vec()).collect::<Vec<_>>();
        assert_eq!(
            expected,
            Solution::k_smallest_pairs(vec![1, 2, 4, 5, 6], vec![3, 5, 7, 9], 3)
        );
    }
}
