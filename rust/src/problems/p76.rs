use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.len() > s.len() {
            return "".to_string();
        }

        let mut sig = HashMap::new();
        for c in t.as_bytes() {
            *sig.entry(*c).or_insert(0) += 1;
        }

        let mut filtered = Vec::new();
        let mut bytes = Vec::new();
        for (i, c) in s.as_bytes().iter().enumerate() {
            if sig.contains_key(c) {
                filtered.push(i);
                bytes.push(*c);
            }
        }

        let mut left = 0;
        let mut min = usize::MAX;
        let mut result = None;

        for right in 0..bytes.len() {
            *sig.entry(bytes[right]).or_default() -= 1;

            while sig.values().all(|v| v <= &0) {
                let len = filtered[right] - filtered[left] + 1;
                if len < min {
                    min = len;
                    result = Some((left, right));
                }
                *sig.entry(bytes[left]).or_default() += 1;
                left += 1;
            }
        }

        if let Some((left, right)) = result {
            let left = filtered[left];
            let right = filtered[right];
            s[left..=right].to_string()
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        let expected = "BANC".to_string();
        assert_eq!(expected, Solution::min_window(s, t));
    }

    #[test]
    fn case2() {
        let s = "a".to_string();
        let t = "a".to_string();
        let expected = "a".to_string();
        assert_eq!(expected, Solution::min_window(s, t));
    }

    #[test]
    fn case3() {
        let s = "a".to_string();
        let t = "aa".to_string();
        let expected = "".to_string();
        assert_eq!(expected, Solution::min_window(s, t));
    }

    #[test]
    fn case4() {
        let s = "ab".to_string();
        let t = "b".to_string();
        let expected = "b".to_string();
        assert_eq!(expected, Solution::min_window(s, t));
    }

    #[test]
    fn case5() {
        let s = "a".to_string();
        let t = "b".to_string();
        let expected = "".to_string();
        assert_eq!(expected, Solution::min_window(s, t));
    }
}
