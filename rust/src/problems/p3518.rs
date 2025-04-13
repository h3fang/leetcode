pub struct Solution;

fn comb(mut n: usize, x: usize, k: usize) -> usize {
    let (m, mut ans) = (x.min(n - x), 1);
    for i in 1..=m {
        ans = ans * n / i;
        if ans >= k {
            return k;
        }
        n -= 1;
    }
    ans
}

fn perm(f: &[i32], k: usize, mut m: usize) -> usize {
    let mut ans = 1;
    for &x in f {
        if x == 0 {
            continue;
        }
        let x = x as usize;
        ans *= comb(m, x, k);
        if ans >= k {
            return k;
        }
        m -= x;
    }
    ans
}

impl Solution {
    pub fn smallest_palindrome(mut s: String, k: i32) -> String {
        let n = s.len();
        let m = n / 2;

        let mut f = [0; 26];
        for &b in &s.as_bytes()[..m] {
            f[(b - b'a') as usize] += 1;
        }

        let mut k = k as usize;

        if perm(&f, k, m) < k {
            return String::new();
        }

        let ans = unsafe { s.as_bytes_mut() };

        for i in 0..m {
            for j in 0..26 {
                if f[j] == 0 {
                    continue;
                }
                f[j] -= 1;
                let total = perm(&f, k, m - i - 1);
                if total >= k {
                    let c = j as u8 + b'a';
                    ans[i] = c;
                    ans[n - i - 1] = c;
                    break;
                } else {
                    k -= total;
                    f[j] += 1;
                }
            }
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
