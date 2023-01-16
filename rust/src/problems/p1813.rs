pub struct Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let s1 = sentence1.split_ascii_whitespace().collect::<Vec<_>>();
        let s2 = sentence2.split_ascii_whitespace().collect::<Vec<_>>();
        let mut i = 0;
        let mut j = 0;
        while i < s1.len() && i < s2.len() && s1[i] == s2[i] {
            i += 1;
        }
        while j < s1.len() - i && j < s2.len() - i && s1[s1.len() - j - 1] == s2[s2.len() - j - 1] {
            j += 1;
        }
        i + j == s1.len().min(s2.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::are_sentences_similar(
            "My name is Haley".to_string(),
            "My Haley".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::are_sentences_similar(
            "of".to_string(),
            "A lot of words".to_string()
        ));
    }

    #[test]
    fn case3() {
        assert!(Solution::are_sentences_similar(
            "Eating right now".to_string(),
            "Eating".to_string()
        ));
    }
}
