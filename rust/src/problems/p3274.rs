pub struct Solution;

impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        let f = |s: &[u8]| s[0] - b'a' + s[1] - b'0';
        f(coordinate1.as_bytes()) % 2 == f(coordinate2.as_bytes()) % 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_two_chessboards(
            "a1".to_string(),
            "c3".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check_two_chessboards(
            "a1".to_string(),
            "h3".to_string()
        ));
    }
}
