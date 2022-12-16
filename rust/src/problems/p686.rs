pub struct Solution;

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        fn kmp(a: &[u8], b: &[u8]) -> i32 {
            let m = a.len();
            let n = b.len();
            if n == 0 {
                return 0;
            }
            let mut p = vec![0; n];
            let mut j = 0;
            for i in 1..n {
                while j > 0 && b[i] != b[j] {
                    j = p[j - 1];
                }
                if b[i] == b[j] {
                    j += 1;
                }
                p[i] = j;
            }
            let mut i = 0;
            j = 0;
            while i - j < m {
                while j > 0 && a[i % m] != b[j] {
                    j = p[j - 1];
                }
                if a[i % m] == b[j] {
                    j += 1;
                }
                if j == n {
                    return (i + 1 - n) as i32;
                }
                i += 1;
            }
            -1
        }
        let m = a.len() as i32;
        let n = b.len() as i32;
        let i = kmp(a.as_bytes(), b.as_bytes());
        if i == -1 {
            return -1;
        }
        if m - i >= n {
            return 1;
        }
        (n + i - m - 1) / m + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let a = "abcd".to_string();
        let b = "cdabcdab".to_string();
        assert_eq!(3, Solution::repeated_string_match(a, b));
    }

    #[test]
    fn case2() {
        let a = "a".to_string();
        let b = "aa".to_string();
        assert_eq!(2, Solution::repeated_string_match(a, b));
    }
}
