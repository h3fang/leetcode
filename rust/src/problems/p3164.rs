pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut f1 = HashMap::new();
        for x in nums1 {
            if x % k == 0 {
                *f1.entry(x / k).or_insert(0) += 1;
            }
        }
        if f1.is_empty() {
            return 0;
        }
        let u = if let Some(&u) = f1.keys().max() {
            u
        } else {
            return 0;
        };
        let mut f2 = HashMap::new();
        for x in nums2 {
            *f2.entry(x).or_insert(0) += 1;
        }
        let mut result = 0;
        for (b, c2) in f2 {
            let mut c1 = 0;
            for a in (b..=u).step_by(b as usize) {
                if let Some(&c) = f1.get(&a) {
                    c1 += c as i64;
                }
            }
            result += c1 * c2 as i64;
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
            5,
            Solution::number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::number_of_pairs(vec![1, 2, 4, 12], vec![2, 4], 3)
        );
    }
}
