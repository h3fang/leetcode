pub struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        nums.windows(2).all(|w| w[0] % 2 != w[1] % 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_array_special(vec![1]));
    }

    #[test]
    fn case2() {
        assert!(Solution::is_array_special(vec![2, 1, 4]));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_array_special(vec![4, 3, 1, 6]));
    }
}
