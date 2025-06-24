pub struct Solution;

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut ans = Vec::with_capacity(n);
        let mut last = i32::MIN / 2;
        for (i, &x) in nums[..k as usize].iter().enumerate().rev() {
            if x == key {
                last = i as i32;
                break;
            }
        }
        for i in 0..n {
            if i + (k as usize) < n && nums[i + k as usize] == key {
                last = i as i32 + k;
            }
            if last >= i as i32 - k {
                ans.push(i as i32);
            }
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
            vec![1, 2, 3, 4, 5, 6],
            Solution::find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![0, 1, 2, 3, 4],
            Solution::find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2)
        );
    }
}
