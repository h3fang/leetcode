pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        let mut prev = i32::MIN;
        for j in 0..nums.len() {
            if nums[j] != prev {
                prev = nums[j];
                nums[i] = prev;
                i += 1;
            }
        }
        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut nums = vec![1, 1, 2];
        let n = Solution::remove_duplicates(&mut nums);
        assert_eq!(2, n);
        assert_eq!(vec![1, 2], &nums[..n as usize]);
    }
}
