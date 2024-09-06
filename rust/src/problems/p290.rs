use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut mapped = [false; 26];
        let mut m2c: HashMap<&str, char> = HashMap::new();
        let words = s.split_ascii_whitespace().collect::<Vec<_>>();
        if words.len() != pattern.len() {
            return false;
        }
        for (c, word) in pattern.chars().zip(words) {
            if let Some(mapped) = m2c.get(word) {
                if *mapped != c {
                    return false;
                }
            } else {
                let i = (c as u8 - b'a') as usize;
                if mapped[i] {
                    return false;
                } else {
                    mapped[i] = true;
                    m2c.insert(word, c);
                }
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
        assert!(Solution::word_pattern(
            "abba".to_string(),
            "dog cat cat dog".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::word_pattern(
            "abba".to_string(),
            "dog cat cat fish".to_string()
        ));
    }
}
