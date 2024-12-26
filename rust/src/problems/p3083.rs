pub struct Solution;

impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        s.as_bytes().windows(2).any(|w| {
            let w = [w[1], w[0]];
            let t = std::str::from_utf8(&w).unwrap();
            s.contains(t)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_substring_present("leetcode".to_string()));
    }

    #[test]
    fn case2() {
        assert!(Solution::is_substring_present("abcba".to_string()));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_substring_present("abcd".to_string()));
    }
}
