pub struct Solution;

fn is_palindrome(s: &[u8], t: &[u8]) -> bool {
    let n = s.len() + t.len();
    for i in 0..n / 2 {
        let a = if i < s.len() { s[i] } else { t[i - s.len()] };
        let b = if (n - i - 1) < s.len() {
            s[n - i - 1]
        } else {
            t[n - i - 1 - s.len()]
        };
        if a != b {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn longest_palindrome(s: String, t: String) -> i32 {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let mut ans = 1;
        for a in 0..=s.len() {
            for b in a..=s.len() {
                for c in 0..=t.len() {
                    for d in c..=t.len() {
                        if is_palindrome(&s[a..b], &t[c..d]) {
                            ans = (b - a + d - c).max(ans);
                        }
                    }
                }
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::longest_palindrome("a".to_string(), "a".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::longest_palindrome("abc".to_string(), "def".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            4,
            Solution::longest_palindrome("b".to_string(), "aaaa".to_string())
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            5,
            Solution::longest_palindrome("abcde".to_string(), "ecdba".to_string())
        );
    }
}
