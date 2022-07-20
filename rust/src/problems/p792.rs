pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut buckets = vec![VecDeque::with_capacity(words.len()); 26];
        for w in &words {
            let w = w.as_bytes();
            let i = (w[0] - b'a') as usize;
            buckets[i].push_back(w);
        }

        let mut result = 0;
        for b in s.as_bytes() {
            let i = (b - b'a') as usize;
            let n = buckets[i].len();
            for _ in 0..n {
                let w = buckets[i].pop_front().unwrap();
                if w.len() == 1 {
                    result += 1;
                } else {
                    let k = (w[1] - b'a') as usize;
                    buckets[k].push_back(&w[1..]);
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
        let s = "abcde".to_string();
        let words = ["a", "bb", "acd", "ace"];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!(3, Solution::num_matching_subseq(s, words));
    }

    #[test]
    fn case2() {
        let s = "dsahjpjauf".to_string();
        let words = ["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!(2, Solution::num_matching_subseq(s, words));
    }
}
