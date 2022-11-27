pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut f = vec![HashMap::new(); nums.len()];
        for i in 1..nums.len() {
            for j in 0..i {
                let d = nums[i] as i64 - nums[j] as i64;
                let c = f[j].get(&(d)).cloned().unwrap_or(0);
                *f[i].entry(d).or_default() += c + 1;
                result += c;
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
            7,
            Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            16,
            Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7])
        );
    }
}
