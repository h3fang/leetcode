pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut m = BTreeMap::new();
        let (mut l, mut result) = (0, 0);
        for (r, &x) in nums.iter().enumerate() {
            *m.entry(x).or_insert(0) += 1;
            while m.keys().next_back().unwrap() - m.keys().next().unwrap() > 2 {
                let e = m.entry(nums[l]).or_default();
                *e -= 1;
                if *e == 0 {
                    m.remove(&nums[l]);
                }
                l += 1;
            }
            result += (r - l + 1) as i64;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(8, Solution::continuous_subarrays(vec![5, 4, 2, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::continuous_subarrays(vec![1, 2, 3]));
    }
}
