pub struct Solution;

use std::sync::OnceLock;

const PRIMES: [i32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
const N: usize = 20;

static COMB: OnceLock<[[i32; N]; N]> = OnceLock::new();

fn init() -> [[i32; N]; N] {
    let mut comb = [[0; N]; N];
    for i in 0..N {
        comb[i][0] = 1;
        for j in 1..=i {
            comb[i][j] = comb[i - 1][j - 1] + comb[i - 1][j];
        }
    }
    comb
}

fn count(high: i32) -> i32 {
    let comb = COMB.get_or_init(init);
    let (mut ans, mut ones) = (0, 0);
    let w = i32::BITS - high.leading_zeros();
    for i in (0..w).rev() {
        if (high >> i) & 1 == 0 {
            continue;
        }
        for p in PRIMES {
            let c = p - ones;
            if c > i as i32 {
                break;
            }
            if c >= 0 {
                ans += comb[i as usize][c as usize];
            }
        }
        ones += 1;
    }
    ans
}

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        count(right + 1) - count(left)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::count_prime_set_bits(6, 10));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::count_prime_set_bits(10, 15));
    }
}
