pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let median = ((n + 1) * n / 2).div_ceil(2);

        let check = |m: usize| {
            let mut k = 0;
            let mut freq = HashMap::with_capacity(n);
            let mut j = 0;
            for (i, &x) in nums.iter().enumerate() {
                *freq.entry(x).or_insert(0) += 1;
                while freq.len() > m {
                    let e = freq.entry(nums[j]).or_insert(0);
                    *e -= 1;
                    if *e == 0 {
                        freq.remove(&nums[j]);
                    }
                    j += 1;
                }
                k += i - j + 1;
            }
            k >= median
        };
        let (mut l, mut r) = (1, n);
        let mut result = 0;
        while l <= r {
            let m = l + (r - l) / 2;
            if check(m) {
                result = m;
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::median_of_uniqueness_array(vec![1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::median_of_uniqueness_array(vec![3, 4, 3, 4, 5]));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::median_of_uniqueness_array(vec![4, 3, 5, 4]));
    }
}
