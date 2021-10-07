pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let tokens = s
            .split(' ')
            .map(|t| t.chars().rev().collect::<String>())
            .collect::<Vec<_>>();
        tokens.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "Let's take LeetCode contest".to_string();
        let expected = "s'teL ekat edoCteeL tsetnoc".to_string();
        assert_eq!(expected, Solution::reverse_words(s));
    }

    #[test]
    fn case2() {
        let s = "God Ding".to_string();
        let expected = "doG gniD".to_string();
        assert_eq!(expected, Solution::reverse_words(s));
    }
}
