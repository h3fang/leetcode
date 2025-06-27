pub struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        fn helper(s: &str) -> i32 {
            let mut balance = 0;
            for (i, c) in s.char_indices() {
                match c {
                    '(' => balance += 1,
                    _ => {
                        balance -= 1;
                        if balance == 0 {
                            let left = if i == 1 { 1 } else { 2 * helper(&s[1..i]) };
                            if i + 1 == s.len() {
                                return left;
                            } else {
                                return left + helper(&s[i + 1..]);
                            }
                        }
                    }
                }
            }
            0
        }
        helper(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::score_of_parentheses("()".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::score_of_parentheses("(())".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::score_of_parentheses("()()".to_string()));
    }
}
