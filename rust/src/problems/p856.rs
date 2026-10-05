pub struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut ans, mut depth) = (0, 0);

        for (i, &b) in s.iter().enumerate() {
            if b == b'(' {
                depth += 1;
            } else {
                depth -= 1;
                if s[i - 1] == b'(' {
                    ans += 1 << depth;
                }
            }
        }

        ans
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
