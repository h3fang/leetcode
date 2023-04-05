pub struct Solution;

impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut sum = 0;
        for (i, n) in nums.into_iter().enumerate() {
            sum += n as i64;
            result = result.max((sum + i as i64) / (i as i64 + 1));
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::minimize_array_value(vec![3, 7, 1, 6]));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::minimize_array_value(vec![10, 1]));
    }
}
