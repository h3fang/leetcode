use std::collections::HashSet;

pub struct Solution;

const MORSE_CODE: [&str; 26] = [
    ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
    "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
];

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let mut s = HashSet::new();
        for w in words {
            let mut encoded = String::new();
            for c in w.chars() {
                let i = (c as u8 - b'a') as usize;
                encoded.push_str(MORSE_CODE[i]);
            }
            s.insert(encoded);
        }
        s.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["gin", "zen", "gig", "msg"];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!(2, Solution::unique_morse_representations(words));
    }

    #[test]
    fn case2() {
        let words = ["a"];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!(1, Solution::unique_morse_representations(words));
    }
}
