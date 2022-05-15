pub struct Solution;

impl Solution {
    pub fn remove_anagrams(mut words: Vec<String>) -> Vec<String> {
        let mut counts = words
            .iter()
            .map(|w| {
                let mut count = [0; 26];
                for b in w.as_bytes() {
                    let i = (b - b'a') as usize;
                    count[i] += 1;
                }
                count
            })
            .collect::<Vec<_>>();

        let mut i = 1;
        while i < words.len() {
            if counts[i] == counts[i - 1] {
                words.remove(i);
                counts.remove(i);
            } else {
                i += 1;
            }
        }
        words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_words(words: &[&str]) -> Vec<String> {
        words.iter().map(|w| w.to_string()).collect()
    }

    #[test]
    fn case1() {
        let words = parse_words(&["abba", "baba", "bbaa", "cd", "cd"]);
        let expected = parse_words(&["abba", "cd"]);
        assert_eq!(expected, Solution::remove_anagrams(words));
    }
}
