pub struct Solution;

impl Solution {
    pub fn find_repeat_number(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        for i in 0..nums.len() {
            let mut a = nums[i];
            if a < 0 {
                a += n;
            }
            if nums[a as usize] < 0 {
                return a;
            }
            nums[a as usize] -= n;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![2, 3, 1, 0, 2, 5, 3];
        assert!([2, 3].contains(&Solution::find_repeat_number(nums)));
    }
}
