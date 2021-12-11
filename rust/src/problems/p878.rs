const MOD: i64 = 10_0000_0007;

pub struct Solution;

impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        fn gcd(x: i32, y: i32) -> i32 {
            if x == 0 {
                y
            } else {
                gcd(y % x, x)
            }
        }

        let l = a / gcd(a, b) * b;
        let m = l / a + l / b - 1;
        let q = n / m;
        let r = n % m;

        let mut ans: i64 = q as i64 * l as i64 % MOD;
        if r == 0 {
            return ans as i32;
        }
        let mut heads = (a, b);
        for _ in 0..r - 1 {
            if heads.0 <= heads.1 {
                heads.0 += a;
            } else {
                heads.1 += b;
            }
        }

        ans += heads.0.min(heads.1) as i64;
        (ans % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {}
}
