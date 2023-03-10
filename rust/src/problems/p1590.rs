pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let x = nums.iter().fold(0, |acc, &n| (acc + n) % p);
        if x == 0 {
            return 0;
        }
        let mut m = HashMap::new();
        m.insert(0, -1);
        let mut y = 0;
        let mut result = nums.len() as i32;
        for (i, &n) in nums.iter().enumerate() {
            y = (y + n) % p;
            m.insert(y, i as i32);
            let left = (y - x + p) % p;
            if let Some(j) = m.get(&left) {
                result = result.min(i as i32 - j);
            }
        }
        if result == nums.len() as i32 {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_subarray(vec![3, 1, 4, 2], 6));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_subarray(vec![6, 3, 5, 2], 9));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::min_subarray(vec![1, 2, 3], 3));
    }

    #[test]
    fn case4() {
        assert_eq!(-1, Solution::min_subarray(vec![1, 2, 3], 7));
    }

    #[test]
    fn case5() {
        assert_eq!(
            0,
            Solution::min_subarray(vec![1000000000, 1000000000, 1000000000], 3)
        );
    }
}
