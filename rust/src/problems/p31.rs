pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut i = n - 1;
        while i > 0 {
            if nums[i - 1] < nums[i] {
                break;
            }
            i -= 1;
        }
        if i == 0 {
            nums.reverse();
        } else {
            let mut j = i;
            while j < n {
                if nums[j] <= nums[i - 1] {
                    break;
                }
                j += 1;
            }
            nums.swap(i - 1, j - 1);
            nums[i..].reverse();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(vec![1, 3, 2], nums);
    }

    #[test]
    fn case2() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(vec![1, 2, 3], nums);
    }

    #[test]
    fn case3() {
        let mut nums = vec![1, 1, 15];
        Solution::next_permutation(&mut nums);
        assert_eq!(vec![1, 15, 1], nums);
    }
}
