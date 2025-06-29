pub struct Solution;

use std::sync::OnceLock;

static F: OnceLock<Vec<i32>> = OnceLock::new();

const MOD: i32 = 10_0000_0007;

fn init() -> Vec<i32> {
    let mut f = vec![0; 10_0000];
    f[0] = 1;
    for i in 1..f.len() {
        f[i] = (f[i - 1] * 2) % MOD;
    }
    f
}

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let f = F.get_or_init(init);
        nums.sort_unstable();
        let (mut l, mut r) = (0, n as i32 - 1);
        let mut result = 0;
        while l <= r {
            if nums[l as usize] + nums[r as usize] <= target {
                result = (result + f[(r - l) as usize]) % MOD;
                l += 1;
            } else {
                r -= 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::num_subseq(vec![3, 5, 6, 7], 9));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::num_subseq(vec![3, 3, 6, 8], 10));
    }

    #[test]
    fn case3() {
        assert_eq!(61, Solution::num_subseq(vec![2, 3, 3, 4, 6, 7], 12));
    }
}
