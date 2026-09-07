pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        let mut f = [0; 26];
        for b in s {
            let k = (b - b'a') as usize;
            let prev = f[k];
            f[k] = (ans + 1) % MOD;
            ans = ((ans + f[k] - prev) % MOD + MOD) % MOD;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(7, Solution::distinct_subseq_ii("abc".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::distinct_subseq_ii("aba".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::distinct_subseq_ii("aaa".to_string()));
    }
}
