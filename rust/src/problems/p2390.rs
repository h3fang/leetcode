pub struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let s = s.as_bytes();
        let mut q = Vec::with_capacity(s.len());
        for &c in s {
            if c == b'*' {
                q.pop();
            } else {
                q.push(c);
            }
        }
        unsafe { String::from_utf8_unchecked(q) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("lecoe", Solution::remove_stars("leet**cod*e".to_string()));
    }
}
