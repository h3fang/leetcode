pub struct Solution;

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        s[n as usize..].to_string() + &s[..n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "abcdefg".to_string();
        let n = 2;
        let expected = "cdefgab".to_string();
        assert_eq!(expected, Solution::reverse_left_words(s, n));
    }

    #[test]
    fn case2() {
        let s = "lrloseumgh".to_string();
        let n = 6;
        let expected = "umghlrlose".to_string();
        assert_eq!(expected, Solution::reverse_left_words(s, n));
    }
}
