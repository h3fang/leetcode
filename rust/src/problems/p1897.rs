pub struct Solution;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut freq = [0; 26];
        let n = words.len();
        for w in words {
            for &b in w.as_bytes() {
                freq[(b - b'a') as usize] += 1;
            }
        }
        freq.into_iter().all(|f| f % n == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["abc", "aabc", "bc"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert!(Solution::make_equal(words));
    }

    #[test]
    fn case2() {
        let words = ["ab", "a"].iter().map(|w| w.to_string()).collect();
        assert!(!Solution::make_equal(words));
    }
}
