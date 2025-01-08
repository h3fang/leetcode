pub struct Solution;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut result = 0;
        for (i, a) in words.iter().enumerate() {
            for b in &words[i + 1..] {
                if b.starts_with(a) && b.ends_with(a) {
                    result += 1;
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
        let words = ["a", "aba", "ababa", "aa"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(4, Solution::count_prefix_suffix_pairs(words));
    }

    #[test]
    fn case2() {
        let words = ["pa", "papa", "ma", "mama"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(2, Solution::count_prefix_suffix_pairs(words));
    }

    #[test]
    fn case3() {
        let words = ["abab", "ab"].iter().map(|w| w.to_string()).collect();
        assert_eq!(0, Solution::count_prefix_suffix_pairs(words));
    }
}
