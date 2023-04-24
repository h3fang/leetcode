pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut f = vec![0; n + 1];
        f[0] = 1;
        let k = k as i64;
        for i in 1..=n {
            let mut num = 0;
            let mut base = 1;
            for j in (i.saturating_sub(10)..i).rev() {
                num += (s[j] - b'0') as i64 * base;
                if num > k {
                    break;
                }
                if s[j] != b'0' {
                    f[i] += f[j];
                    f[i] %= MOD;
                }
                base *= 10;
            }
        }
        f[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::number_of_arrays("1000".to_string(), 10000));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::number_of_arrays("1000".to_string(), 10));
    }

    #[test]
    fn case3() {
        assert_eq!(8, Solution::number_of_arrays("1317".to_string(), 2000));
    }
}
