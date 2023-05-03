pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s = s.as_bytes();
        let mut t = Vec::with_capacity(s.len());
        for &c in s {
            t.push(c);
            if t.len() >= 3 {
                let n = t.len();
                if t[n - 1] == b'c' && t[n - 2] == b'b' && t[n - 3] == b'a' {
                    t.drain(n - 3..);
                }
            }
        }
        t.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_valid("aabcbc".to_string()));
    }

    #[test]
    fn case2() {
        assert!(Solution::is_valid("abcabcababcc".to_string()));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_valid("abccba".to_string()));
    }
}
