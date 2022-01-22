pub struct Solution;

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        let s = s.as_bytes();
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            if s[left] != s[right] {
                return 2;
            }
            left += 1;
            right -= 1;
        }
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::remove_palindrome_sub("ababa".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::remove_palindrome_sub("abb".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::remove_palindrome_sub("baabb".to_string()));
    }
}
