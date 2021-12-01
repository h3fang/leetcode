pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let s = s.to_ascii_lowercase();
        let s = s.as_bytes();
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            while left < right && !s[left].is_ascii_alphanumeric() {
                left += 1;
            }
            while left < right && !s[right].is_ascii_alphanumeric() {
                right -= 1;
            }
            if left > right || s[left] != s[right] {
                return false;
            }
            if left == right {
                break;
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
        let s = "A man, a plan, a canal: Panama".to_string();
        assert_eq!(true, Solution::is_palindrome(s));
    }

    #[test]
    fn case2() {
        let s = "A*".to_string();
        assert_eq!(true, Solution::is_palindrome(s));
    }

    #[test]
    fn case3() {
        let s = "OP".to_string();
        assert_eq!(false, Solution::is_palindrome(s));
    }
}
