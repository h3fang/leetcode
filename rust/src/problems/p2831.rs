pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut indices: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut result = 0;
        for (i, &x) in nums.iter().enumerate() {
            indices.entry(x).or_default().push(i);
        }
        for ind in indices.values() {
            let mut i = 0;
            for (j, &idx) in ind.iter().enumerate() {
                while idx - ind[i] - (j - i) > k {
                    i += 1;
                }
                result = result.max(j + 1 - i);
            }
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
            3,
            Solution::longest_equal_subarray(vec![1, 3, 2, 3, 1, 3], 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            4,
            Solution::longest_equal_subarray(vec![1, 1, 2, 2, 1, 1], 2)
        );
    }
}
