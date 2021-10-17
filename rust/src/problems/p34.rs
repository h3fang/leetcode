use std::vec;

pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0i32;
        let mut right = nums.len() as i32 - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if nums.is_empty() || nums[left as usize] != target {
            return vec![-1, -1];
        }
        let r_l = left;

        right = nums.len() as i32 - 1;
        while left < right {
            let mid = left + (right - left) / 2 + 1;
            if nums[mid as usize] > target {
                right = mid - 1;
            } else {
                left = mid;
            }
        }

        vec![r_l, right]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![-1, -1],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(vec![-1, -1], Solution::search_range(vec![], 6));
    }

    #[test]
    fn case4() {
        assert_eq!(vec![0, 0], Solution::search_range(vec![6], 6));
    }

    #[test]
    fn case5() {
        assert_eq!(
            vec![10, 13],
            Solution::search_range(
                vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 6, 6, 6, 8, 10, 10],
                4
            )
        );
    }
}
