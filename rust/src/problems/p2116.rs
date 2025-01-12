pub struct Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }
        let (mut free, mut left, mut right) = (0, 0, 0);
        for (&a, &b) in s.as_bytes().iter().zip(locked.as_bytes()) {
            if b == b'0' {
                free += 1;
            } else {
                match a {
                    b'(' => left += 1,
                    b')' => {
                        if left > 0 {
                            left -= 1;
                        } else if free > 0 {
                            free -= 1;
                        } else {
                            return false;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        for (&a, &b) in s.as_bytes().iter().zip(locked.as_bytes()).rev() {
            if b == b'0' {
                right += 1;
            } else {
                match a {
                    b'(' => {
                        if right == 0 {
                            return false;
                        }
                        left -= 1;
                        right -= 1;
                    }
                    b')' => right += 1,
                    _ => unreachable!(),
                }
            }
            if left == 0 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_be_valid(
            "))()))".to_string(),
            "010100".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(Solution::can_be_valid(
            "()()".to_string(),
            "0000".to_string()
        ));
    }

    #[test]
    fn case3() {
        assert!(!Solution::can_be_valid(")".to_string(), "0".to_string()));
    }
}
