pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn split_array_same_average(mut nums: Vec<i32>) -> bool {
        let n = nums.len() as i32;
        if n == 1 {
            return false;
        }
        let sum = nums.iter().sum::<i32>();
        nums.iter_mut().for_each(|e| *e = *e * n - sum);
        let m = n / 2;
        let mut left = HashSet::new();
        for i in 1..(1 << m) {
            let s = nums[..m as usize]
                .iter()
                .enumerate()
                .filter_map(|(j, e)| if i & (1 << j) > 0 { Some(*e) } else { None })
                .sum::<i32>();
            if s == 0 {
                return true;
            }
            left.insert(s);
        }

        let r_sum = nums[m as usize..].iter().sum::<i32>();
        for i in 1..(1 << (n - m)) {
            let s = nums[m as usize..]
                .iter()
                .enumerate()
                .filter_map(|(j, e)| if i & (1 << j) > 0 { Some(*e) } else { None })
                .sum::<i32>();
            if s == 0 || (s != r_sum && left.contains(&(-s))) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::split_array_same_average(vec![
            1, 2, 3, 4, 5, 6, 7, 8
        ]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::split_array_same_average(vec![3, 1]));
    }
}
