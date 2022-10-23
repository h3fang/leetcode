pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        let n = w1.len().min(w2.len());
        let mut s = w1[..n]
            .iter()
            .zip(&w2[..n])
            .flat_map(|(&a, &b)| [a as char, b as char])
            .collect::<String>();
        if w1.len() > n {
            s.push_str(&word1[n..]);
        } else if w2.len() > n {
            s.push_str(&word2[n..]);
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "apbqcr",
            Solution::merge_alternately("abc".to_owned(), "pqr".to_owned())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "apbqrs",
            Solution::merge_alternately("ab".to_owned(), "pqrs".to_owned())
        );
    }
}
