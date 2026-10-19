pub struct Solution;

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut less, mut equal) = (0, 0);
        for x in nums {
            if x < target {
                less += 1;
            } else if x == target {
                equal += 1;
            }
        }
        (less..less + equal).collect()
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
