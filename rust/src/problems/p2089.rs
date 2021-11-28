pub struct Solution;

impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort_unstable();
        let mut left = -1;
        let mut right = -1;
        for (i, &n) in nums.iter().enumerate() {
            if n == target {
                if left == -1 {
                    left = i as i32;
                }
                right = i as i32;
            }
        }
        if left == -1 {
            vec![]
        } else {
            (left..=right).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 2;
        assert_eq!(vec![1, 2], Solution::target_indices(nums, target));
    }

    #[test]
    fn case2() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 3;
        assert_eq!(vec![3], Solution::target_indices(nums, target));
    }

    #[test]
    fn case3() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 4;
        assert_eq!(vec![0; 0], Solution::target_indices(nums, target));
    }
}
