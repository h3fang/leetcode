pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut bits = [0; 30];
        for mut x in nums {
            let mut i = 0;
            while x > 0 {
                if x & 1 == 1 {
                    bits[i] += 1;
                }
                i += 1;
                x >>= 1;
            }
        }
        let mut ans = 0;
        for _ in 0..k {
            let mut x = 0;
            for (i, c) in bits.iter_mut().enumerate() {
                if *c > 0 {
                    *c -= 1;
                    x |= 1 << i;
                }
            }
            ans = (ans + x * x) % MOD;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(261, Solution::max_sum(vec![2, 6, 5, 8], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(90, Solution::max_sum(vec![4, 5, 4, 7], 3));
    }
}
