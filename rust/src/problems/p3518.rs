pub struct Solution;

impl Solution {
    pub fn smallest_palindrome(mut s: String, k: i32) -> String {
        let n = s.len();
        let m = n / 2;
        let k = k as usize;

        let freq = {
            let mut f = [0; 26];
            for &b in &s.as_bytes()[..m] {
                f[(b - b'a') as usize] += 1;
            }
            f
        };

        let mut cnt = [0; 26];

        let (mut i, mut j, mut perm) = (m as i64 - 1, 25, 1);

        while i >= 0 && perm < k {
            while cnt[j] == freq[j] {
                j -= 1;
            }
            cnt[j] += 1;
            perm = perm * (m - i as usize) / cnt[j];

            i -= 1;
        }

        if perm < k {
            return String::new();
        }

        let ans = unsafe { s.as_bytes_mut() };
        let mut pos = 0;

        for (k, c) in cnt.iter().enumerate().take(j + 1) {
            let b = b'a' + k as u8;
            for _ in 0..freq[k] - c {
                ans[pos] = b;
                pos += 1;
            }
        }

        let (mut i, j0, mut k) = ((i + 1) as usize, j, k);

        while i < m {
            for (j, c) in cnt.iter_mut().enumerate().skip(j0) {
                if *c == 0 {
                    continue;
                }

                let p = perm * *c / (m - i);

                if p >= k {
                    ans[pos] = b'a' + j as u8;
                    pos += 1;
                    *c -= 1;
                    perm = p;
                    break;
                }

                k -= p;
            }

            i += 1;
        }

        for i in 0..n / 2 {
            ans[n - i - 1] = ans[i];
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("baab", Solution::smallest_palindrome("abba".to_string(), 2));
    }

    #[test]
    fn case2() {
        assert_eq!("", Solution::smallest_palindrome("aa".to_string(), 2));
    }

    #[test]
    fn case3() {
        assert_eq!(
            "abcba",
            Solution::smallest_palindrome("bacab".to_string(), 1)
        );
    }

    #[test]
    fn case4() {
        assert_eq!("ubu", Solution::smallest_palindrome("ubu".to_string(), 1));
    }

    #[test]
    fn case5() {
        assert_eq!(
            "ghdhhdhg",
            Solution::smallest_palindrome("ghdhhdhg".to_string(), 5)
        );
    }
}
