pub struct Solution;

impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut result = 0;
        while i < s.len() {
            if s[i] == b'X' {
                result += 1;
                i += 3;
            } else {
                i += 1;
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
        assert_eq!(2, Solution::minimum_moves("XXOX".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::minimum_moves("XXX".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::minimum_moves("OOOO".to_string()));
    }
}
