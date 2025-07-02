pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn possible_string_count(word: String, mut k: i32) -> i32 {
        let n = word.len();
        if n < k as usize {
            return 0;
        }
        let w = word.as_bytes();
        let mut freq = Vec::with_capacity(k as usize);
        let (mut c, mut ans) = (0, 1);
        for (i, &b) in w.iter().enumerate() {
            c += 1;
            if i + 1 == n || w[i + 1] != b {
                if c > 1 {
                    if k > 0 {
                        freq.push(c - 1);
                    }
                    ans = (ans * c) % MOD;
                }
                k -= 1;
                c = 0;
            }
        }
        if k <= 0 {
            return ans as i32;
        }
        let k = k as usize;
        let mut f = vec![1; k];
        for c in freq {
            for j in 1..k {
                f[j] = (f[j - 1] + f[j]) % MOD;
            }
            for j in (1 + c as usize..k).rev() {
                f[j] = (f[j] - f[j - c as usize - 1]) % MOD;
            }
        }
        ((ans - f[k - 1] + MOD) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            5,
            Solution::possible_string_count("aabbccdd".to_string(), 7)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::possible_string_count("aabbccdd".to_string(), 8)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(8, Solution::possible_string_count("aaabbb".to_string(), 3));
    }
}
