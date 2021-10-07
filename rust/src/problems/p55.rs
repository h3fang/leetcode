pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len() as i32;
        let mut reach: i32 = 1;
        for (i, a) in nums.iter().enumerate() {
            let i = i as i32;
            if reach <= i {
                return false;
            }
            reach = reach.max(i + 1 + a);
            if reach >= n {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(true, Solution::can_jump(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![3, 2, 1, 0, 4];
        assert_eq!(false, Solution::can_jump(nums));
    }
}
