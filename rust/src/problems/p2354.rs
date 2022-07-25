pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let s = nums.into_iter().collect::<HashSet<_>>();
        let mut count = [0; 61];
        for n in s {
            count[n.count_ones() as usize] += 1;
        }
        let mut result = 0;
        for (a, ca) in count.iter().enumerate() {
            for (b, cb) in count.iter().enumerate() {
                if a + b >= k {
                    result += ca * cb;
                }
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
        assert_eq!(5, Solution::count_excellent_pairs(vec![1, 2, 3, 1], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_excellent_pairs(vec![5, 1, 1], 10));
    }
}
