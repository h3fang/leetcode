pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut m: BTreeMap<i32, u32> = BTreeMap::new();
        let (mut result, mut l) = (0, 0);
        for (r, &x) in nums.iter().enumerate() {
            *m.entry(x).or_default() += 1;
            while m.last_key_value().unwrap().0 - m.first_key_value().unwrap().0 > limit {
                let e = m.entry(nums[l]).or_default();
                *e -= 1;
                if *e == 0 {
                    m.remove(&nums[l]);
                }
                l += 1;
            }
            result = result.max((r - l + 1) as i32);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::longest_subarray(vec![8, 2, 4, 7], 4));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::longest_subarray(vec![10, 1, 2, 4, 7, 2], 5));
    }

    #[test]
    fn case3() {
        assert_eq!(
            3,
            Solution::longest_subarray(vec![4, 2, 2, 2, 4, 4, 2, 2], 0)
        );
    }
}
