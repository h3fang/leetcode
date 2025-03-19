pub struct Solution;

impl Solution {
    pub fn minimum_subarray_length(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let (mut result, mut l, mut b, mut or) = (n + 1, 0, 0, 0);
        for r in 0..n {
            or |= nums[r];
            while l <= r && nums[l] | or >= k {
                result = result.min(r - l + 1);
                l += 1;
                if b < l {
                    for i in (l..r).rev() {
                        nums[i] |= nums[i + 1];
                    }
                    b = r;
                    or = 0;
                }
            }
        }
        if result == n + 1 { -1 } else { result as i32 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::minimum_subarray_length(vec![1, 2, 3], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::minimum_subarray_length(vec![2, 1, 8], 10));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::minimum_subarray_length(vec![1, 2], 0));
    }
}
