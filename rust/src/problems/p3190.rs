pub struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter().map(|x| i32::from(x % 3 != 0)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::minimum_operations(vec![1, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::minimum_operations(vec![3, 6, 9]));
    }
}
