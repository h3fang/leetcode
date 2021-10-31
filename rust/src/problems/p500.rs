pub struct Solution;

const TABLE: [u8; 26] = [
    2, 3, 3, 2, 1, 2, 2, 2, 1, 2, 2, 2, 3, 3, 1, 1, 1, 1, 2, 1, 1, 3, 1, 3, 1, 3,
];

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        words
            .into_iter()
            .filter(|w| {
                let w = w.to_ascii_lowercase();
                let w = w.as_bytes();
                let idx = TABLE[(w[0] - b'a') as usize];
                w.iter().all(|b| TABLE[(*b - b'a') as usize] == idx)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["Hello", "Alaska", "Dad", "Peace"];
        let words = words.iter().map(|e| e.to_string()).collect::<Vec<_>>();
        let expected = ["Alaska", "Dad"];
        let mut expected = expected.iter().map(|e| e.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::find_words(words);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let words = ["omk"];
        let words = words.iter().map(|e| e.to_string()).collect::<Vec<_>>();
        let expected: [&str; 0] = [];
        let mut expected = expected.iter().map(|e| e.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::find_words(words);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
