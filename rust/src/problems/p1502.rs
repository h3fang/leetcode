pub struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        let d = arr[1] - arr[0];
        arr.windows(2).all(|w| w[1] - w[0] == d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_make_arithmetic_progression(vec![3, 5, 1]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::can_make_arithmetic_progression(vec![1, 2, 4]));
    }
}
