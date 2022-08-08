pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let n = nums.len() as i64;
        let mut result = n * (n - 1) / 2;
        let mut m = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let e = m.entry(num - i as i32).or_insert(0);
            result -= *e;
            *e += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::count_bad_pairs(vec![4, 1, 3, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_bad_pairs(vec![1, 2, 3, 4, 5]));
    }
}
