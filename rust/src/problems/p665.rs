pub struct Solution;

impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let mut greater = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                greater += 1;
                if greater > 1 {
                    return false;
                }
                if i > 0 && nums[i - 1] > nums[i + 1] {
                    nums[i + 1] = nums[i];
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::check_possibility(vec![4, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::check_possibility(vec![4, 2, 1]));
    }
}
