pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut f = [0; 256];
        for b in s.bytes() {
            f[b as usize] += 1;
        }
        let mut result = 0;
        for c in f {
            result += c / 2 * 2;
            if result % 2 == 0 && c % 2 == 1 {
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
        assert_eq!(7, Solution::longest_palindrome("abccccdd".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::longest_palindrome("a".to_string()));
    }
}
