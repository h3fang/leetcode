pub struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut left, mut ans) = (0, 0);
        let mut i = 0;
        while i < s.len() {
            if s[i] == b'(' {
                left += 1;
            } else {
                if left == 0 {
                    ans += 1;
                } else {
                    left -= 1;
                }
                if i + 1 == s.len() || s[i + 1] != b')' {
                    ans += 1;
                } else {
                    i += 1;
                }
            }
            i += 1;
        }

        left * 2 + ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_insertions("(()))".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_insertions("())".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::min_insertions("))())(".to_string()));
    }

    #[test]
    fn case4() {
        assert_eq!(12, Solution::min_insertions("((((((".to_string()));
    }

    #[test]
    fn case5() {
        assert_eq!(5, Solution::min_insertions(")))))))".to_string()));
    }
}
