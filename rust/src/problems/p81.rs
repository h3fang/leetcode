pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        fn helper(nums: &[i32], target: i32) -> bool {
            if nums.is_empty() {
                return false;
            }
            let mut left = 0;
            let mut right = nums.len() - 1;
            while left <= right {
                let mid = (left + right) / 2;
                if nums[mid] == target {
                    return true;
                }
                match nums[0].cmp(&nums[nums.len() - 1]) {
                    std::cmp::Ordering::Less => match nums[mid].cmp(&target) {
                        std::cmp::Ordering::Less => left = mid + 1,
                        _ => {
                            if mid == 0 {
                                return false;
                            }
                            right = mid - 1;
                        }
                    },
                    _ => {
                        if helper(&nums[..mid], target) {
                            return true;
                        } else {
                            return helper(&nums[mid + 1..], target);
                        }
                    }
                }
            }
            false
        }
        helper(&nums, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    }

    #[test]
    fn case2() {
        assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }

    #[test]
    fn case3() {
        assert!(!Solution::search(vec![1, 3], 0));
    }
}
