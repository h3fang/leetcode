pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let mut f = [0; 26];
        for b in s.bytes() {
            f[(b - b'a') as usize] += 1i32;
        }
        for _ in 0..t {
            let mut g = [0; 26];
            g[0] = f[25];
            g[1] = (f[0] + f[25]) % MOD;
            for (i, &e) in f.iter().enumerate().skip(1).take(24) {
                g[i + 1] = e;
            }
            f = g;
        }
        f.into_iter().fold(0, |acc, x| (acc + x) % MOD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            7,
            Solution::length_after_transformations("abcyy".to_string(), 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            5,
            Solution::length_after_transformations("azbk".to_string(), 1)
        );
    }
}
