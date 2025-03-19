pub struct Solution;

const MOD: i32 = 10_0000_0007;

fn gcd(a: usize, b: usize) -> i32 {
    if a == 0 { b as i32 } else { gcd(b % a, a) }
}

impl Solution {
    pub fn distinct_sequences(n: i32) -> i32 {
        if n == 1 {
            return 6;
        }
        let n = n as usize;
        let mut f = vec![vec![vec![0; 6]; 6]; n + 1];
        for i in 0..6 {
            for j in 0..6 {
                if i != j && gcd(i + 1, j + 1) == 1 {
                    f[2][i][j] = 1;
                }
            }
        }
        for k in 2..n {
            for c in 0..6 {
                for b in 0..6 {
                    if b != c && gcd(c + 1, b + 1) == 1 {
                        let mut cnt = 0;
                        for a in 0..6 {
                            if a != c {
                                cnt = (cnt + f[k][b][a]) % MOD;
                            }
                        }
                        f[k + 1][c][b] = cnt;
                    }
                }
            }
        }
        f[n].iter().flatten().fold(0, |acc, e| (acc + e) % MOD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(184, Solution::distinct_sequences(4));
    }

    #[test]
    fn case2() {
        assert_eq!(22, Solution::distinct_sequences(2));
    }
}
