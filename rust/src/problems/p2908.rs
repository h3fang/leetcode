pub struct Solution;

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut suf = vec![0; n];
        suf[n - 1] = nums[n - 1];
        for (i, &x) in nums.iter().enumerate().rev().skip(1) {
            suf[i] = suf[i + 1].min(x);
        }
        let mut result = i32::MAX;
        let mut min = nums[0];
        for (i, &x) in nums.iter().enumerate().skip(1).take(n - 2) {
            if x <= min {
                min = x;
            } else if x > suf[i + 1] {
                let r = x + min + suf[i + 1];
                result = result.min(r);
            }
        }
        if result == i32::MAX {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9, Solution::minimum_sum(vec![8, 6, 1, 5, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(13, Solution::minimum_sum(vec![5, 4, 8, 7, 10, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::minimum_sum(vec![6, 5, 4, 3, 4, 5]));
    }
}
