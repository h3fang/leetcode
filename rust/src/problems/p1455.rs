pub struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        for (i, w) in sentence.split_ascii_whitespace().enumerate() {
            if w.starts_with(&search_word) {
                return i as i32 + 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            4,
            Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string())
        );
    }
}
