pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let mut f = vec![0; high as usize + 1];
        f[0] = 1;
        for i in 1..f.len() {
            for c in [zero, one] {
                if i as i32 >= c {
                    f[i] = (f[i] + f[i - c as usize]) % MOD;
                }
            }
        }
        f[low as usize..=high as usize]
            .iter()
            .fold(0, |acc, &n| (acc + n) % MOD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(8, Solution::count_good_strings(3, 3, 1, 1));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::count_good_strings(2, 3, 1, 2));
    }
}
