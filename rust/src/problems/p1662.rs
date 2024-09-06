pub struct Solution;

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.iter().map(|w| w.len()).sum::<usize>() == word2.iter().map(|w| w.len()).sum::<usize>()
            && word1
                .iter()
                .flat_map(|w| w.as_bytes())
                .zip(word2.iter().flat_map(|w| w.as_bytes()))
                .all(|(a, b)| a == b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let word1 = ["ab", "c"].iter().map(|w| w.to_string()).collect();
        let word2 = ["a", "bc"].iter().map(|w| w.to_string()).collect();
        assert!(Solution::array_strings_are_equal(word1, word2));
    }

    #[test]
    fn case2() {
        let word1 = ["a", "cb"].iter().map(|w| w.to_string()).collect();
        let word2 = ["ab", "c"].iter().map(|w| w.to_string()).collect();
        assert!(!Solution::array_strings_are_equal(word1, word2));
    }

    #[test]
    fn case3() {
        let word1 = ["abc", "d", "defg"].iter().map(|w| w.to_string()).collect();
        let word2 = ["abcddefg"].iter().map(|w| w.to_string()).collect();
        assert!(Solution::array_strings_are_equal(word1, word2));
    }
}
