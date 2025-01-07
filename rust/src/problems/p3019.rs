pub struct Solution;

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        s.as_bytes()
            .windows(2)
            .map(|w| i32::from(w[0].to_ascii_lowercase() != w[1].to_ascii_lowercase()))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::count_key_changes("aAbBcC".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_key_changes("AaAaAaaA".to_string()));
    }
}
