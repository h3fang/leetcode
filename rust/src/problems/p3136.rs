pub struct Solution;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        let w = word.as_bytes();
        if w.len() < 3 {
            return false;
        }
        let (mut vowel, mut consonant) = (false, false);
        for &b in w {
            if b.is_ascii_alphabetic() {
                let b = b.to_ascii_lowercase();
                if b"aeiou".contains(&b) {
                    vowel = true;
                } else {
                    consonant = true;
                }
            } else if !b.is_ascii_digit() {
                return false;
            }
        }
        vowel && consonant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_valid("234Adas".to_string()));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_valid("b3".to_string()));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_valid("a3$e".to_string()));
    }
}
