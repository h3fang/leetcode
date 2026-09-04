pub struct Solution;

impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let (mut zeros, mut ones, mut max) = (0, 0, 0);
        for &c in s.as_bytes() {
            if c == b'0' {
                if ones > 0 {
                    max = max.max(ones.min(zeros));
                    zeros = 0;
                    ones = 0;
                }
                zeros += 1;
            } else {
                ones += 1;
            }
        }
        max = max.max(ones.min(zeros));
        max * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            6,
            Solution::find_the_longest_balanced_substring("01000111".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            4,
            Solution::find_the_longest_balanced_substring("00111".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            0,
            Solution::find_the_longest_balanced_substring("111".to_string())
        );
    }
}
