pub struct Solution;

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut ans, mut i, mut prev) = (0, 1, nums[0]);
        while i < n {
            let mut j = i + 1;
            while j < n && nums[j] == nums[i] {
                j += 1;
            }
            if j == n {
                break;
            }
            if (prev < nums[i] && nums[i] > nums[j]) || (prev > nums[i] && nums[i] < nums[j]) {
                ans += 1;
            }
            prev = nums[i];
            i = j;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::count_hill_valley(vec![2, 4, 1, 1, 6, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_hill_valley(vec![6, 6, 5, 5, 4, 1]));
    }
}
