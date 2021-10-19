pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn two_sum(nums: &[i32], target: i32, result: &mut Vec<Vec<i32>>) {
            if target > 0 {
                return;
            }
            let n = nums.len();
            let mut left = 0;
            let mut right = n.saturating_sub(1);

            while left < right {
                match (nums[left] + nums[right]).cmp(&-target) {
                    std::cmp::Ordering::Less => left += 1,
                    std::cmp::Ordering::Greater => right = right.saturating_sub(1),
                    std::cmp::Ordering::Equal => {
                        result.push(vec![target, nums[left], nums[right]]);
                        left += 1;
                        right = right.saturating_sub(1);
                        while left < right && nums[left - 1] == nums[left] {
                            left += 1;
                        }
                    }
                }
            }
        }

        let mut result = Vec::new();
        nums.sort_unstable();
        for i in 0..nums.len() {
            if i == 0 || nums[i - 1] != nums[i] {
                two_sum(&nums[i + 1..], nums[i], &mut result);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(expected, Solution::three_sum(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(expected, Solution::three_sum(nums));
    }

    #[test]
    fn case3() {
        let nums = vec![-2, 0, 0, 2, 2];
        let expected: Vec<Vec<i32>> = vec![vec![-2, 0, 2]];
        assert_eq!(expected, Solution::three_sum(nums));
    }
}
