pub struct Solution;

impl Solution {
    pub fn final_string(s: String) -> String {
        let s = s.as_bytes();
        let mut r = Vec::with_capacity(s.len());
        for &b in s {
            if b == b'i' {
                r.reverse();
            } else {
                r.push(b);
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
        assert_eq!("rtsng", Solution::final_string("string".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("ponter", Solution::final_string("poiinter".to_string()));
    }
}
