pub struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();
        let mut pi = vec![0; n];
        for i in 1..n {
            let mut j = pi[i - 1];
            while j > 0 && b[i] != b[j] {
                j = pi[j - 1];
            }
            if b[i] == b[j] {
                j += 1;
            }
            pi[i] = j;
        }
        let mut j = 0;
        for i in (0..n).rev() {
            while j > 0 && b[j] != b[i] {
                j = pi[j - 1];
            }
            if b[j] == b[i] {
                j += 1;
            }
        }
        if j == n {
            s
        } else {
            let prefix = b[j..]
                .iter()
                .rev()
                .map(|b| (*b) as char)
                .collect::<String>();
            prefix + &s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "aaacecaaa",
            Solution::shortest_palindrome("aacecaaa".into())
        );
    }

    #[test]
    fn case2() {
        assert_eq!("dcbabcd", Solution::shortest_palindrome("abcd".into()));
    }
}
