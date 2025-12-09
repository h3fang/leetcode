pub struct Solution;

use std::collections::HashMap;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut c1 = HashMap::with_capacity(2 * n);
        let mut c12 = HashMap::with_capacity(2 * n);
        let mut ans = 0;

        for x in nums {
            if x % 2 == 0 {
                ans = (ans + c12.get(&(x / 2)).unwrap_or(&0)) % MOD;
            }
            *c12.entry(x).or_insert(0) += c1.get(&(2 * x)).unwrap_or(&0);
            *c1.entry(x).or_insert(0) += 1;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::special_triplets(vec![6, 3, 6]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::special_triplets(vec![0, 1, 0, 0]));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::special_triplets(vec![8, 4, 2, 8, 4]));
    }
}
