use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let banned = banned.iter().map(|w| w.as_str()).collect::<HashSet<_>>();
        let mut count = HashMap::new();
        let mut max = 0;
        let mut result = "".to_string();
        for w in paragraph
            .split(|c: char| !c.is_ascii_alphabetic())
            .filter(|w| !w.is_empty())
        {
            let lower = w.to_ascii_lowercase();
            if !banned.contains(lower.as_str()) {
                let e = count.entry(lower.to_string()).or_insert(0);
                *e += 1;
                if *e > max {
                    max = *e;
                    result = lower;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let paragraph = "Bob hit a ball, the hit BALL flew far after it was hit.".to_string();
        let banned = vec!["hit".to_string()];
        assert_eq!("ball", Solution::most_common_word(paragraph, banned));
    }
}
