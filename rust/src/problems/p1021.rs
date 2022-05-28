pub struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut diff = 0;
        let mut start = 0;
        let mut result = String::with_capacity(s.len());
        for (i, c) in s.char_indices() {
            match c {
                '(' => diff += 1,
                _ => diff -= 1,
            }
            if diff == 0 {
                result += &s[start + 1..i];
                start = i + 1;
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
        assert_eq!(
            "()()()",
            Solution::remove_outer_parentheses("(()())(())".into())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "()()()()(())",
            Solution::remove_outer_parentheses("(()())(())(()(()))".into())
        );
    }
}
