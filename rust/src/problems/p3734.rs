pub struct Solution;

impl Solution {
    pub fn lex_palindromic_permutation(s: String, target: String) -> String {
        let mut f = [0; 26];
        for s in s.bytes() {
            f[usize::from(s - b'a')] += 1;
        }

        let mut mid = b' ';
        for (i, c) in f.iter_mut().enumerate() {
            if *c % 2 == 0 {
                continue;
            }
            if mid != b' ' {
                return String::new();
            }
            mid = i as u8 + b'a';
            *c -= 1;
        }

        let mut s = s.into_bytes();
        let t = target.as_bytes();
        let n = t.len();

        for &b in &t[..n / 2] {
            f[usize::from(b - b'a')] -= 2;
        }

        let m = n.div_ceil(2);

        if f.iter().all(|&e| e >= 0) {
            s[..n / 2].copy_from_slice(&t[..n / 2]);
            if mid != b' ' {
                s[n / 2] = mid;
            }
            s[m..].copy_from_slice(&t[..n / 2]);
            s[m..].reverse();
            if &s[..] > t {
                return unsafe { String::from_utf8_unchecked(s) };
            }
        }

        for (i, &x) in t[..n / 2].iter().enumerate().rev() {
            f[usize::from(x - b'a')] += 2;
            if f.iter().any(|&e| e < 0) {
                continue;
            }
            for y in x + 1..=b'z' {
                let j = usize::from(y - b'a');
                if f[j] == 0 {
                    continue;
                }
                f[j] -= 2;

                s[..i].copy_from_slice(&t[..i]);
                s[i] = y;
                s.truncate(i + 1);

                for (k, &c) in f.iter().enumerate() {
                    let b = k as u8 + b'a';
                    for _ in 0..c / 2 {
                        s.push(b);
                    }
                }
                if mid != b' ' {
                    s.push(mid);
                }
                s.resize(n, b' ');
                let (left, right) = s.split_at_mut(m);
                right.copy_from_slice(&left[..n / 2]);
                right.reverse();
                return unsafe { String::from_utf8_unchecked(s) };
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "baab",
            Solution::lex_palindromic_permutation("baba".to_string(), "abba".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "",
            Solution::lex_palindromic_permutation("baba".to_string(), "bbaa".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "",
            Solution::lex_palindromic_permutation("abc".to_string(), "abb".to_string())
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            "aca",
            Solution::lex_palindromic_permutation("aac".to_string(), "abb".to_string())
        );
    }
}
