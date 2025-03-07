pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut f = HashMap::new();
        let mut i = 0;
        let mut result = 0;
        for (j, x) in nums.iter().enumerate() {
            *f.entry(x).or_insert(0) += 1;
            while *f.get(&x).unwrap() > k {
                *f.get_mut(&nums[i]).unwrap() -= 1;
                i += 1;
            }
            result = result.max(j - i + 1);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            6,
            Solution::max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::max_subarray_length(vec![1, 2, 1, 2, 1, 2, 1, 2], 1)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            4,
            Solution::max_subarray_length(vec![5, 5, 5, 5, 5, 5, 5], 4)
        );
    }
}
