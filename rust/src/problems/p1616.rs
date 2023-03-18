pub struct Solution;

impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        let a = a.as_bytes();
        let b = b.as_bytes();
        fn check(a: &[u8], b: &[u8]) -> bool {
            let n = a.len();
            let (mut i, mut j) = (0, n - 1);
            while i < j {
                if a[i] != b[j] {
                    break;
                }
                i += 1;
                j -= 1;
            }
            i >= j || is_palindrome(&a[i..=j]) || is_palindrome(&b[i..=j])
        }
        fn is_palindrome(s: &[u8]) -> bool {
            let n = s.len() / 2;
            s.iter()
                .take(n)
                .zip(s.iter().rev().take(n))
                .all(|(&a, &b)| a == b)
        }
        check(a, b) || check(b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_palindrome_formation(
            "x".to_string(),
            "y".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(Solution::check_palindrome_formation(
            "abdef".to_string(),
            "fecab".to_string()
        ));
    }

    #[test]
    fn case3() {
        assert!(Solution::check_palindrome_formation(
            "ulacfd".to_string(),
            "jizalu".to_string()
        ));
    }
}
