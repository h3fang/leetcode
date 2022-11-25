pub struct Solution;

impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        fn check(s: &[u8], w: &[u8]) -> bool {
            let mut i = 0;
            let mut j = 0;
            while i < s.len() && j < w.len() {
                if s[i] != w[j] {
                    return false;
                }
                let c = s[i];
                let mut c_a = 0;
                while i < s.len() && s[i] == c {
                    i += 1;
                    c_a += 1;
                }
                let mut c_b = 0;
                while j < w.len() && w[j] == c {
                    j += 1;
                    c_b += 1;
                }
                if c_a < c_b || (c_a > c_b && c_a < 3) {
                    return false;
                }
            }
            i == s.len() && j == w.len()
        }

        words
            .iter()
            .filter(|w| check(s.as_bytes(), w.as_bytes()))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "heeellooo".to_string();
        let words = ["hello", "hi", "helo"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(1, Solution::expressive_words(s, words));
    }
}
