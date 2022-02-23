pub struct Solution;

const PRIMES: [i32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
const MOD: i64 = 1000000007;

impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let mut freq = [0; 31];
        for &n in &nums {
            freq[n as usize] += 1;
        }

        let mut f = vec![0i64; 1 << PRIMES.len()];
        f[0] = 1;
        for _ in 0..freq[1] {
            f[0] = (f[0] * 2) % MOD;
        }
        for i in 2..=30 {
            if freq[i as usize] == 0 {
                continue;
            }
            let mut primes = 0;
            let mut valid = true;
            for (j, &p) in PRIMES.iter().enumerate() {
                if i % (p * p) == 0 {
                    valid = false;
                    break;
                }
                if i % p == 0 {
                    primes |= 1 << j;
                }
            }
            if !valid {
                continue;
            }
            for mask in (1..(1 << PRIMES.len())).rev() {
                if mask & primes == primes {
                    f[mask as usize] =
                        (f[mask as usize] + f[(mask ^ primes) as usize] * (freq[i as usize])) % MOD;
                }
            }
        }
        let mut result = 0;
        for n in &f[1..] {
            result = (result + n) % MOD;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::number_of_good_subsets(vec![1, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::number_of_good_subsets(vec![4, 2, 3, 15]));
    }
}
