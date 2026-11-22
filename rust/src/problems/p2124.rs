pub struct Solution;

impl Solution {
    pub fn check_string(s: String) -> bool {
        s.as_bytes().is_sorted()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_string("aaabbb".to_string()));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check_string("abab".to_string()));
    }

    #[test]
    fn case3() {
        assert!(Solution::check_string("bbb".to_string()));
    }
}
