pub struct Solution;

impl Solution {
    pub fn smallest_subarrays(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; n];
        let (mut pre, mut suf, mut r, mut bottom) = (0, 0, n as i32 - 1, n as i32 - 1);
        for l in (0..n).rev() {
            suf |= nums[l];
            pre |= nums[l];
            while r >= l as i32 && pre | nums[r as usize] == suf {
                r -= 1;
                if bottom > r {
                    for i in l as i32 + 1..=r {
                        nums[i as usize] |= nums[i as usize - 1];
                    }
                    bottom = l as i32;
                    pre = 0;
                }
            }
            ans[l] = r + 2 - l as i32;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![3, 3, 2, 2, 1],
            Solution::smallest_subarrays(vec![1, 0, 2, 1, 3])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![2, 1], Solution::smallest_subarrays(vec![1, 2]));
    }
}
