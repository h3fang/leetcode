pub struct Solution;

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut j = 0;
        for i in 0..n {
            if i + 1 < n && nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
            if nums[i] != 0 {
                nums.swap(i, j);
                j += 1;
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![1, 4, 2, 0, 0, 0],
            Solution::apply_operations(vec![1, 2, 2, 1, 1, 0])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![1, 0], Solution::apply_operations(vec![0, 1]));
    }
}
