pub struct Solution;

impl Solution {
    pub fn seconds_to_remove_occurrences(s: String) -> i32 {
        let s = s.as_bytes();
        let mut result = 0;
        let mut zeros = 0;
        for &c in s {
            if c == b'0' {
                zeros += 1;
            } else if zeros > 0 {
                result = (result + 1).max(zeros);
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
        assert_eq!(
            4,
            Solution::seconds_to_remove_occurrences("0110101".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            0,
            Solution::seconds_to_remove_occurrences("11100".to_string())
        );
    }
}
