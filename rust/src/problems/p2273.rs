pub struct Solution;

impl Solution {
    pub fn remove_anagrams(mut words: Vec<String>) -> Vec<String> {
        let counts = words
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
        for (j, p) in counts.windows(2).enumerate() {
            if p[0] != p[1] {
                words.swap(i, j + 1);
                i += 1;
            }
        }
        words.resize(i, String::new());
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
