pub struct Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }
        let (mut min, mut max) = (0, 0);
        for (&a, &b) in s.as_bytes().iter().zip(locked.as_bytes()) {
            if b == b'0' {
                max += 1;
                min -= 1;
            } else {
                match a {
                    b'(' => {
                        min += 1;
                        max += 1;
                    }
                    _ => {
                        min -= 1;
                        max -= 1;
                    }
                }
            }
            if max < 0 {
                return false;
            }
            if min == -1 {
                min = 1;
            }
        }
        min == 0
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

    #[test]
    fn case4() {
        assert!(!Solution::can_be_valid(
            "(((())".to_string(),
            "111111".to_string()
        ));
    }
}
