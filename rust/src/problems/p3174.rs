pub struct Solution;

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut t = Vec::with_capacity(s.len());
        for b in s.bytes() {
            if b.is_ascii_digit() {
                t.pop();
            } else {
                t.push(b);
            }
        }
        unsafe { String::from_utf8_unchecked(t) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("abc", Solution::clear_digits("abc".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("", Solution::clear_digits("cb34".to_string()));
    }
}
