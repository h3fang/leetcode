pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut ids = (0..nums1.len()).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|&i| -nums2[i]);
        let mut q = BinaryHeap::new();
        let mut sum: i64 = 0;
        for &i in &ids[..k] {
            q.push(Reverse(nums1[i]));
            sum += nums1[i] as i64;
        }
        let mut result = sum * nums2[ids[k - 1]] as i64;
        for &i in &ids[k..] {
            if nums1[i] > q.peek().unwrap().0 {
                sum += nums1[i] as i64 - q.peek().unwrap().0 as i64;
                q.pop();
                q.push(Reverse(nums1[i]));
                result = result.max(sum * nums2[i] as i64);
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
        assert_eq!(
            12,
            Solution::max_score(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            30,
            Solution::max_score(vec![4, 2, 3, 1, 1], vec![7, 5, 10, 9, 6], 1)
        );
    }
}
