pub struct Solution;

fn check(mut nums: Vec<i32>, mut k: i32, target: i32) -> bool {
    let n = nums.len();
    for i in 0..nums.len() - 1 {
        if nums[i] != target {
            if k == 0 {
                return false;
            }
            nums[i + 1] *= -1;
            k -= 1;
        }
    }
    nums[n - 1] == target
}

impl Solution {
    pub fn can_make_equal(nums: Vec<i32>, k: i32) -> bool {
        check(nums.clone(), k, 1) || check(nums, k, -1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_make_equal(vec![1, -1, 1, -1, 1], 3));
    }

    #[test]
    fn case2() {
        assert!(!Solution::can_make_equal(vec![-1, -1, -1, 1, 1, 1], 5));
    }

    #[test]
    fn case3() {
        assert!(Solution::can_make_equal(vec![1, -1, 1], 2));
    }
}
