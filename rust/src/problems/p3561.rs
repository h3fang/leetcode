pub struct Solution;

impl Solution {
    pub fn resulting_string(s: String) -> String {
        let mut r = Vec::with_capacity(s.len());
        for b in s.bytes() {
            if r.last().is_some_and(|&a| {
                let diff = b.abs_diff(a);
                diff == 1 || diff == 25
            }) {
                r.pop();
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
        assert_eq!("c", Solution::resulting_string("abc".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("", Solution::resulting_string("adcb".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!("db", Solution::resulting_string("zadb".to_string()));
    }
}
