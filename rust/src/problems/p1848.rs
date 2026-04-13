pub struct Solution;

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let start = start as usize;
        let mut ans = usize::MAX / 2;

        if let Some(i) = nums[..=start].iter().rposition(|&x| x == target) {
            ans = ans.min(start - i);
        }

        let r = (start + 1 + ans).min(nums.len());
        if let Some(i) = nums[start + 1..r].iter().position(|&x| x == target) {
            ans = ans.min(i + 1);
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::get_min_distance(vec![1, 2, 3, 4, 5], 5, 3));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::get_min_distance(vec![1], 1, 0));
    }

    #[test]
    fn case3() {
        assert_eq!(
            0,
            Solution::get_min_distance(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 0)
        );
    }
}
