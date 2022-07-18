pub struct Solution;

impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, n| acc | n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(7, Solution::maximum_xor(vec![3, 2, 4, 6]));
    }

    #[test]
    fn case2() {
        assert_eq!(11, Solution::maximum_xor(vec![1, 2, 3, 9, 2]));
    }
}
