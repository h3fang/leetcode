pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut pre = nums[0];
        for &n in &nums[1..] {
            pre = n.max(pre + 1);
            result += pre - n;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::min_operations(vec![1, 1, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(14, Solution::min_operations(vec![1, 5, 2, 4, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::min_operations(vec![8]));
    }
}
