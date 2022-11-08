pub struct Solution;

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut r = String::with_capacity(s.len());
        for &b in s.as_bytes() {
            if let Some(&p) = r.as_bytes().last() {
                if p.to_ascii_lowercase() == b.to_ascii_lowercase() && p != b {
                    r.pop();
                } else {
                    r.push(b as char);
                }
            } else {
                r.push(b as char);
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("leetcode", Solution::make_good("leEeetcode".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("", Solution::make_good("abBAcC".to_string()));
    }
}
