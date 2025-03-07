pub struct Solution;

impl Solution {
    pub fn count_pairs(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut result = 0;
        let (mut i, mut j) = (0, nums.len() - 1);
        while i < j {
            while j > i && nums[i] + nums[j] >= target {
                j -= 1;
            }
            result += (j - i) as i32;
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::count_pairs(vec![-1, 1, 2, 3, 1], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::count_pairs(vec![-6, 2, 5, -2, -7, -1, 3], -2));
    }
}
