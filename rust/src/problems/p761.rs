pub struct Solution;

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        fn dfs(s: &[u8]) -> Vec<u8> {
            if s.len() <= 2 {
                return s.to_vec();
            }
            let mut count = 0;
            let mut parts = vec![];
            let mut start = 0;
            for (i, &b) in s.iter().enumerate() {
                if b == b'1' {
                    count += 1;
                } else {
                    count -= 1;
                    if count == 0 {
                        let mut s = dfs(&s[start + 1..i]);
                        s.insert(0, b'1');
                        s.push(b'0');
                        parts.push(s);
                        start = i + 1;
                    }
                }
            }
            parts.sort_unstable_by(|a, b| b.cmp(a));
            parts.into_iter().flatten().collect()
        }
        let r = dfs(s.as_bytes());
        unsafe { String::from_utf8_unchecked(r) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "11100100",
            Solution::make_largest_special("11011000".into())
        );
    }

    #[test]
    fn case2() {
        assert_eq!("10", Solution::make_largest_special("10".into()));
    }
}
