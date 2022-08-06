pub struct Solution;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        words
            .iter()
            .enumerate()
            .filter_map(|(i, a)| {
                if words
                    .iter()
                    .enumerate()
                    .any(|(j, b)| i != j && b.contains(a))
                {
                    Some(a.to_string())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["mass", "as", "hero", "superhero"];
        let words = words.iter().map(|w| w.to_string()).collect();
        let expected = ["as", "hero"];
        let mut expected = expected.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::string_matching(words);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
