pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut a = 0;
        let mut b = 0;
        let s = s.as_bytes();
        let t = t.as_bytes();
        while a < s.len() && b < t.len() {
            if s[a] == t[b] {
                a += 1;
            }
            b += 1;
        }
        a == s.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            true,
            Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            false,
            Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string())
        );
    }
}
