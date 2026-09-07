pub struct Solution;

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let (mut last_ones, mut prev_max, mut max) = (nums[0].count_ones(), i32::MIN, nums[0]);
        for &x in nums.iter().skip(1) {
            let ones = x.count_ones();
            if ones != last_ones {
                prev_max = max;
                last_ones = ones;
            }
            max = max.max(x);
            if x < prev_max {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_sort_array(vec![8, 4, 2, 30, 15]));
    }

    #[test]
    fn case2() {
        assert!(Solution::can_sort_array(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn case3() {
        assert!(!Solution::can_sort_array(vec![3, 16, 8, 4, 2]));
    }

    #[test]
    fn case4() {
        assert!(!Solution::can_sort_array(vec![2, 28, 9]));
    }
}
