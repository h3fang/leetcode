pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut m = HashMap::new();
        let (mut sum, mut result) = (0, 0);
        for x in nums {
            *m.entry(sum).or_insert(0) += 1;
            sum += x;
            if let Some(&b) = m.get(&(sum - goal)) {
                result += b;
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
        assert_eq!(4, Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(15, Solution::num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0));
    }
}
