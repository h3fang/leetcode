pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        let (n, delay, forget) = (n as usize, delay as usize, forget as usize);
        let mut d = vec![0i64; n + 2];
        d[1] = 1;
        d[2] = -1;
        let (mut ans, mut aware) = (0, 0);
        for i in 1..n + 1 {
            aware = (aware + d[i] + MOD) % MOD;
            if i + forget > n {
                ans = (ans + aware) % MOD;
            }
            d[(i + delay).min(n + 1)] += aware;
            d[(i + forget).min(n + 1)] -= aware;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::people_aware_of_secret(6, 2, 4));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::people_aware_of_secret(4, 1, 3));
    }
}
