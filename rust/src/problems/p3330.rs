pub struct Solution;

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let w = word.as_bytes();
        1 + w.windows(2).filter(|w| w[0] == w[1]).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::possible_string_count("abbcccc".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::possible_string_count("abcd".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::possible_string_count("aaaa".to_string()));
    }
}
