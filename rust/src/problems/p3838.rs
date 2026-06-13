pub struct Solution;

impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        words
            .iter()
            .map(|w| {
                let w: i32 = w
                    .as_bytes()
                    .iter()
                    .map(|&b| weights[(b - b'a') as usize])
                    .sum();
                (25 - (w % 26) as u8 + b'a') as char
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["abcd", "def", "xyz"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let weights = vec![
            5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2,
        ];
        assert_eq!("rij", Solution::map_word_weights(words, weights));
    }

    #[test]
    fn case2() {
        let words = ["a", "b", "c"].iter().map(|w| w.to_string()).collect();
        let weights = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
        assert_eq!("yyy", Solution::map_word_weights(words, weights));
    }

    #[test]
    fn case3() {
        let words = ["abcd"].iter().map(|w| w.to_string()).collect();
        let weights = vec![
            7, 5, 3, 4, 3, 5, 4, 9, 4, 2, 2, 7, 10, 2, 5, 10, 6, 1, 2, 2, 4, 1, 3, 4, 4, 5,
        ];
        assert_eq!("g", Solution::map_word_weights(words, weights));
    }
}
