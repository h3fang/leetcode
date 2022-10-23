pub struct Solution;

impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let mut dup = -1;
        let mut missing = 1;
        let n = nums.len();
        for i in 0..n {
            let j = nums[i].unsigned_abs() as usize - 1;
            if nums[j] < 0 {
                dup = nums[i].abs();
            } else {
                nums[j] *= -1;
            }
        }

        for (i, n) in nums.into_iter().enumerate() {
            if n > 0 {
                missing = i as i32 + 1;
            }
        }
        vec![dup, missing]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![2, 3], Solution::find_error_nums(vec![1, 2, 2, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![1, 2], Solution::find_error_nums(vec![1, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![3, 1],
            Solution::find_error_nums(vec![3, 2, 3, 4, 6, 5])
        );
    }
}
