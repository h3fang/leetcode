pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut m1: HashMap<i32, i32> = HashMap::new();
        let mut m2: HashMap<i32, i32> = HashMap::new();
        let (mut i1, mut i2) = (0, 0);
        let mut result = 0;
        for &x in &nums {
            *m1.entry(x).or_default() += 1;
            *m2.entry(x).or_default() += 1;
            while m1.len() > k {
                let e = m1.get_mut(&nums[i1]).unwrap();
                *e -= 1;
                if *e == 0 {
                    m1.remove(&nums[i1]);
                }
                i1 += 1;
            }
            while m2.len() > k - 1 {
                let e = m2.get_mut(&nums[i2]).unwrap();
                *e -= 1;
                if *e == 0 {
                    m2.remove(&nums[i2]);
                }
                i2 += 1;
            }
            result += i2 - i1;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            7,
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            3,
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3)
        );
    }
}
