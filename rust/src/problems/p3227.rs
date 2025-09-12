pub struct Solution;

impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        s.as_bytes().iter().any(|c| b"aeiou".contains(c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::does_alice_win("leetcoder".to_string()));
    }

    #[test]
    fn case2() {
        assert!(!Solution::does_alice_win("bbcd".to_string()));
    }
}
