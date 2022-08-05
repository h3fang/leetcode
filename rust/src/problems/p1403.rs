pub struct Solution;

impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable_by(|a, b| b.cmp(a));
        let mut rem = nums.iter().sum::<i32>();
        let mut sum = 0;
        for (i, n) in nums.iter().enumerate() {
            sum += n;
            rem -= n;
            if sum > rem {
                return nums[..=i].to_vec();
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![10, 9], Solution::min_subsequence(vec![4, 3, 10, 9, 8]));
    }
}
