pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut f: HashMap<&str, i32> = HashMap::new();
        for w in &words {
            *f.entry(w.as_str()).or_default() += 1;
        }
        let len = s.len();
        let m = words.len();
        let n = words[0].len();
        let mut result = Vec::with_capacity(len);
        for i in 0..n {
            if i + m * n > len {
                break;
            }
            let mut f1 = f.clone();
            for j in 0..m {
                let w = &s[i + j * n..i + (j + 1) * n];
                let e = f1.entry(w).or_default();
                *e -= 1;
                if *e == 0 {
                    f1.remove(w);
                }
            }
            if f1.is_empty() {
                result.push(i as i32);
            }
            for j in (i + n..len).step_by(n) {
                if j + m * n > len {
                    break;
                }
                // previous word
                let w = &s[j - n..j];
                let e = f1.entry(w).or_default();
                *e += 1;
                if *e == 0 {
                    f1.remove(w);
                }

                // last word
                let w = &s[j + (m - 1) * n..j + m * n];
                let e = f1.entry(w).or_default();
                *e -= 1;
                if *e == 0 {
                    f1.remove(w);
                }
                if f1.is_empty() {
                    result.push(j as i32);
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
        let s = "barfoothefoobarman".to_string();
        let words = ["foo", "bar"];
        let words = words.iter().map(|w| w.to_string()).collect();
        let mut result = Solution::find_substring(s, words);
        result.sort_unstable();
        assert_eq!(vec![0, 9], result);
    }

    #[test]
    fn case2() {
        let s = "wordgoodgoodgoodbestword".to_string();
        let words = ["word", "good", "best", "word"];
        let words = words.iter().map(|w| w.to_string()).collect();
        let mut result = Solution::find_substring(s, words);
        result.sort_unstable();
        assert_eq!(vec![0; 0], result);
    }
}
