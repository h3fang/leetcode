pub struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        nums.iter().map(|n| n.signum()).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::array_sign(vec![1, 5, 0, 2, -3]));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::array_sign(vec![-1, 1, -1, 1, -1]));
    }
}
