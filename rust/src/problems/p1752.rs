pub struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut i = 0;
        for (j, w) in nums.windows(2).enumerate() {
            if w[0] > w[1] {
                i = j + 1;
                break;
            }
        }
        if i == 0 {
            return true;
        }
        nums[nums.len() - 1] <= nums[0] && nums[i + 1..].windows(2).all(|w| w[0] <= w[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check(vec![3, 4, 5, 1, 2]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check(vec![2, 1, 3, 4]));
    }

    #[test]
    fn case3() {
        assert!(Solution::check(vec![1, 2, 3]));
    }
}
