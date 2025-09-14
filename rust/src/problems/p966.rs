pub struct Solution;

use std::collections::{HashMap, HashSet};

fn to_wildcard(s: &str) -> String {
    let mut w = s.to_string();
    let bytes = unsafe { w.as_bytes_mut() };
    bytes.iter_mut().for_each(|b| {
        if b"aeiou".contains(b) {
            *b = b'*';
        }
    });
    w
}

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let words = wordlist.iter().map(|w| w.as_str()).collect::<HashSet<_>>();
        let mut lower = HashMap::with_capacity(wordlist.len());
        let mut vowels = HashMap::with_capacity(wordlist.len());
        for (i, w) in wordlist.iter().enumerate().rev() {
            let w = w.to_ascii_lowercase();

            vowels.insert(to_wildcard(&w), i);
            lower.insert(w, i);
        }
        queries
            .into_iter()
            .map(|q| {
                if words.contains(q.as_str()) {
                    return q;
                }
                let s = q.to_ascii_lowercase();
                if let Some(&i) = lower.get(&s) {
                    return wordlist[i].clone();
                }
                let s = to_wildcard(&s);
                if let Some(&i) = vowels.get(&s) {
                    wordlist[i].clone()
                } else {
                    String::new()
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
        let wordlist = ["KiTe", "kite", "hare", "Hare"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let queries = [
            "kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let expected: Vec<String> = [
            "kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "", "KiTe",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(expected, Solution::spellchecker(wordlist, queries));
    }

    #[test]
    fn case2() {
        let wordlist = ["yellow"].iter().map(|s| s.to_string()).collect();
        let queries = ["YellOw"].iter().map(|s| s.to_string()).collect();
        let expected: Vec<String> = ["yellow"].iter().map(|s| s.to_string()).collect();
        assert_eq!(expected, Solution::spellchecker(wordlist, queries));
    }
}
