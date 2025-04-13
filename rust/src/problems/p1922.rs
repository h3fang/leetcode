pub struct Solution;

const MOD: i64 = 10_0000_0007;

fn quick_pow(mut x: i64, mut n: i64) -> i64 {
    let mut ans = 1;
    while n > 0 {
        if n & 1 > 0 {
            ans = (ans * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    ans
}

impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        let even = (n + 1) / 2;
        let odd = n - even;
        ((quick_pow(5, even) * quick_pow(4, odd)) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::count_good_numbers(1));
    }

    #[test]
    fn case2() {
        assert_eq!(400, Solution::count_good_numbers(4));
    }

    #[test]
    fn case3() {
        assert_eq!(564908303, Solution::count_good_numbers(50));
    }
}
