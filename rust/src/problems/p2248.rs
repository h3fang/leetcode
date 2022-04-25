use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut first = nums[0].iter().cloned().collect::<HashSet<_>>();
        nums.iter().for_each(|arr| {
            let arr = arr.iter().cloned().collect::<HashSet<_>>();
            first = first.intersection(&arr).cloned().collect();
        });
        let mut result = first.into_iter().collect::<Vec<_>>();
        result.sort_unstable();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![vec![3, 1, 2, 4, 5], vec![1, 2, 3, 4], vec![3, 4, 5, 6]];
        assert_eq!(vec![3, 4], Solution::intersection(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(vec![0i32; 0], Solution::intersection(nums));
    }
}
