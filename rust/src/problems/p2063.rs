pub struct Solution;

impl Solution {
    pub fn count_vowels(word: String) -> i64 {
        fn is_vowel(b: u8) -> bool {
            b == b'a' || b == b'e' || b == b'i' || b == b'o' || b == b'u'
        }
        let n = word.len();
        let w = word.as_bytes();
        let mut result = 0;

        for (i, &b) in w.iter().enumerate() {
            if is_vowel(b) {
                result += (i + 1) * (n - i);
            }
        }

        result as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let word = "aba".to_string();
        assert_eq!(6, Solution::count_vowels(word));
    }

    #[test]
    fn case2() {
        let word = "abc".to_string();
        assert_eq!(3, Solution::count_vowels(word));
    }

    #[test]
    fn case3() {
        let word = "noosabasboosa".to_string();
        assert_eq!(237, Solution::count_vowels(word));
    }
}
