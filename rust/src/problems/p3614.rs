pub struct Solution;

impl Solution {
    pub fn process_str(s: String, mut k: i64) -> char {
        let s = s.as_bytes();
        let mut len = 0;
        for &b in s {
            match b {
                b'*' => {
                    if len > 0 {
                        len -= 1;
                    }
                }
                b'#' => len *= 2,
                b'%' => {}
                _ => len += 1,
            }
        }
        if k >= len {
            return '.';
        }
        for &b in s.iter().rev() {
            match b {
                b'#' => {
                    if k >= len / 2 {
                        k -= len / 2;
                    }
                    len /= 2;
                }
                b'%' => k = len - 1 - k,
                b'*' => len += 1,
                c => {
                    if k == len - 1 {
                        return c as char;
                    }
                    len -= 1;
                }
            }
        }
        '.'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!('a', Solution::process_str("a#b%*".to_string(), 1));
    }

    #[test]
    fn case2() {
        assert_eq!('d', Solution::process_str("cd%#*#".to_string(), 3));
    }

    #[test]
    fn case3() {
        assert_eq!('.', Solution::process_str("z*#".to_string(), 0));
    }
}
