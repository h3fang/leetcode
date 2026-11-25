pub struct Solution;

use std::collections::HashMap;

const MOD: i64 = 2i64.pow(31) - 1;
const BASE: i64 = 2i64.pow(19) - 1;

fn hash(hashes: &[i64], bases: &[i64], l: usize, r: usize) -> i64 {
    (hashes[r] - hashes[l] * bases[r - l]).rem_euclid(MOD)
}

fn check(hashes: &[i64], bases: &[i64], m: usize) -> bool {
    let n = hashes.len() - 1;
    let mut freq = HashMap::with_capacity(n);
    for i in m..n + 1 {
        let h = hash(hashes, bases, i - m, i);
        *freq.entry(h).or_insert(0) += 1;
    }
    freq.values().any(|v| *v == 1)
}

impl Solution {
    pub fn smallest_unique_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut bases = vec![0; n + 1];
        let mut hashes = vec![0; n + 1];

        bases[0] = 1;
        for (i, &x) in nums.iter().enumerate() {
            bases[i + 1] = (bases[i] * BASE) % MOD;
            hashes[i + 1] = (hashes[i] * BASE + i64::from(x)) % MOD;
        }

        let (mut l, mut r) = (0, n);

        while l + 1 < r {
            let m = l.midpoint(r);
            if check(&hashes, &bases, m) {
                r = m;
            } else {
                l = m;
            }
        }

        r as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::smallest_unique_subarray(vec![3, 3, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::smallest_unique_subarray(vec![2, 1, 2, 3, 3]));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::smallest_unique_subarray(vec![1, 1, 2, 2, 1]));
    }
}
