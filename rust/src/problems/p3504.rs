pub struct Solution;

fn f(s: &[u8], t: &[u8]) -> i32 {
    let (m, n) = (s.len(), t.len());
    let mut f = vec![vec![0; n + 1]; m + 1];
    let mut max = vec![0; m + 1];
    for (i, &x) in s.iter().enumerate() {
        for (j, &y) in t.iter().enumerate() {
            if x == y {
                f[i + 1][j] = f[i][j + 1] + 1;
                max[i + 1] = max[i + 1].max(f[i + 1][j]);
            }
        }
    }
    let mut ans = max.iter().max().unwrap() * 2;
    for i in 0..2 * m as i32 - 1 {
        let (mut l, mut r) = (i / 2, (i + 1) / 2);
        while l >= 0 && r < m as i32 && s[l as usize] == s[r as usize] {
            l -= 1;
            r += 1;
        }
        if l + 1 < r {
            ans = ans.max(r - l - 1 + 2 * max[(l + 1) as usize]);
        }
    }

    ans
}

impl Solution {
    pub fn longest_palindrome(s: String, t: String) -> i32 {
        let (mut s, mut t) = (s.into_bytes(), t.into_bytes());
        let ans = f(&s, &t);
        s.reverse();
        t.reverse();
        ans.max(f(&t, &s))
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

    #[test]
    fn case5() {
        assert_eq!(
            2,
            Solution::longest_palindrome("i".to_string(), "ih".to_string())
        );
    }

    #[test]
    fn case6() {
        assert_eq!(
            3,
            Solution::longest_palindrome("gf".to_string(), "gkpb".to_string())
        );
    }
}
