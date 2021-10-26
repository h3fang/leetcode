use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut pos = HashMap::new();
        let mut stack = Vec::new();
        for n in nums2 {
            while let Some(&e) = stack.last() {
                if n < e {
                    break;
                }
                stack.pop();
                pos.insert(e, n);
            }
            stack.push(n);
        }
        nums1
            .iter()
            .map(|n1| pos.get(n1).cloned().unwrap_or(-1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        let expected = vec![-1, 3, -1];
        assert_eq!(expected, Solution::next_greater_element(nums1, nums2));
    }

    #[test]
    fn case2() {
        let nums1 = vec![2, 4];
        let nums2 = vec![1, 2, 3, 4];
        let expected = vec![3, -1];
        assert_eq!(expected, Solution::next_greater_element(nums1, nums2));
    }
}
