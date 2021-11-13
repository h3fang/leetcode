use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

const ALPHABETS: [u8; 26] = [
    b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
    b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
];

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let words = word_list.iter().map(|w| w.as_str()).collect::<HashSet<_>>();
        if !words.contains(end_word.as_str()) {
            return result;
        }
        let mut g: HashMap<&str, HashSet<&str>> = HashMap::new();
        let mut q = VecDeque::new();
        let mut dist = HashMap::with_capacity(words.len());
        dist.insert(begin_word.as_str(), 0);
        q.push_back((begin_word.as_str(), 0));
        let mut reachable = false;
        while let Some((w, c)) = q.pop_front() {
            if w == end_word {
                reachable = true;
                break;
            }
            let mut bytes = w.to_owned().into_bytes();
            for i in 0..bytes.len() {
                let original = bytes[i];
                for &alphabet in &ALPHABETS {
                    if original != alphabet {
                        bytes[i] = alphabet;
                        unsafe {
                            let neighbor = std::str::from_utf8_unchecked(&bytes);
                            if let Some(d) = dist.get(neighbor) {
                                if *d < c + 1 {
                                    continue;
                                }
                            }
                            if let Some(&n) = words.get(neighbor) {
                                dist.insert(n, c + 1);
                                q.push_back((n, c + 1));
                                g.entry(w).or_default().insert(n);
                            }
                        }
                    }
                }
                bytes[i] = original;
            }
        }

        if !reachable {
            return result;
        }

        fn dfs<'a>(
            g: &HashMap<&str, HashSet<&'a str>>,
            dist: &HashMap<&str, i32>,
            result: &mut Vec<Vec<String>>,
            end_word: &str,
            curr: &mut Vec<&'a str>,
        ) {
            let last = *curr.last().unwrap();
            if last == end_word {
                result.push(curr.iter().map(|s| s.to_string()).collect());
            } else if let Some(children) = g.get(last) {
                for next in children {
                    curr.push(*next);
                    dfs(g, dist, result, end_word, curr);
                    curr.pop();
                }
            }
        }

        let mut curr = vec![begin_word.as_str()];
        dfs(&g, &dist, &mut result, &end_word, &mut curr);
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
