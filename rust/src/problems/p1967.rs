pub struct Solution;

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns.into_iter().filter(|p| word.contains(p)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let patterns = ["a", "abc", "bc", "d"]
            .iter()
            .map(|p| p.to_string())
            .collect();
        assert_eq!(3, Solution::num_of_strings(patterns, "abc".to_string()));
    }

    #[test]
    fn case2() {
        let patterns = ["a", "b", "c"].iter().map(|p| p.to_string()).collect();
        assert_eq!(
            2,
            Solution::num_of_strings(patterns, "aaaaabbbbb".to_string())
        );
    }

    #[test]
    fn case3() {
        let patterns = ["a", "a", "a"].iter().map(|p| p.to_string()).collect();
        assert_eq!(3, Solution::num_of_strings(patterns, "ab".to_string()));
    }
}
