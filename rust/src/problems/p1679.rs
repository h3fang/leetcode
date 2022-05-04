use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut m = HashMap::new();
        let mut result = 0;
        for n in nums {
            if let Some(c) = m.get_mut(&(k - n)) {
                if *c > 0 {
                    *c -= 1;
                    result += 1;
                    continue;
                }
            }
            *m.entry(n).or_insert(0) += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::max_operations(vec![1, 2, 3, 4], 5));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::max_operations(vec![3, 1, 3, 4, 3], 6));
    }
}
