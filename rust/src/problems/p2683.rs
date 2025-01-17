pub struct Solution;

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        derived.into_iter().fold(0, |acc, x| acc ^ x) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::does_valid_array_exist(vec![1, 1, 0]));
    }

    #[test]
    fn case2() {
        assert!(Solution::does_valid_array_exist(vec![1, 1]));
    }

    #[test]
    fn case3() {
        assert!(!Solution::does_valid_array_exist(vec![1, 0]));
    }
}
