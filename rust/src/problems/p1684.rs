pub struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut mask = 0;
        for &b in allowed.as_bytes() {
            mask |= 1 << (b - b'a');
        }
        words
            .iter()
            .filter(|w| w.as_bytes().iter().all(|&b| mask & (1 << (b - b'a')) > 0))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let allowed = "ab".to_string();
        let words = ["ad", "bd", "aaab", "baa", "badab"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(2, Solution::count_consistent_strings(allowed, words));
    }

    #[test]
    fn case2() {
        let allowed = "abc".to_string();
        let words = ["a", "b", "c", "ab", "ac", "bc", "abc"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(7, Solution::count_consistent_strings(allowed, words));
    }
}
