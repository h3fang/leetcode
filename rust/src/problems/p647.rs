pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut result = 0;
        for i in 0..n {
            result += 1;
            for k in 1..=i.min(n - 1 - i) {
                if s[i - k] != s[i + k] {
                    break;
                }
                result += 1;
            }
        }
        for i in 0..n - 1 {
            for k in 0..=i.min(n - 2 - i) {
                if s[i - k] != s[i + 1 + k] {
                    break;
                }
                result += 1;
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
        assert_eq!(3, Solution::count_substrings("abc".into()));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::count_substrings("aaa".into()));
    }
}
