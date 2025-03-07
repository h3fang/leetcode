pub struct Solution;

impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        s.len() == words.len()
            && s.as_bytes()
                .iter()
                .zip(words)
                .all(|(&c, w)| c == w.as_bytes()[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["alice", "bob", "charlie"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert!(Solution::is_acronym(words, "abc".to_string()));
    }

    #[test]
    fn case2() {
        let words = ["an", "apple"].iter().map(|w| w.to_string()).collect();
        assert!(!Solution::is_acronym(words, "a".to_string()));
    }

    #[test]
    fn case3() {
        let words = ["never", "gonna", "give", "up", "on", "you"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert!(Solution::is_acronym(words, "ngguoy".to_string()));
    }
}
