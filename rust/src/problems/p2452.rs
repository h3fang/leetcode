pub struct Solution;

impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        queries
            .into_iter()
            .filter(|q| {
                dictionary
                    .iter()
                    .any(|d| d.bytes().zip(q.bytes()).filter(|(a, b)| a != b).count() <= 2)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = ["word", "note", "ants", "wood"]
            .iter()
            .map(|w| w.to_string())
            .collect::<Vec<_>>();
        let dictionary = ["wood", "joke", "moat"]
            .iter()
            .map(|w| w.to_string())
            .collect::<Vec<_>>();
        let expected = ["word", "note", "wood"]
            .iter()
            .map(|w| w.to_string())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::two_edit_words(queries, dictionary));
    }

    #[test]
    fn case2() {
        let queries = ["yes"].iter().map(|w| w.to_string()).collect::<Vec<_>>();
        let dictionary = ["not"].iter().map(|w| w.to_string()).collect::<Vec<_>>();
        assert!(Solution::two_edit_words(queries, dictionary).is_empty());
    }
}
