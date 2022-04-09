pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut last = i32::MIN;
        let mut count = 0;
        let mut curr = 0;
        for i in 0..nums.len() {
            if nums[i] == last {
                count += 1;
                if count <= 2 {
                    nums[curr] = nums[i];
                    curr += 1;
                }
            } else {
                last = nums[i];
                count = 1;
                nums[curr] = nums[i];
                curr += 1;
            }
        }
        curr as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(5, k);
        assert_eq!(&[1, 1, 2, 2, 3], &nums[..k as usize]);
    }

    #[test]
    fn case2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(7, k);
        assert_eq!(&[0, 0, 1, 1, 2, 3, 3], &nums[..k as usize]);
    }
}
