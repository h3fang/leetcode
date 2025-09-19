pub struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        for i in 1..=n / 2 {
            if !n.is_multiple_of(i) {
                continue;
            }
            if s == s[..i].repeat(n / i) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::repeated_substring_pattern("abab".to_string()));
    }

    #[test]
    fn case2() {
        assert!(!Solution::repeated_substring_pattern("aba".to_string()));
    }

    #[test]
    fn case3() {
        assert!(Solution::repeated_substring_pattern(
            "abcabcabcabc".to_string()
        ));
    }
}
