pub struct Solution;

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        words.iter().filter(|&w| s.starts_with(w)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["a", "b", "c", "ab", "bc", "abc"];
        let words = words.iter().map(|w| w.to_string()).collect();
        let s = "abc";
        assert_eq!(3, Solution::count_prefixes(words, s.into()));
    }
}
