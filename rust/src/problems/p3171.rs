pub struct Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut result = i32::MAX;
        let (mut l, mut bottom, mut right_or) = (0, 0, 0);
        for r in 0..nums.len() {
            let x = nums[r];
            right_or |= x;
            while l <= r && (nums[l] | right_or) > k {
                result = result.min((nums[l] | right_or) - k);
                l += 1;
                if bottom < l {
                    for i in (l..r).rev() {
                        nums[i] |= nums[i + 1];
                    }
                    bottom = r;
                    right_or = 0;
                }
            }
            if l <= r {
                result = result.min(k - (nums[l] | right_or));
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
        assert_eq!(0, Solution::minimum_difference(vec![1, 2, 4, 5], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::minimum_difference(vec![1, 3, 1, 3], 2));
    }

    #[test]
    fn case3() {
        assert_eq!(9, Solution::minimum_difference(vec![1], 10));
    }
}
