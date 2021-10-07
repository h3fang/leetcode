pub struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut r = nums.clone();
        let mut a = 0;
        let mut b = nums.len() - 1;
        let mut i = b;
        while a < b {
            if nums[a].abs() < nums[b].abs() {
                r[i] = nums[b] * nums[b];
                b -= 1;
            } else {
                r[i] = nums[a] * nums[a];
                a += 1;
            }
            i -= 1;
        }
        r[0] = nums[b] * nums[b];

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![-4, -1, 0, 3, 10];
        let expected = vec![0, 1, 9, 16, 100];
        assert_eq!(expected, Solution::sorted_squares(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![-7, -3, 2, 3, 11];
        let expected = vec![4, 9, 9, 49, 121];
        assert_eq!(expected, Solution::sorted_squares(nums));
    }
}
