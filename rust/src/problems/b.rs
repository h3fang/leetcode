pub struct Solution;

impl Solution {
    pub fn longest_palindromic_subsequence(s: String, k: i32) -> i32 {
        let (n, k) = (s.len(), k as usize);
        let s = s.into_bytes();
        let mut f = vec![vec![vec![0; k + 1]; n]; n];
        for (i, v) in f.iter_mut().enumerate() {
            v[i].iter_mut().for_each(|x| *x = 1);
        }
        for len in 2..=n {
            for i in 0..n - len + 1 {
                let j = i + len - 1;
                for l in 0..=k {
                    if s[i] == s[j] {
                        f[i][j][l] = f[i + 1][j - 1][l] + 2;
                    } else {
                        let d = s[i].abs_diff(s[j]);
                        let d = d.min(26 - d) as usize;
                        if l >= d {
                            f[i][j][l] = f[i + 1][j - 1][l - d] + 2;
                        }
                        f[i][j][l] = f[i][j][l].max(f[i][j - 1][l]).max(f[i + 1][j][l]);
                    }
                }
            }
        }
        f[0][n - 1][k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::longest_palindromic_subsequence("abced".to_string(), 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            6,
            Solution::longest_palindromic_subsequence("aaazzz".to_string(), 4)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            3,
            Solution::longest_palindromic_subsequence("wehzr".to_string(), 3)
        );
    }
}
