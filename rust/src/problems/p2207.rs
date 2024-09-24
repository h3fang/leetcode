pub struct Solution;

impl Solution {
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let (a, b) = (pattern.as_bytes()[0], pattern.as_bytes()[1]);
        let (mut x, mut y, mut r) = (0, 0, 0);
        for c in text.bytes() {
            if c == b {
                y += 1;
                r += x;
            }
            if c == a {
                x += 1;
            }
        }
        (r + x).max(r + y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            4,
            Solution::maximum_subsequence_count("abdcdbc".to_string(), "ac".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            6,
            Solution::maximum_subsequence_count("aabb".to_string(), "ab".to_string())
        );
    }
}
