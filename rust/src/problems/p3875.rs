pub struct Solution;

impl Solution {
    pub fn uniform_array(_nums1: Vec<i32>) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::uniform_array(vec![2, 3]));
    }

    #[test]
    fn case2() {
        assert!(Solution::uniform_array(vec![4, 6]));
    }
}
