pub struct Solution;

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .into_iter()
            .enumerate()
            .filter(|(_, s)| s.contains(x))
            .map(|(i, _)| i as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["leet", "code"].iter().map(|w| w.to_string()).collect();
        let mut ans = Solution::find_words_containing(words, 'e');
        ans.sort_unstable();
        assert_eq!(vec![0, 1], ans);
    }

    #[test]
    fn case2() {
        let words = ["abc", "bcd", "aaaa", "cbc"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let mut ans = Solution::find_words_containing(words, 'a');
        ans.sort_unstable();
        assert_eq!(vec![0, 2], ans);
    }

    #[test]
    fn case3() {
        let words = ["abc", "bcd", "aaaa", "cbc"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let ans = Solution::find_words_containing(words, 'z');
        assert!(ans.is_empty());
    }
}
