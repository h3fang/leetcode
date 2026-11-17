pub struct Solution;

use std::sync::LazyLock;

const SIZE: usize = 201;

static DIVISORS: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| {
    let mut f = vec![vec![]; SIZE];
    for i in 1..SIZE {
        for j in (i * 2..SIZE).step_by(i) {
            f[j].push(i as i32);
        }
    }
    f
});

fn changes(s: &[u8]) -> i32 {
    let n = s.len();
    let mut ans = n as i32;
    for &d in &DIVISORS[n] {
        let mut cnt = 0;
        for i0 in 0..d {
            let (mut i, mut j) = (i0, n as i32 - d + i0);
            while i < j {
                cnt += i32::from(s[i as usize] != s[j as usize]);
                i += d;
                j -= d;
            }
        }
        ans = ans.min(cnt);
    }
    ans
}

impl Solution {
    pub fn minimum_changes(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut cnt = vec![vec![0; n]; n - 1];

        for l in 0..n - 1 {
            for r in l + 1..n {
                cnt[l][r] = changes(&s[l..r + 1]);
            }
        }

        let mut f = cnt[0].clone();

        for i in 1..k {
            for j in (i * 2 + 1..n as i32 - (k - i - 1) * 2).rev() {
                f[j as usize] = i32::MAX;
                for l in i * 2..j {
                    f[j as usize] =
                        f[j as usize].min(f[l as usize - 1] + cnt[l as usize][j as usize]);
                }
            }
        }

        f[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::minimum_changes("abcac".to_string(), 2));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::minimum_changes("abcdef".to_string(), 2));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::minimum_changes("aabbaa".to_string(), 3));
    }
}
