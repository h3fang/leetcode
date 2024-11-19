pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let (mut sum, mut result) = (0, 0);
        let mut m: HashMap<i32, u32> = HashMap::with_capacity(k);
        for &x in &nums[..k] {
            sum += x as i64;
            *m.entry(x).or_default() += 1;
        }
        if m.len() == k {
            result = sum;
        }
        for (i, &x) in nums.iter().enumerate().skip(k) {
            let e = m.get_mut(&nums[i - k]).unwrap();
            *e -= 1;
            if *e == 0 {
                m.remove(&nums[i - k]);
            }
            *m.entry(x).or_default() += 1;

            sum += (x - nums[i - k]) as i64;
            if m.len() == k {
                result = result.max(sum);
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
        assert_eq!(
            15,
            Solution::maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::maximum_subarray_sum(vec![4, 4, 4], 3));
    }
}
