use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut m = HashMap::new();
        for n1 in &nums1 {
            for n2 in &nums2 {
                *m.entry(n1 + n2).or_insert(0) += 1;
            }
        }

        let mut result = 0;
        for n1 in &nums3 {
            for n2 in &nums4 {
                if let Some(c) = m.get(&(-(n1 + n2))) {
                    result += c;
                }
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
        let nums1 = vec![1, 2];
        let nums2 = vec![-2, -1];
        let nums3 = vec![-1, 2];
        let nums4 = vec![0, 2];
        assert_eq!(2, Solution::four_sum_count(nums1, nums2, nums3, nums4));
    }

    #[test]
    fn case2() {
        let nums1 = vec![0];
        let nums2 = vec![0];
        let nums3 = vec![0];
        let nums4 = vec![0];
        assert_eq!(1, Solution::four_sum_count(nums1, nums2, nums3, nums4));
    }
}
