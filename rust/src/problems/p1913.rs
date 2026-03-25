pub struct Solution;

impl Solution {
    pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        nums[n - 1] * nums[n - 2] - nums[0] * nums[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(34, Solution::max_product_difference(vec![5, 6, 2, 7, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            64,
            Solution::max_product_difference(vec![4, 2, 5, 9, 7, 4, 8])
        );
    }
}
