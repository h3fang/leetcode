pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, n| acc ^ n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::single_number(vec![2, 2, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::single_number(vec![4, 1, 2, 1, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(69, Solution::single_number(vec![69]));
    }
}
