pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut max, mut start, mut end) = (i32::MIN, 0, i32::MAX);
        let mut q = BinaryHeap::new();
        for (i, v) in nums.iter().enumerate() {
            q.push((Reverse(v[0]), i, 0));
            max = max.max(v[0]);
        }
        while let Some((Reverse(min), i, j)) = q.pop() {
            if max - min < end - start {
                start = min;
                end = max;
            }
            if j + 1 < nums[i].len() {
                let v = nums[i][j + 1];
                q.push((Reverse(v), i, j + 1));
                max = max.max(v);
            } else {
                break;
            }
        }
        vec![start, end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![
            vec![4, 10, 15, 24, 26],
            vec![0, 9, 12, 20],
            vec![5, 18, 22, 30],
        ];
        assert_eq!(vec![20, 24], Solution::smallest_range(nums));
    }

    #[test]
    fn case2() {
        let nums = [[1, 2, 3], [1, 2, 3], [1, 2, 3]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        assert_eq!(vec![1, 1], Solution::smallest_range(nums));
    }
}
