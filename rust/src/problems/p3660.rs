pub struct Solution;

impl Solution {
    pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut pre_max = vec![0; n];
        pre_max[0] = nums[0];
        for (i, &x) in nums.iter().enumerate().skip(1) {
            pre_max[i] = pre_max[i - 1].max(x);
        }

        let mut suf_min = i32::MAX;

        for (i, &x) in nums.iter().enumerate().rev() {
            if pre_max[i] > suf_min {
                pre_max[i] = pre_max[i + 1]
            }
            suf_min = suf_min.min(x);
        }

        pre_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![2, 2, 3], Solution::max_value(vec![2, 1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![3, 3, 3], Solution::max_value(vec![2, 3, 1]));
    }
}
