pub struct Solution;

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let p = part.as_bytes();
        let n = p.len();
        let mut r = Vec::with_capacity(s.len());
        for b in s.bytes() {
            r.push(b);
            if r.len() >= n && r.ends_with(p) {
                r.resize(r.len() - n, b' ');
            }
        }
        unsafe { String::from_utf8_unchecked(r) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "dab",
            Solution::remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "ab",
            Solution::remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string())
        );
    }
}
