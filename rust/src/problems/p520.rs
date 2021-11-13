pub struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        word.chars().all(|c| c.is_ascii_uppercase())
            || word.chars().skip(1).all(|c| c.is_ascii_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::detect_capital_use("USA".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::detect_capital_use("FlaG".to_string()));
    }
}
