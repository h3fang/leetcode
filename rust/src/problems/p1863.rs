pub struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let result = nums.into_iter().fold(0, |acc, x| acc | x);
        result << (n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::subset_xor_sum(vec![1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(28, Solution::subset_xor_sum(vec![5, 1, 6]));
    }

    #[test]
    fn case3() {
        assert_eq!(480, Solution::subset_xor_sum(vec![3, 4, 5, 6, 7, 8]));
    }
}
