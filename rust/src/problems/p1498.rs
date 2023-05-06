pub struct Solution;

const MOD: i32 = 10_0000_0007;

fn upper_bound(nums: &[i32], target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len();
    while l < r {
        let m = l + (r - l) / 2;
        if nums[m] > target {
            r = m;
        } else {
            l = m + 1;
        }
    }
    r as i32
}

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut f = vec![0; n];
        f[0] = 1;
        for i in 1..f.len() {
            f[i] = (f[i - 1] * 2) % MOD;
        }
        nums.sort_unstable();
        let mut result = 0;
        for (i, &min) in nums.iter().enumerate() {
            if min * 2 > target {
                break;
            }
            let max = target - min;
            let p = upper_bound(&nums, max) - 1;
            let a = if p >= i as i32 { f[p as usize - i] } else { 0 };
            result = (result + a) % MOD;
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
