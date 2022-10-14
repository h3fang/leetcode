pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let s = s.as_bytes();
        let mut f = vec![1; s.len()];
        let mut last = [-1; 26];
        for (i, b) in s.iter().enumerate() {
            let k = (b - b'a') as usize;
            for &j in &last {
                if j != -1 {
                    f[i] = (f[i] + f[j as usize]) % MOD;
                }
            }
            last[k] = i as i32;
        }
        last.iter()
            .filter(|&&k| k != -1)
            .map(|&k| f[k as usize])
            .fold(0, |acc, e| (acc + e) % MOD)
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
