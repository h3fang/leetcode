pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        fn binary(nums: &[i32], target: i32, mut left: i32, mut right: i32) -> i32 {
            while left <= right {
                let mid = left + (right - left) / 2;
                match nums[mid as usize].cmp(&target) {
                    std::cmp::Ordering::Less => {
                        if nums[left as usize] < nums[right as usize] {
                            left = mid + 1;
                        } else {
                            let r1 = binary(nums, target, left, mid - 1);
                            if r1 != -1 {
                                return r1;
                            }
                            return binary(nums, target, mid + 1, right);
                        }
                    }
                    std::cmp::Ordering::Equal => return mid,
                    std::cmp::Ordering::Greater => {
                        if nums[left as usize] < nums[right as usize] {
                            right = mid - 1;
                        } else {
                            let r1 = binary(nums, target, left, mid - 1);
                            if r1 != -1 {
                                return r1;
                            }
                            return binary(nums, target, mid + 1, right);
                        }
                    }
                }
            }
            -1
        }

        binary(&nums, target, 0, nums.len() as i32 - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::search(vec![1], 0));
    }
}
