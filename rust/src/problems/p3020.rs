pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::with_capacity(nums.len());
        for x in nums {
            *m.entry(x).or_default() += 1;
        }

        let mut ans = (*m.get(&1).unwrap_or(&0) - 1) | 1;

        for &x in m.keys() {
            if x == 1 {
                continue;
            }
            let (mut x, mut len) = (x, 0);
            while m.get(&x).is_some_and(|&c| c >= 2) {
                len += 2;
                x *= x;
            }
            if m.contains_key(&x) {
                len += 1;
            } else {
                len -= 1;
            }
            ans = ans.max(len);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::maximum_length(vec![5, 4, 1, 2, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::maximum_length(vec![1, 3, 2, 4]));
    }
}
