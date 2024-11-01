pub struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut t = Vec::with_capacity(s.len());
        for b in s.bytes() {
            if t.len() >= 2 && t[t.len() - 2] == t[t.len() - 1] && t[t.len() - 1] == b {
                continue;
            }
            t.push(b);
        }
        unsafe { String::from_utf8_unchecked(t) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "leetcode",
            Solution::make_fancy_string("leeetcode".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!("aabaa", Solution::make_fancy_string("aaabaaaa".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!("aab", Solution::make_fancy_string("aab".to_string()));
    }
}
