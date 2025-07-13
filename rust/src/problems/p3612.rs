pub struct Solution;

impl Solution {
    pub fn process_str(s: String) -> String {
        let mut r = Vec::with_capacity(s.len());
        for &b in s.as_bytes() {
            match b {
                b'*' => _ = r.pop(),
                b'#' => {
                    for i in 0..r.len() {
                        r.push(r[i]);
                    }
                }
                b'%' => r.reverse(),
                _ => r.push(b),
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
        assert_eq!("ba", Solution::process_str("a#b%*".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("", Solution::process_str("z*#".to_string()));
    }
}
