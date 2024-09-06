pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' | ']' | '}' => {
                    let p = stack.pop().map(|p| (p, c));
                    match p {
                        Some(('(', ')')) | Some(('[', ']')) | Some(('{', '}')) => {
                            continue;
                        }
                        _ => return false,
                    }
                }
                _ => return false,
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn case2() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_valid("(}".to_string()));
    }
}
