pub struct Solution;

impl Solution {
    pub fn largest_merge(word1: String, word2: String) -> String {
        let mut r = String::with_capacity(word1.len() + word2.len());
        let (mut i, mut j) = (0, 0);
        let (w1, w2) = (word1.as_bytes(), word2.as_bytes());
        while i < w1.len() && j < w2.len() {
            match w1[i..].cmp(&w2[j..]) {
                std::cmp::Ordering::Less => {
                    r.push(w2[j] as char);
                    j += 1;
                }
                _ => {
                    r.push(w1[i] as char);
                    i += 1;
                }
            }
        }
        if i < w1.len() {
            r.push_str(&word1[i..]);
        } else if j < w2.len() {
            r.push_str(&word2[j..]);
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "cbcabaaaaa",
            Solution::largest_merge("cabaa".to_string(), "bcaaa".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "abdcabcabcaba",
            Solution::largest_merge("abcabc".to_string(), "abdcaba".to_string())
        );
    }
}
