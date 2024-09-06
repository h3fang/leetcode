pub struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        fn is_valid(s: &[u8]) -> bool {
            if s.is_empty() {
                return true;
            }
            let mut left = 0;
            let mut right = s.len() - 1;
            while left < right {
                if s[left] != s[right] {
                    return false;
                }
                left += 1;
                right -= 1;
            }
            true
        }

        let s = s.as_bytes();
        if s.is_empty() {
            return true;
        }
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            if s[left] != s[right] {
                return is_valid(&s[left + 1..=right]) || is_valid(&s[left..=right - 1]);
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::valid_palindrome("aba".into()));
    }

    #[test]
    fn case2() {
        assert!(Solution::valid_palindrome("abca".into()));
    }

    #[test]
    fn case3() {
        assert!(!Solution::valid_palindrome("abc".into()));
    }
}
