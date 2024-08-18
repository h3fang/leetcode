pub struct Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        s.bytes().filter(|&b| b == b'A').count() < 2
            && s.as_bytes()
                .windows(3)
                .all(|w| !(w.iter().all(|&b| b == b'L')))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_record("PPALLP".to_string()));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check_record("PPALLL".to_string()));
    }
}
