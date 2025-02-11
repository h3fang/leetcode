pub struct Solution;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        fn freq(s: &str) -> [u32; 26] {
            let mut f = [0; 26];
            s.as_bytes()
                .iter()
                .for_each(|c| f[(c - b'a') as usize] += 1);
            f
        }
        let f0 = freq(&chars);
        words
            .iter()
            .map(|w| {
                let f = freq(w);
                if f.iter().zip(&f0).all(|(a, b)| a <= b) {
                    w.len()
                } else {
                    0
                }
            })
            .sum::<usize>() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["cat", "bt", "hat", "tree"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let chars = "atach".to_string();
        assert_eq!(6, Solution::count_characters(words, chars));
    }

    #[test]
    fn case2() {
        let words = ["hello", "world", "leetcode"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let chars = "welldonehoneyr".to_string();
        assert_eq!(10, Solution::count_characters(words, chars));
    }
}
