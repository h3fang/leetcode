pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut m = HashMap::with_capacity(nums.len());
        let (mut ans, mut l, mut pairs) = (0, 0, 0);
        for &x in &nums {
            let e = m.entry(x).or_insert(0);
            pairs += *e;
            *e += 1;
            while pairs >= k {
                let e = m.entry(nums[l]).or_insert(0);
                *e -= 1;
                pairs -= *e;
                l += 1;
            }
            ans += l as i64;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::count_good(vec![1, 1, 1, 1, 1], 10));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::count_good(vec![3, 1, 4, 3, 2, 2, 4], 2));
    }
}
