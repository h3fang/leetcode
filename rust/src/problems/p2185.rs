pub struct Solution;

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.into_iter().filter(|w| w.starts_with(&pref)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["pay", "attention", "practice", "attend"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(2, Solution::prefix_count(words, "at".to_string()));
    }

    #[test]
    fn case2() {
        let words = ["leetcode", "win", "loops", "success"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(0, Solution::prefix_count(words, "code".to_string()));
    }
}
