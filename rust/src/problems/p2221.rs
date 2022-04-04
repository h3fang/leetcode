pub struct Solution;

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        let mut next = vec![0; nums.len()];
        let mut right = nums.len() - 1;
        while right > 0 {
            for (i, w) in nums[..=right].windows(2).enumerate() {
                next[i] = (w[0] + w[1]) % 10;
            }
            std::mem::swap(&mut nums, &mut next);
            right -= 1;
        }
        nums[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(8, Solution::triangular_sum(vec![1, 2, 3, 4, 5]));
    }
}
