pub struct Solution;

use std::sync::LazyLock;

const MOD: i32 = 10_0000_0007;

static GCD: LazyLock<[[u16; 201]; 201]> = LazyLock::new(|| {
    let mut f = [[0; 201]; 201];
    for (i, r) in f.iter_mut().enumerate() {
        for (j, c) in r.iter_mut().enumerate() {
            *c = gcd(i as u16, j as u16);
        }
    }
    f
});

fn gcd(mut a: u16, mut b: u16) -> u16 {
    while a != 0 {
        (a, b) = (b % a, a);
    }
    b
}

impl Solution {
    pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap() as usize;
        let mut f = vec![vec![0; max + 1]; max + 1];
        (1..=max).for_each(|j| f[j][j] = 1);

        for x in nums {
            let mut next = vec![vec![0; max + 1]; max + 1];
            for j in 0..=max {
                for k in 0..=max {
                    let j1 = GCD[x as usize][j] as usize;
                    let k1 = GCD[x as usize][k] as usize;
                    next[j][k] = ((f[j][k] + f[j1][k]) % MOD + f[j][k1]) % MOD;
                }
            }
            f = next;
        }

        f[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(10, Solution::subsequence_pair_count(vec![1, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::subsequence_pair_count(vec![10, 20, 30]));
    }
}
