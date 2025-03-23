pub struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let (mut min, mut max) = (0, 0);
        for &b in s.as_bytes() {
            match b {
                b'(' => {
                    min += 1;
                    max += 1;
                }
                b')' => {
                    min = (min - 1).max(0);
                    max -= 1;
                    if max < 0 {
                        return false;
                    }
                }
                b'*' => {
                    min = (min - 1).max(0);
                    max += 1;
                }
                _ => {}
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
