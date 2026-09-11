pub struct Solution;

impl Solution {
    pub fn make_smallest_palindrome(mut s: String) -> String {
        let a = unsafe { s.as_bytes_mut() };
        let n = a.len();
        (0..n / 2).for_each(|i| {
            let min = a[i].min(a[n - 1 - i]);
            a[i] = min;
            a[n - 1 - i] = min;
        });
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "efcfe",
            Solution::make_smallest_palindrome("egcfe".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "abba",
            Solution::make_smallest_palindrome("abcd".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "neven",
            Solution::make_smallest_palindrome("seven".to_string())
        );
    }
}
