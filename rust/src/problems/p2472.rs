pub struct Solution;

fn is_palindrome(s: &[u8]) -> bool {
    let (mut l, mut r) = (0, s.len() - 1);
    while l < r {
        if s[l] != s[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let (n, k) = (s.len(), k as usize);
        let s = s.as_bytes();
        let (mut ans, mut i) = (0, 0);
        while i <= n - k {
            if is_palindrome(&s[i..i + k]) {
                ans += 1;
                i += k;
            } else if i + k < n && is_palindrome(&s[i..i + k + 1]) {
                ans += 1;
                i += k + 1;
            } else {
                i += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::max_palindromes("abaccdbbd".to_string(), 3));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::max_palindromes("adbcda".to_string(), 2));
    }
}
