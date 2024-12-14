pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

const MOD: i64 = 10_0000_0007;

fn pow(mut a: i64, mut b: i32) -> i64 {
    a %= MOD;
    let mut result = 1;
    while b > 0 {
        if b & 1 == 1 {
            result = (result * a) % MOD;
        }
        a = (a * a) % MOD;
        b >>= 1;
    }
    result
}

impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, mut k: i32, multiplier: i32) -> Vec<i32> {
        if multiplier == 1 {
            return nums;
        }
        let max = *nums.iter().max().unwrap() as i64;
        let mut q = BinaryHeap::with_capacity(nums.len());
        for (i, &x) in nums.iter().enumerate() {
            q.push((Reverse(x as i64), Reverse(i)));
        }
        let multiplier = multiplier as i64;
        while k > 0 && q.peek().unwrap().0 .0 < max {
            let (Reverse(x), i) = q.pop().unwrap();
            q.push((Reverse(x * multiplier), i));
            k -= 1;
        }
        let (mut i, n) = (0, nums.len() as i32);
        let m1 = pow(multiplier, k / n);
        let m2 = (m1 * multiplier) % MOD;
        while let Some((Reverse(x), Reverse(j))) = q.pop() {
            let mul = if i < k % n { m2 } else { m1 };
            nums[j] = ((x % MOD * mul) % MOD) as i32;
            i += 1;
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![8, 4, 6, 5, 6],
            Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![16, 8], Solution::get_final_state(vec![1, 2], 3, 4));
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![999999307, 999999993],
            Solution::get_final_state(vec![100000, 2000], 2, 1000000)
        );
    }
}
