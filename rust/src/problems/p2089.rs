pub struct Solution;

impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort_unstable();
        let l = nums.partition_point(|x| *x < target);
        let r = nums.partition_point(|x| *x <= target);
        if l == nums.len() {
            vec![]
        } else {
            (l as i32..r as i32).collect()
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
