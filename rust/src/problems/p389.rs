pub struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        s.chars().chain(t.chars()).fold(0, |sum, c| sum ^ c as u8) as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            'e',
            Solution::find_the_difference("abcd".to_string(), "abcde".to_string())
        );
    }
}
