use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut words = word_list.iter().map(|s| s.as_str()).collect::<HashSet<_>>();
        if !words.contains(end_word.as_str()) {
            return 0;
        }
        let mut forward = HashSet::new();
        let mut backward = HashSet::new();
        forward.insert(begin_word.as_str());
        backward.insert(end_word.as_str());
        let mut dist = 2;
        while !forward.is_empty() && !backward.is_empty() {
            let m = forward.len();
            let n = backward.len();
            let q = &mut forward;
            let other = &mut backward;
            if n < m {
                std::mem::swap(q, other);
            }
            let mut next_set = HashSet::new();
            for w in q.iter() {
                let mut bytes = w.to_string().into_bytes();
                words.remove(w);
                for i in 0..bytes.len() {
                    let original = bytes[i];
                    for alphabet in b'a'..=b'z' {
                        if alphabet != original {
                            bytes[i] = alphabet;
                            let next = unsafe { std::str::from_utf8_unchecked(&bytes) };
                            if let Some(&n) = words.get(next) {
                                if other.contains(&n) {
                                    return dist;
                                }
                                next_set.insert(n);
                            }
                        }
                    }
                    bytes[i] = original;
                }
            }
            *q = next_set;
            dist += 1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = ["hot", "dot", "dog", "lot", "log", "cog"];
        let word_list = word_list.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        assert_eq!(5, Solution::ladder_length(begin_word, end_word, word_list));
    }

    #[test]
    fn case2() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = ["hot", "dot", "dog", "lot", "log"];
        let word_list = word_list.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        assert_eq!(0, Solution::ladder_length(begin_word, end_word, word_list));
    }

    #[test]
    fn case3() {
        let begin_word = "ymain".to_string();
        let end_word = "oecij".to_string();
        let word_list = [
            "ymann", "yycrj", "oecij", "ymcnj", "yzcrj", "yycij", "xecij", "yecij", "ymanj",
            "yzcnj", "ymain",
        ];
        let word_list = word_list.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        assert_eq!(10, Solution::ladder_length(begin_word, end_word, word_list));
    }
}
