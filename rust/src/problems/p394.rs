pub struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut parts = vec![];
        let mut num = 0;
        let mut curr = String::new();
        for &b in s.as_bytes() {
            if b.is_ascii_digit() {
                num = num * 10 + (b - b'0') as usize;
            } else if b == b'[' {
                parts.push((curr, num));
                curr = String::new();
                num = 0;
            } else if b == b']' {
                let (prev, n) = parts.pop().unwrap();
                curr = prev + &curr.repeat(n);
            } else {
                curr.push(b as char);
            }
        }
        curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "aaabcbc".to_string(),
            Solution::decode_string("3[a]2[bc]".into())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "abcabccdcdcdef".to_string(),
            Solution::decode_string("2[abc]3[cd]ef".into())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "accaccacc".to_string(),
            Solution::decode_string("3[a2[c]]".into())
        );
    }
}
