pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        result[0] = 1;
        for i in 1..nums.len() {
            result[i] = result[i - 1] * nums[i - 1];
        }
        let mut right = 1;
        for i in (0..nums.len() - 1).rev() {
            right *= nums[i + 1];
            result[i] *= right;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![24, 12, 8, 6],
            Solution::product_except_self(vec![1, 2, 3, 4])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![1], Solution::product_except_self(vec![24]));
    }
}
