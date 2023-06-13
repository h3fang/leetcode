pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut m = HashMap::new();
        for v in nums {
            *m.entry(v).or_insert(0) += 1;
        }
        let (mut r, mut prev) = (0, 0);
        for &v in m.values() {
            r += prev * v * (n - prev - v);
            prev += v;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::unequal_triplets(vec![4, 4, 2, 4, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::unequal_triplets(vec![1, 1, 1, 1, 1]));
    }
}
