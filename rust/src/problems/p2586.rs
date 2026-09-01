pub struct Solution;

const VOWELS: &[u8] = b"aeiou";

impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        words[left as usize..=right as usize]
            .iter()
            .filter(|w| {
                let w = w.as_bytes();
                VOWELS.contains(&w[0]) && VOWELS.contains(&w[w.len() - 1])
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["are", "amy", "u"].iter().map(|w| w.to_string()).collect();
        assert_eq!(2, Solution::vowel_strings(words, 0, 2));
    }

    #[test]
    fn case2() {
        let words = ["hey", "aeo", "mu", "ooo", "artro"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(3, Solution::vowel_strings(words, 1, 4));
    }
}
