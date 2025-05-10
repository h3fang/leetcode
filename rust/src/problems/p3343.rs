pub struct Solution;

const MOD: i64 = 10_0000_0007;

fn pow(mut x: i64, mut n: i64) -> i64 {
    let mut ans = 1;
    while n > 0 {
        if n % 2 == 1 {
            ans = (ans * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    ans
}

impl Solution {
    pub fn count_balanced_permutations(num: String) -> i32 {
        let mut fac = [0i64; 41];
        fac[0] = 1;
        for i in 1..41 {
            fac[i] = (fac[i - 1] * i as i64) % MOD;
        }

        let mut inv = [0i64; 41];
        inv[40] = pow(fac[40], MOD - 2);
        for i in (1..41).rev() {
            inv[i - 1] = (inv[i] * i as i64) % MOD;
        }
        let mut count = [0; 10];
        let mut total = 0;
        for b in num.bytes() {
            let b = (b - b'0') as usize;
            count[b] += 1;
            total += b as i64;
        }
        if total % 2 != 0 {
            return 0;
        }
        let n = num.len() as i64;
        let n1 = n / 2;
        let mut f = vec![vec![0i64; total as usize / 2 + 1]; n1 as usize + 1];
        f[0][0] = 1;
        let (mut sc, mut s) = (0, 0);
        for (i, c) in count.into_iter().enumerate() {
            sc += c;
            s += c * i as i64;
            for l1 in ((sc - (n - n1)).max(0)..=sc.min(n1)).rev() {
                let l2 = sc - l1;
                for ls in ((s - total / 2).max(0)..=s.min(total / 2)).rev() {
                    let mut r = 0;
                    for k in (c - l2).max(0)..=c.min(l1) {
                        if k * i as i64 > ls {
                            break;
                        }
                        r = (r
                            + ((f[(l1 - k) as usize][(ls - k * i as i64) as usize]
                                * inv[k as usize])
                                % MOD)
                                * inv[(c - k) as usize])
                            % MOD;
                    }
                    f[l1 as usize][ls as usize] = r;
                }
            }
        }
        let r = ((fac[n1 as usize] * fac[(n - n1) as usize]) % MOD
            * f[n1 as usize][(total / 2) as usize])
            % MOD;
        r as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::count_balanced_permutations("123".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::count_balanced_permutations("112".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(
            0,
            Solution::count_balanced_permutations("12345".to_string())
        );
    }

    #[test]
    fn case4() {
        assert_eq!(4, Solution::count_balanced_permutations("5353".to_string()));
    }
}
