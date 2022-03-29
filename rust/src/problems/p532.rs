use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut c = HashMap::new();
        for n in nums {
            *c.entry(n).or_insert(0) += 1;
        }
        let mut result = 0;
        if k == 0 {
            for v in c.values() {
                if *v > 1 {
                    result += 1;
                }
            }
        } else {
            for i in c.keys() {
                if c.contains_key(&(i + k)) {
                    result += 1;
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
        assert_eq!(2, Solution::find_pairs(vec![3, 1, 4, 1, 5], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::find_pairs(vec![1, 2, 3, 4, 5], 1));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::find_pairs(vec![1, 3, 1, 5, 4], 0));
    }
}
