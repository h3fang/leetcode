pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::new();
        for n in nums {
            *m.entry(n).or_insert(0) += 1;
        }
        let mut ans = 0;
        for c in m.into_values() {
            if c == 1 {
                return -1;
            }
            ans += (c + 2) / 3;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::min_operations(vec![2, 3, 3, 2, 2, 4, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::min_operations(vec![2, 1, 2, 2, 3, 3]));
    }
}
