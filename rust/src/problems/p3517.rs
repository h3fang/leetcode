pub struct Solution;

impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let n = s.len();
        let mut f = [0; 26];
        for &b in s.as_bytes() {
            f[(b - b'a') as usize] += 1;
        }
        let mut s = s.into_bytes();
        let mut k = 0;
        for (i, &f) in f.iter().enumerate() {
            if f == 0 {
                continue;
            }
            let c = b'a' + i as u8;
            for j in k..k + f / 2 {
                s[j] = c;
                s[n - j - 1] = c;
            }
            if f % 2 == 1 {
                s[n / 2] = c;
            }
            k += f / 2;
        }
        String::from_utf8(s).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("z", Solution::smallest_palindrome("z".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("abbba", Solution::smallest_palindrome("babab".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(
            "acddca",
            Solution::smallest_palindrome("daccad".to_string())
        );
    }
}
