pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = 1 << n;
        let mut f = vec![vec![0; n]; m];
        for i in 0..n {
            f[1 << i][i] = 1;
        }
        for s in 1..m {
            for (i, &x) in nums.iter().enumerate() {
                if s & (1 << i) == 0 {
                    continue;
                }
                for (j, &y) in nums.iter().enumerate() {
                    if i == j || s & (1 << j) == 0 || (x % y != 0 && y % x != 0) {
                        continue;
                    }
                    f[s][i] = (f[s][i] + f[s ^ (1 << i)][j]) % MOD;
                }
            }
        }
        f[m - 1].iter().fold(0, |acc, x| (acc + x) % MOD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::special_perm(vec![2, 3, 6]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::special_perm(vec![1, 4, 3]));
    }
}
