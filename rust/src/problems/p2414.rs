pub struct Solution;

impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut result = 0;
        let mut i = 0;
        while i < s.len() {
            let mut j = i;
            while j + 1 < s.len() && s[j + 1] == s[j] + 1 {
                j += 1;
            }
            result = result.max(j - i + 1);
            i = j + 1;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::longest_continuous_substring("abacaba".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            5,
            Solution::longest_continuous_substring("abcde".to_string())
        );
    }
}
