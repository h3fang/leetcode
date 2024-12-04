pub struct Solution;

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        if str1.len() < str2.len() {
            return false;
        }
        let (s1, s2) = (str1.as_bytes(), str2.as_bytes());
        let mut i = 0;
        for &b in s2 {
            while i < s1.len() && (s1[i] != b && b'a' + (s1[i] - b'a' + 1) % 26 != b) {
                i += 1;
            }
            if i >= s1.len() {
                return false;
            }
            i += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_make_subsequence(
            "abc".to_string(),
            "ad".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(Solution::can_make_subsequence(
            "zc".to_string(),
            "ad".to_string()
        ));
    }

    #[test]
    fn case3() {
        assert!(!Solution::can_make_subsequence(
            "ab".to_string(),
            "d".to_string()
        ));
    }
}
