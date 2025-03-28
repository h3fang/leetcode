pub struct Solution;

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(!Solution::can_win_nim(4));
    }

    #[test]
    fn case2() {
        assert!(Solution::can_win_nim(1));
    }

    #[test]
    fn case3() {
        assert!(Solution::can_win_nim(2));
    }
}
