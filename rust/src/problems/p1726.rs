pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::new();
        for (i, &a) in nums.iter().enumerate() {
            for &b in &nums[i + 1..] {
                *m.entry(a * b).or_insert(0) += 1;
            }
        }
        m.values().map(|&c| 4 * c * (c - 1)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(8, Solution::tuple_same_product(vec![2, 3, 4, 6]));
    }

    #[test]
    fn case2() {
        assert_eq!(16, Solution::tuple_same_product(vec![1, 2, 4, 5, 10]));
    }
}
