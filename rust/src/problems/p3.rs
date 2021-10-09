pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut left = 0;
        let mut result = 0;
        let mut prev = vec![usize::MAX; 256];

        for (right, c) in s.chars().enumerate() {
            if prev[c as usize] != usize::MAX && left <= prev[c as usize] {
                left = prev[c as usize] + 1;
            }
            prev[c as usize] = right;
            result = result.max(right - left + 1);
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "abcabcbb".to_string();
        assert_eq!(3, Solution::length_of_longest_substring(s));
    }

    #[test]
    fn case2() {
        let s = "bbbbbb".to_string();
        assert_eq!(1, Solution::length_of_longest_substring(s));
    }

    #[test]
    fn case3() {
        let s = "pwwkew".to_string();
        assert_eq!(3, Solution::length_of_longest_substring(s));
    }

    #[test]
    fn case4() {
        let s = "".to_string();
        assert_eq!(0, Solution::length_of_longest_substring(s));
    }
}
