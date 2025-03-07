pub struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let (mut left, mut star) = (vec![], vec![]);
        for (i, &b) in s.as_bytes().iter().enumerate() {
            match b {
                b'(' => left.push(i),
                b')' => {
                    if !left.is_empty() {
                        left.pop();
                    } else if !star.is_empty() {
                        star.pop();
                    } else {
                        return false;
                    }
                }
                b'*' => star.push(i),
                _ => {}
            }
        }
        while let Some(i) = left.pop() {
            if let Some(j) = star.pop() {
                if j < i {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_valid_string("()".to_string()));
    }

    #[test]
    fn case2() {
        assert!(Solution::check_valid_string("(*)".to_string()));
    }

    #[test]
    fn case3() {
        assert!(Solution::check_valid_string("(*))".to_string()));
    }
}
