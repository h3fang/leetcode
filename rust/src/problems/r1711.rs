pub struct Solution;

impl Solution {
    pub fn find_closest(words: Vec<String>, word1: String, word2: String) -> i32 {
        let mut w1 = -1;
        let mut w2 = -1;
        let mut result = i32::MAX;
        for (i, w) in words.iter().enumerate() {
            if w == &word1 {
                w1 = i as i32;
                if w2 >= 0 {
                    result = result.min((w1 - w2).abs());
                }
            } else if w == &word2 {
                w2 = i as i32;
                if w1 >= 0 {
                    result = result.min((w1 - w2).abs());
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = [
            "I",
            "am",
            "a",
            "student",
            "from",
            "a",
            "university",
            "in",
            "a",
            "city",
        ];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!(
            1,
            Solution::find_closest(words, "a".into(), "student".into())
        )
    }
}
