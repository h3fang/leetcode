pub struct Solution;

const MOD: u64 = 10_0000_0007;

fn pow(mut x: u64, mut y: u64) -> u64 {
    let mut result = 1;
    while y > 0 {
        if y % 2 == 1 {
            result = (result * x) % MOD;
        }
        x = (x * x) % MOD;
        y >>= 1;
    }
    result
}

impl Solution {
    pub fn min_non_zero_product(p: i32) -> i32 {
        if p == 1 {
            return 1;
        }
        let x = ((1 << p) - 1) % MOD;
        let y = 1 << (p - 1);
        ((x * pow(x - 1, y - 1)) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_non_zero_product(1));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::min_non_zero_product(2));
    }

    #[test]
    fn case3() {
        assert_eq!(1512, Solution::min_non_zero_product(3));
    }

    #[test]
    fn case4() {
        assert_eq!(505517599, Solution::min_non_zero_product(32));
    }

    #[test]
    fn case5() {
        assert_eq!(813987236, Solution::min_non_zero_product(60));
    }
}
