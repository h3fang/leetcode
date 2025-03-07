pub struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut t = Vec::with_capacity(s.len());
        for &b in s.as_bytes() {
            if (b == b'B' && !t.is_empty() && *t.last().unwrap() == b'A')
                || (b == b'D' && !t.is_empty() && *t.last().unwrap() == b'C')
            {
                t.pop();
            } else {
                t.push(b);
            }
        }
        t.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::min_length("ABFCACDB".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::min_length("ACBBD".to_string()));
    }
}
