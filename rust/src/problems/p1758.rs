pub struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        fn find_operaations(s: &[u8], mut c: u8) -> i32 {
            let mut result = 0;
            for &e in s {
                if e != c {
                    result += 1;
                }
                if c == b'1' {
                    c = b'0';
                } else {
                    c = b'1';
                }
            }
            result
        }
        find_operaations(s.as_bytes(), b'1').min(find_operaations(s.as_bytes(), b'0'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_operations("0100".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_operations("10".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::min_operations("1111".to_string()));
    }
}
