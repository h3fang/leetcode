pub struct Solution;

impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        for i in 0..2 {
            if !((s1[i] == s2[i] && s1[i + 2] == s2[i + 2])
                || (s1[i] == s2[i + 2] && s1[i + 2] == s2[i]))
            {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_be_equal(
            "abcd".to_string(),
            "cdab".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::can_be_equal(
            "abcd".to_string(),
            "dacb".to_string()
        ));
    }
}
