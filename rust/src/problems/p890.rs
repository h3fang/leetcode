use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        fn match_pattern(w: &str, p: &str) -> bool {
            let mut m: HashMap<char, char> = HashMap::new();
            for (a, b) in w.chars().zip(p.chars()) {
                if let Some(&e) = m.get(&a) {
                    if e != b {
                        return false;
                    }
                } else {
                    m.insert(a, b);
                }
            }
            true
        }
        words
            .into_iter()
            .filter_map(|w| {
                if match_pattern(&w, &pattern) && match_pattern(&pattern, &w) {
                    Some(w)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["abc", "deq", "mee", "aqq", "dkd", "ccc"];
        let words = words.iter().map(|w| w.to_string()).collect();
        let pattern = "abb".to_string();
        let expected = ["mee", "aqq"];
        let mut expected = expected.iter().map(|e| e.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::find_and_replace_pattern(words, pattern);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
