pub struct Solution;

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut words = word_list.iter().map(|w| w.as_str()).collect::<HashSet<_>>();
        if !words.contains(end_word.as_str()) {
            return result;
        }
        words.remove(begin_word.as_str());
        let mut g: HashMap<&str, HashSet<&str>> = HashMap::new();
        let mut dist = HashMap::with_capacity(words.len());
        let mut q = VecDeque::with_capacity(words.len());
        q.push_back(begin_word.as_str());
        dist.insert(begin_word.as_str(), 0);
        let mut d = 0;
        let mut reachable = false;
        while !q.is_empty() {
            d += 1;
            let n = q.len();
            for _ in 0..n {
                let w = q.pop_front().unwrap();
                let mut bytes = w.as_bytes().to_vec();
                for i in 0..bytes.len() {
                    let original = bytes[i];
                    for alphabet in b'a'..=b'z' {
                        if original == alphabet {
                            continue;
                        }
                        bytes[i] = alphabet;
                        unsafe {
                            let neighbor = std::str::from_utf8_unchecked(&bytes);
                            if let Some(&d1) = dist.get(neighbor) {
                                if d1 == d {
                                    g.get_mut(neighbor).unwrap().insert(w);
                                }
                            }
                            if let Some(n) = words.get(neighbor) {
                                if n == &end_word {
                                    reachable = true;
                                }
                                dist.insert(n, d);
                                q.push_back(n);
                                g.entry(n).or_default().insert(w);
                                words.remove(neighbor);
                            }
                        }
                    }
                    bytes[i] = original;
                }
            }
            if reachable {
                break;
            }
        }

        if !reachable {
            return result;
        }

        fn dfs<'a>(
            g: &HashMap<&'a str, HashSet<&'a str>>,
            result: &mut Vec<Vec<String>>,
            target: &str,
            curr: &mut Vec<&'a str>,
        ) {
            let last = *curr.last().unwrap();
            if last == target {
                result.push(curr.iter().rev().map(|s| s.to_string()).collect());
            } else if let Some(children) = g.get(last) {
                for &next in children {
                    curr.push(next);
                    dfs(g, result, target, curr);
                    curr.pop();
                }
            }
        }

        let mut curr = vec![end_word.as_str()];
        dfs(&g, &mut result, &begin_word, &mut curr);
        result
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
        let expected = [
            ["hit", "hot", "dot", "dog", "cog"],
            ["hit", "hot", "lot", "log", "cog"],
        ];
        let mut expected = expected
            .iter()
            .map(|p| p.iter().map(|w| w.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::find_ladders(begin_word, end_word, word_list);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = ["hot", "dot", "dog", "lot", "log"];
        let word_list = word_list.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        let expected: Vec<Vec<String>> = vec![];
        assert_eq!(
            expected,
            Solution::find_ladders(begin_word, end_word, word_list)
        );
    }

    #[test]
    fn case3() {
        let begin_word = "red".to_string();
        let end_word = "tax".to_string();
        let word_list = ["ted", "tex", "red", "tax", "tad", "den", "rex", "pee"];
        let word_list = word_list.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        let expected = [
            ["red", "ted", "tad", "tax"],
            ["red", "ted", "tex", "tax"],
            ["red", "rex", "tex", "tax"],
        ];
        let mut expected = expected
            .iter()
            .map(|p| p.iter().map(|w| w.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::find_ladders(begin_word, end_word, word_list);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
