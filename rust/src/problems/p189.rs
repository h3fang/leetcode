pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len() as i32;
        let k = (k % n) as usize;
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        let expected = vec![5, 6, 7, 1, 2, 3, 4];
        Solution::rotate(&mut nums, k);
        assert_eq!(expected, nums);
    }

    #[test]
    fn case2() {
        let mut nums = vec![-1, -100, 3, 99];
        let k = 2;
        let expected = vec![3, 99, -1, -100];
        Solution::rotate(&mut nums, k);
        assert_eq!(expected, nums);
    }
}
