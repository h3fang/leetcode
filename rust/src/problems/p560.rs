use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut prefix: HashMap<i32, i32> = HashMap::new();
        let mut pre = 0;
        prefix.insert(pre, 1);
        for n in &nums {
            pre += n;
            result += prefix.get(&(pre - k)).unwrap_or(&0);
            *prefix.entry(pre).or_default() += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::subarray_sum(vec![1, 1, 1], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::subarray_sum(vec![1, 2, 3], 3));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::subarray_sum(vec![1], 0));
    }
}
