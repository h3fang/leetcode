pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut sum: HashMap<i32, i32> = HashMap::new();
        let mut result = -1;
        for &n in &nums {
            let mut e = n;
            let mut s = 0;
            while e > 0 {
                s += e % 10;
                e /= 10;
            }
            if let Some(&max) = sum.get(&s) {
                result = result.max(n + max);
                if n > max {
                    sum.insert(s, n);
                }
            } else {
                sum.insert(s, n);
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
        assert_eq!(54, Solution::maximum_sum(vec![18, 43, 36, 13, 7]));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::maximum_sum(vec![10, 12, 19, 14]));
    }
}
