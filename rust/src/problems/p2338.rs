pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        let mut b = vec![[0; 14]; n as usize + 13];
        b[0][0] = 1;
        for i in 1..n as usize + 13 {
            b[i][0] = 1;
            for j in 1..=i.min(13) {
                b[i][j] = (b[i - 1][j] + b[i - 1][j - 1]) % MOD;
            }
        }

        let mut exp_of_factors = vec![vec![]; max_value as usize + 1];
        for i in 2..=max_value {
            let mut x = i;
            let mut p = 2;
            while p * p <= x {
                if x % p == 0 {
                    let mut c = 0;
                    while x % p == 0 {
                        c += 1;
                        x /= p;
                    }
                    exp_of_factors[i as usize].push(c);
                }
                p += 1;
            }
            if x > 1 {
                exp_of_factors[i as usize].push(1);
            }
        }

        let mut result = 0;
        for x in 1..=max_value {
            let mut c = 1;
            for &k in &exp_of_factors[x as usize] {
                c = (c * b[(n + k - 1) as usize][k as usize]) % MOD;
            }
            result = (result + c) % MOD;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(10, Solution::ideal_arrays(2, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(11, Solution::ideal_arrays(5, 3));
    }
}
