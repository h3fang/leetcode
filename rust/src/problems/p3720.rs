pub struct Solution;

impl Solution {
    pub fn lex_greater_permutation(mut s: String, target: String) -> String {
        let mut f = [0; 26];
        for (s, t) in s.bytes().zip(target.bytes()) {
            f[usize::from(s - b'a')] += 1;
            f[usize::from(t - b'a')] -= 1;
        }
        s.clone_from(&target);

        let mut s = s.into_bytes();

        for (i, t) in target.bytes().enumerate().rev() {
            f[usize::from(t - b'a')] += 1;
            if f.iter().any(|&e| e < 0) {
                continue;
            }

            for b in t + 1..=b'z' {
                let j = usize::from(b - b'a');
                if f[j] > 0 {
                    f[j] -= 1;
                    s[i] = b;
                    s.truncate(i + 1);
                    for (i, &c) in f.iter().enumerate() {
                        let b = i as u8 + b'a';
                        for _ in 0..c {
                            s.push(b);
                        }
                    }
                    return unsafe { String::from_utf8_unchecked(s) };
                }
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
            "bca",
            Solution::lex_greater_permutation("abc".to_string(), "bba".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "eelt",
            Solution::lex_greater_permutation("leet".to_string(), "code".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "",
            Solution::lex_greater_permutation("baba".to_string(), "bbaa".to_string())
        );
    }
}
