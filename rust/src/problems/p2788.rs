pub struct Solution;

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut result = vec![];
        words.into_iter().for_each(|w| {
            for p in w.split(separator) {
                if !p.is_empty() {
                    result.push(p.to_string());
                }
            }
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["one.two.three", "four.five", "six"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let expected = ["one", "two", "three", "four", "five", "six"]
            .iter()
            .map(|w| w.to_string())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::split_words_by_separator(words, '.'));
    }

    #[test]
    fn case2() {
        let words = ["$easy$", "$problem$"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let expected = ["easy", "problem"]
            .iter()
            .map(|w| w.to_string())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::split_words_by_separator(words, '$'));
    }

    #[test]
    fn case3() {
        let words = ["|||"].iter().map(|w| w.to_string()).collect();
        assert!(Solution::split_words_by_separator(words, '|').is_empty());
    }
}
